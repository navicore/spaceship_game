use bevy::prelude::*;

pub struct LightingPlugin;

impl Plugin for LightingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_lighting);
    }
}

fn setup_lighting(mut commands: Commands) {
    // Add a directional light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::WHITE,
            illuminance: 10000.0,
            ..Default::default()
        },
        transform: Transform {
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4),
            ..Default::default()
        },
        ..Default::default()
    });

    // Optionally, add an ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.75,
    });
}
