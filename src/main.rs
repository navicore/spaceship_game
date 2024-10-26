use bevy::prelude::*;

#[derive(Component, Debug)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component, Debug)]
struct Velocity {
    x: f32,
    y: f32,
}

fn main() {
    App::new()
        .add_systems(Startup, spawn_spaceship)
        .add_systems(Update, (update_positions, print_position))
        .add_plugins(DefaultPlugins)
        .run()
}

fn spawn_spaceship(mut commands: Commands) {
    commands.spawn((Position { x: 0.0, y: 0.0 }, Velocity { x: 1.0, y: 1.0 }));
}

fn update_positions(mut query: Query<(&Velocity, &mut Position)>) {
    for (velocity, mut position) in query.iter_mut() {
        position.x += velocity.x;
        position.y += velocity.y;
    }
}

fn print_position(query: Query<(Entity, &Position)>) {
    // log the entity ID and position
    for (entity, position) in query.iter() {
        info!("Entity {:?} is at position {:?}", entity, position);
    }
}
