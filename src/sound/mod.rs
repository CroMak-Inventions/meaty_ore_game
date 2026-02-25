use bevy::prelude::*;

pub mod ambient;
use ambient::AmbientSoundPlugin;

pub mod effects;
use effects::SoundFXPlugin;


pub struct SoundPlugin;

impl Plugin for SoundPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins((
            AmbientSoundPlugin,
            SoundFXPlugin,
        ));
    }
}