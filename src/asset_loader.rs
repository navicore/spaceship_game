use bevy::prelude::*;

#[derive(Resource, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct SceneAssets {
    pub asteroid: Handle<Scene>,
    pub spaceship: Handle<Scene>,
    pub missiles: Handle<Scene>,
    pub material: Handle<StandardMaterial>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneAssets>()
            .add_systems(Startup, load_assets);
    }
}

fn load_assets(
    mut scene_assets: ResMut<SceneAssets>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let material_handle = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 1.0, 1.0),
        ..Default::default()
    });

    *scene_assets = SceneAssets {
        //asteroid: asset_server.load("Asteroid.glb#Scene0"),
        asteroid: asset_server.load("Planet-18Uxrb2dIc.glb#Scene0"),
        //spaceship: asset_server.load("Spaceship.glb#Scene0"),
        spaceship: asset_server.load("Spaceship-Jqfed124pQ.glb#Scene0"),
        //spaceship: asset_server.load("Spaceship-u105mYHLHU.glb#Scene0"),
        //spaceship: asset_server.load("Spaceship-VSxUAFhzbA.glb#Scene0"),
        missiles: asset_server.load("Missiles.glb#Scene0"),
        material: material_handle,
    };
}
