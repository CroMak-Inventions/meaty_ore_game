use bevy::{audio, prelude::*};

use crate::{
    asset_loader::SceneAssets,
    schedule::InGameSet,
};

const SOUND_EFFECTS_VOLUME: audio::Volume = audio::Volume::Linear(0.8);


#[derive(Component, Debug)]
pub struct GameSoundEffects {
    volume_is_set: bool,
    volume: audio::Volume,
}


// in the future, we will have multiple sounds.  It might make sense
// for the type of sound to be an enum and stored in the event.
#[derive(Event, Debug)]
pub struct ShootingSoundEvent;

#[derive(Event, Debug)]
pub struct AsteroidCollisionSoundEvent;


pub struct SoundFXPlugin;

impl Plugin for SoundFXPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(
            Update,
            (
                play_shooting_sound,
                play_meteor_collision_sound,
                set_sound_fx_volume,
            ).in_set(InGameSet::EntityUpdates),
        )
        .add_event::<ShootingSoundEvent>()
        .add_event::<AsteroidCollisionSoundEvent>();
    }
}

//
// Well, this kinda sux from a perspective of resource reuse.
// Apparently the Audiosink component can only play the audio it is
// connected to one time.  No rewind, No replay.  Curiously there is an
// audio loop configuration, where the audio can loop.  So I don't really
// understand why the audio can't be replayed, if it can be looped.
//
// So the strategy for sound effects is that a new AudioSink component
// needs to be spawned every time we want something to make a sound.
// .....Well......OK.....
//
fn play_shooting_sound(
    mut sound_event_reader: EventReader<ShootingSoundEvent>,
    mut commands: Commands,
    scene_assets: Res<SceneAssets>,
) {
    for _ in sound_event_reader.read() {
        commands.spawn((
            AudioPlayer::new(scene_assets.shooting_sound.clone()),
            GameSoundEffects {
                volume_is_set: false,
                volume: SOUND_EFFECTS_VOLUME,
            },
        ));
    }
}

fn play_meteor_collision_sound(
    mut sound_event_reader: EventReader<AsteroidCollisionSoundEvent>,
    mut commands: Commands,
    scene_assets: Res<SceneAssets>,
) {
    for _ in sound_event_reader.read() {
        commands.spawn((
            AudioPlayer::new(scene_assets.meteor_hit_sound.clone()),
            GameSoundEffects {
                volume_is_set: false,
                volume: SOUND_EFFECTS_VOLUME,
            },
        ));
    }
}

fn set_sound_fx_volume(
    mut query: Query<(&mut AudioSink, &mut GameSoundEffects)>,
) {
    // It would be very convenient if we could set the volume at the time of
    // spawning our sound effects, but it doesn't seem possible, at least for
    // our current version of Bevy.  So we immediately set the volume the
    // first time it appears in the query.
    for (mut audio_sink, mut game_sound_fx) in query.iter_mut() {
        if !game_sound_fx.volume_is_set {
            audio_sink.set_volume(game_sound_fx.volume);
            game_sound_fx.volume_is_set = true;
        }
    }
}
