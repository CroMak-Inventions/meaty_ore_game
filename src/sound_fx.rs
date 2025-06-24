use bevy::{audio::{self, PlaybackMode}, prelude::*};

use crate::{asset_loader::SceneAssets, schedule::InGameSet};


#[derive(Component, Debug)]
pub struct GameSoundEffects;


// in the future, we will have multiple sounds.  It might make sense
// for the type of sound to be an enum and stored in the event.
#[derive(Event, Debug)]
pub struct ShootingSoundEvent {
    pub entity: Entity,
}

impl ShootingSoundEvent {
    pub fn new(entity: Entity) -> Self {
        Self {entity}
    }
}

pub struct SoundFXPlugin;

impl Plugin for SoundFXPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(
            Update,
            play_shooting_sound.in_set(InGameSet::EntityUpdates),
        )
        .add_event::<ShootingSoundEvent>();
    }
}

fn play_shooting_sound(
    mut sound_event_reader: EventReader<ShootingSoundEvent>,
    mut commands: Commands,
    scene_assets: Res<SceneAssets>,
) {
    // Well, this kinda sux from a perspective of resource reuse.
    // Apparently the Audiosink component can only play the audio it is
    // connected to one time.  No rewind, No replay.  Curiously there is an
    // audio loop configuration, where the audio can loop.  So I don't really
    // understand why the audio can't be replayed, if it can be looped.
    //
    // So the strategy for sound effects is that a new AudioSink component
    // needs to be spawned every time we want something to make a sound.
    // .....Well......OK.....
    for _ in sound_event_reader.read() {
        println!("Play a Shooting sound");

        // no need to refer to any particular entity, just play the sound.
        commands.spawn((
            AudioPlayer::new(scene_assets.shooting_sound.clone()),
            GameSoundEffects,
        ));
    }
}
