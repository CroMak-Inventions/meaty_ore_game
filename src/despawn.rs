use bevy::prelude::*;

use crate::{
    asteroids::{Asteroid, AsteroidSpawnChildrenEvent}, health::Health, movement::{Acceleration, Rotation, Velocity}, schedule::InGameSet, sound::effects::GameSoundEffects, spaceship::{
        Spaceship,
        shield::Shield,
    }, state::GameState
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
    mut spawn_children_event_writer: MessageWriter<AsteroidSpawnChildrenEvent>,
    query: Query<(Entity, &Health, &Name), Without<Shield>>,
    asteroid_query: Query<(&Asteroid, &Transform, &Velocity, &Rotation, &Acceleration)>
) {
    for (
        entity,
        health,
        name
    ) in query.iter() {
        if health.value <= 0.0 {
            if name.contains("asteroid") {
                
                for (
                    asteroid,
                    xform,
                    velocity,
                    rotation,
                    acceleration
                ) in asteroid_query.get(entity).iter() {
                    spawn_children_event_writer.write(
                        AsteroidSpawnChildrenEvent::new(
                            xform,
                            velocity,
                            rotation,
                            acceleration,
                            asteroid.level
                        )
                    );
                }

            }

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
