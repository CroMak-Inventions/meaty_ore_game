use bevy::prelude::*;
use bevy::audio::{PlaybackMode, Volume};

use crate::asset_loader::SceneAssets;


#[derive(Component, Debug)]
pub struct GameAmbientSound;

#[derive(Component, Debug)]
pub struct ThrusterSound;

#[derive(Component, Debug)]
pub struct SaucerSound;

pub struct AmbientSoundPlugin;

impl Plugin for AmbientSoundPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, spawn_ambient_sound);
    }
}


fn spawn_ambient_sound(
    mut commands: Commands,
    scene_assets: Res<SceneAssets>,
) {
    commands.spawn((
        AudioPlayer::new(scene_assets.background_music.clone()),
        PlaybackSettings {
            mode: PlaybackMode::Loop,
            volume: Volume::Linear(1.0),
            ..default()
        },
        GameAmbientSound,
    ));

    commands.spawn((
        AudioPlayer::new(scene_assets.thruster_sound.clone()),
        PlaybackSettings {
            mode: PlaybackMode::Loop,
            volume: Volume::Linear(1.0),
            ..default()
        },
        ThrusterSound,
    ));

    commands.spawn((
        AudioPlayer::new(scene_assets.saucer_sound.clone()),
        PlaybackSettings {
            mode: PlaybackMode::Loop,
            volume: Volume::Linear(1.0),
            ..default()
        },
        SaucerSound,
    ));
}
