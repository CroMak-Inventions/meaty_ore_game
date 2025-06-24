use bevy::prelude::*;

use crate::{asset_loader::SceneAssets, schedule::InGameSet};

#[derive(Component, Debug)]
pub struct GameAmbientSound;

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
        PlaybackSettings::LOOP,
        GameAmbientSound,
    ));
}