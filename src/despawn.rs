use bevy::prelude::*;

use crate::{
    asteroids::Asteroid,
    health::Health,
    schedule::InGameSet,
    sound_fx::GameSoundEffects,
    spaceship::{Spaceship, Shield},
    state::GameState
};

const DESPAWN_DISTANCE: f32 = 100.0;

pub struct DespawnPlugin;

impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                despawn_far_away_entities::<Asteroid>,
                despawn_far_away_entities::<Spaceship>,
                despawn_dead_entities,
                despawn_old_audiosink_entities,
            ).in_set(InGameSet::DespawnEntities),
        )
        .add_systems(
            OnEnter(GameState::GameOver),
            despawn_all_entities::<Health>,
        );
    }
}

fn despawn_far_away_entities<T: Component>(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<T>>
) {
    for (entity, transform) in query.iter() {
        let distance = transform.translation.distance(Vec3::ZERO);

        if distance > DESPAWN_DISTANCE {
            commands.entity(entity).despawn();
        }
    }
}

fn despawn_dead_entities(
    mut commands: Commands,
    query: Query<(Entity, &Health), Without<Shield>>,
) {
    for (entity, health) in query.iter() {
        if health.value <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn despawn_all_entities<T: Component>(
    mut commands: Commands,
    query: Query<Entity, With<T>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn despawn_old_audiosink_entities(
    mut commands: Commands,
    query: Query<(Entity, &AudioSink), With<GameSoundEffects>>,
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
    for (entity, audio_sink) in query.iter() {
        if audio_sink.empty() {
            commands.entity(entity).despawn();
        }
    }
}
