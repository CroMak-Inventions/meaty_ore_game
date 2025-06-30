use bevy::prelude::*;

use crate::asset_loader::SceneAssets;

#[derive(Component, Debug)]
pub struct GameAmbientSound;

#[derive(Component, Debug)]
pub struct ThrusterSound;

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

    commands.spawn((
        AudioPlayer::new(scene_assets.thruster_sound.clone()),
        PlaybackSettings::LOOP,
        ThrusterSound,
    ));
}
