use bevy::prelude::*;

mod asset_loader;
use asset_loader::AssetLoaderPlugin;

mod spaceship;
use spaceship::SpaceshipPlugin;

mod asteroids;
use asteroids::AsteroidPlugin;

mod movement;
use movement::MovementPlugin;

mod collision_detection;
use collision_detection::CollisionDetectionPlugin;

mod camera;
use camera::CameraPlugin;

mod debug;
use debug::DebugPlugin;


fn main() {
    App::new()
        // System defined plugings
        .insert_resource(ClearColor(Color::linear_rgb(0.01,0.0,0.015)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 750.0,
            affects_lightmapped_meshes: true
        })
        .add_plugins(DefaultPlugins)
        // User defined plugins
        .add_plugins((
            AssetLoaderPlugin,
            CameraPlugin,
            SpaceshipPlugin,
            AsteroidPlugin,
            MovementPlugin,
            CollisionDetectionPlugin,
            DebugPlugin
        ))
        .run();
    
}
