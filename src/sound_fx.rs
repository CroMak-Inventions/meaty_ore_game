use bevy::{audio, prelude::*};

use crate::{
    asset_loader::SceneAssets,
    schedule::InGameSet,
    spaceship::ShieldReadyEvent,
};

const SOUND_EFFECTS_VOLUME: audio::Volume = audio::Volume::Linear(0.8);
const SHIELD_READY_VOLUME: audio::Volume = audio::Volume::Linear(0.75);


#[derive(Component, Debug)]
pub struct GameSoundEffects {
    volume_is_set: bool,
    volume: audio::Volume,
}


// in the future, we will have multiple sounds.  It might make sense
// for the type of sound to be an enum and stored in the event.
#[derive(Message, Debug)]
pub struct ShootingSoundEvent;

#[derive(Message, Debug)]
pub struct SaucerShootingSoundEvent;

#[derive(Message, Debug)]
pub struct AsteroidCollisionSoundEvent;


pub struct SoundFXPlugin;

impl Plugin for SoundFXPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(
            Update,
            (
                play_shooting_sound,
                play_saucer_shooting_sound,
                play_meteor_collision_sound,
                play_shield_ready_sound,
                set_sound_fx_volume,
            ).in_set(InGameSet::EntityUpdates),
        )
        .add_message::<ShootingSoundEvent>()
        .add_message::<SaucerShootingSoundEvent>()
        .add_message::<ShieldReadyEvent>()
        .add_message::<AsteroidCollisionSoundEvent>();
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
    mut sound_event_reader: MessageReader<ShootingSoundEvent>,
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

fn play_saucer_shooting_sound(
    mut sound_event_reader: MessageReader<SaucerShootingSoundEvent>,
    mut commands: Commands,
    scene_assets: Res<SceneAssets>,
) {
    for _ in sound_event_reader.read() {
        commands.spawn((
            AudioPlayer::new(scene_assets.saucer_shooting_sound.clone()),
            GameSoundEffects {
                volume_is_set: false,
                volume: SOUND_EFFECTS_VOLUME,
            },
        ));
    }
}

fn play_meteor_collision_sound(
    mut sound_event_reader: MessageReader<AsteroidCollisionSoundEvent>,
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

fn play_shield_ready_sound(
    mut reader: MessageReader<ShieldReadyEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let sfx: Handle<AudioSource> = asset_server.load("sound/Shoot-2.ogg"); // try this first

    for _ in reader.read() {
        commands.spawn((
            AudioPlayer::new(sfx.clone()),
            GameSoundEffects {
                volume_is_set: false,
                volume: SHIELD_READY_VOLUME,
            },
        ));
    }
    info!("ShieldReadyEvent sound played");
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
