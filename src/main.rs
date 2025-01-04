mod asset_loader;
mod asteriods;
mod camera;
mod collision_detection;
mod debug;
mod despawn;
mod health;
mod lighting;
mod movement;
mod schedule;
mod spaceship;
mod state;

use asset_loader::AssetLoaderPlugin;
use asteriods::AsteroidPlugin;
use bevy::prelude::*;
#[cfg(debug_assertions)]
use bevy::remote::{http::RemoteHttpPlugin, RemotePlugin};

use bevy_embedded_assets::{EmbeddedAssetPlugin, PluginMode};
use camera::CameraPlugin;
use collision_detection::CollisionDetectionPlugin;
use lighting::LightingPlugin;
//use debug::DebugPlugin;
use despawn::DespawnPlugin;
use movement::MovementPlugin;
use schedule::SchedulePlugin;
use spaceship::SpaceshipPlugin;
use state::StatePlugin;

fn main() {
    let mut app = App::new();

    app.insert_resource(ClearColor(Color::srgb(0.1, 0.0, 0.15)))
        .add_plugins((
            EmbeddedAssetPlugin {
                mode: PluginMode::ReplaceDefault,
            },
            DefaultPlugins,
            AssetLoaderPlugin,
            LightingPlugin,
            SpaceshipPlugin,
            AsteroidPlugin,
            MovementPlugin,
            CameraPlugin,
            CollisionDetectionPlugin,
            DespawnPlugin,
            SchedulePlugin,
            StatePlugin,
        ));

    #[cfg(debug_assertions)]
    {
        app.add_plugins((RemotePlugin::default(), RemoteHttpPlugin::default()));
    }

    app.run();
}
