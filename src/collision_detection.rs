use bevy::{prelude::*, utils::HashMap};

use crate::{
    asteriods::Asteroid,
    health::Health,
    schedule::InGameSet,
    spaceship::{Missile, Spaceship},
};

#[derive(Component, Debug)]
pub struct Collider {
    pub radius: f32,
    pub colliding_entities: Vec<Entity>,
}

impl Collider {
    pub const fn new(radius: f32) -> Self {
        Self {
            radius,
            colliding_entities: Vec::new(),
        }
    }
}

#[derive(Component, Debug)]
pub struct CollisionDamage {
    pub amount: f32,
}
impl CollisionDamage {
    pub const fn new(amount: f32) -> Self {
        Self { amount }
    }
}

#[derive(Event, Debug)]
pub struct CollisionEvent {
    pub entity: Entity,
    pub collided_entity: Entity,
}
impl CollisionEvent {
    pub const fn new(entity: Entity, collided_entity: Entity) -> Self {
        Self {
            entity,
            collided_entity,
        }
    }
}

pub struct CollisionDetectionPlugin;

impl Plugin for CollisionDetectionPlugin {
    fn build(&self, app: &mut App) {
        //app.add_systems(Update, collision_detection);
        app.add_systems(
            Update,
            collision_detection.in_set(InGameSet::CollisionDetection),
        )
        .add_systems(
            Update,
            (
                (
                    handle_collisions::<Asteroid>,
                    handle_collisions::<Spaceship>,
                    handle_collisions::<Missile>,
                ),
                apply_collision_damage,
            )
                .chain()
                .in_set(InGameSet::EntityUpdates),
        )
        .add_event::<CollisionEvent>();
    }
}

fn collision_detection(mut query: Query<(Entity, &GlobalTransform, &mut Collider)>) {
    let mut colliding_entities: HashMap<Entity, Vec<Entity>> = HashMap::new();

    for (entity_a, transform_a, collider_a) in query.iter() {
        for (entity_b, transform_b, collider_b) in query.iter() {
            let distance = transform_a
                .translation()
                .distance(transform_b.translation());
            if distance < collider_a.radius + collider_b.radius {
                colliding_entities
                    .entry(entity_a)
                    .or_insert_with(Vec::new)
                    .push(entity_b);
            }
        }
    }

    for (entity, _, mut collider) in &mut query {
        collider.colliding_entities.clear();
        if let Some(collisions) = colliding_entities.get(&entity) {
            collider
                .colliding_entities
                .extend(collisions.iter().copied());
        }
    }
}

fn handle_collisions<T: Component>(
    mut collision_event_writer: EventWriter<CollisionEvent>,
    query: Query<(Entity, &Collider), With<T>>,
) {
    for (entity, collider) in query.iter() {
        for &collided_entity in &collider.colliding_entities {
            if query.get(collided_entity).is_ok() {
                continue;
            }
            // send collision event
            collision_event_writer.send(CollisionEvent::new(entity, collided_entity));
        }
    }
}

pub fn apply_collision_damage(
    mut collitision_event_reader: EventReader<CollisionEvent>,
    mut health_query: Query<&mut Health>,
    collision_damage_query: Query<&CollisionDamage>,
) {
    for &CollisionEvent {
        entity,
        collided_entity,
    } in collitision_event_reader.read()
    {
        let Ok(mut health) = health_query.get_mut(entity) else {
            continue;
        };

        let Ok(collision_damage) = collision_damage_query.get(collided_entity) else {
            continue;
        };

        health.value -= collision_damage.amount;
    }
}
