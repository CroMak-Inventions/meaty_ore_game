use bevy::prelude::*;

mod ambient_sound;
mod asset_loader;
mod asteroids;
mod camera;
mod collision_detection;
mod despawn;
mod game_over;
mod health;
mod lighting;
mod movement;
mod schedule;
mod score_text;
mod sound_fx;
mod spaceship;
mod state;
mod debug;


use ambient_sound::AmbientSoundPlugin;
use asset_loader::AssetLoaderPlugin;
use asteroids::AsteroidPlugin;
use camera::CameraPlugin;
use collision_detection::CollisionDetectionPlugin;
use despawn::DespawnPlugin;
use game_over::GameOverPlugin;
use lighting::LightingPlugin;
use movement::MovementPlugin;
use schedule::SchedulePlugin;
use score_text::ScorePlugin;
use sound_fx::SoundFXPlugin;
use spaceship::SpaceshipPlugin;
use state::StatePlugin;
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
            GameOverPlugin,
            //DebugPlugin,
        ))
        .run();
    
}
