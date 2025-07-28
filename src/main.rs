use bevy::prelude::*;

mod asset_loader;
mod movement;
mod spaceship;
mod asteroids;
mod camera;
mod collision_detection;
mod despawn;
mod schedule;
mod state;
mod health;
mod lighting;
mod sound_fx;
mod ambient_sound;
mod score_text;
mod debug;


use asset_loader::AssetLoaderPlugin;
use movement::MovementPlugin;
use spaceship::SpaceshipPlugin;
use asteroids::AsteroidPlugin;
use camera::CameraPlugin;
use collision_detection::CollisionDetectionPlugin;
use despawn::DespawnPlugin;
use schedule::SchedulePlugin;
use state::StatePlugin;
use lighting::LightingPlugin;
use sound_fx::SoundFXPlugin;
use ambient_sound::AmbientSoundPlugin;
use score_text::ScorePlugin;
use debug::DebugPlugin;

fn main() {
    App::new()
        // System defined plugings
        .insert_resource(ClearColor(Color::linear_rgb(0.0005,0.0,0.005)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 250.0,
            affects_lightmapped_meshes: true
        })
        .add_plugins(DefaultPlugins)
        // User defined plugins
        .add_plugins((
            AssetLoaderPlugin,
            MovementPlugin,
            SpaceshipPlugin,
            AsteroidPlugin,
            CameraPlugin,
            CollisionDetectionPlugin,
            DespawnPlugin,
            SchedulePlugin,
            StatePlugin,
            LightingPlugin,
            SoundFXPlugin,
            AmbientSoundPlugin,
            ScorePlugin,
            //DebugPlugin,
        ))
        .run();
    
}
