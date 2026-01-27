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
mod hud;
mod lighting;
mod movement;
mod saucer;
mod schedule;
mod score_text;
mod sound_fx;
mod spaceship;
mod state;
#[cfg(feature = "debug")]
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
use hud::HudPlugin;
use lighting::LightingPlugin;
use movement::MovementPlugin;
use saucer::SaucerPlugin;
use schedule::SchedulePlugin;
use score_text::ScorePlugin;
use sound_fx::SoundFXPlugin;
use spaceship::SpaceshipPlugin;
use state::StatePlugin;
#[cfg(feature = "debug")]
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
            SaucerPlugin,
            CameraPlugin,
            CollisionDetectionPlugin,
            DespawnPlugin,
            SchedulePlugin,
            LightingPlugin,
            SoundFXPlugin,
            AmbientSoundPlugin,
            ScorePlugin,
        ))
        .add_plugins((
            // max 15 plugins in a tuple, so we split it up.
            StatePlugin,
            GameOverPlugin,
            HudPlugin,
            #[cfg(feature = "debug")]
            DebugPlugin,
        ))
        .run();
    
}
