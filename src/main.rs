use bevy::prelude::*;

mod ambient_sound;
mod app_globals;
mod app_setup;
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
use app_globals::AppGlobalsPlugin;
use app_setup::AppSetupPlugin;
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
        .add_plugins((
            AppSetupPlugin,
            AppGlobalsPlugin,
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
        ))
        .add_plugins((
            // max 15 plugins in a tuple, so we split it up.
            GameOverPlugin,
            //DebugPlugin,
        ))
        .run();
    
}
