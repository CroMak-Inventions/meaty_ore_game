use std::ops::Range;

use bevy::prelude::*;
use rand::Rng;

use crate::collision_detection::Collider;
use crate::schedule::InGameSet;

use super::asset_loader::SceneAssets;
use super::movement::{Acceleration, Velocity, MovingObjectBundle, SceneBundle};

const VELOCITY_SCALAR: f32 = 5.0;
const ACCELERATION_SCALAR: f32 = 1.0;
const SPAWN_RANGE_X: Range<f32> =  -25.0..25.0;
const SPAWN_RANGE_Z: Range<f32> = 0.0..25.0;
const SPAWN_TIME_SECONDS: f32 = 4.0;
const ROTATE_SPEED: f32 = 2.5;
const RADIUS: f32 = 1.5;


#[derive(Component, Debug)]
pub struct Asteroid;

#[derive(Resource, Debug)]
pub struct SpawnTimer {
    timer: Timer,
}


pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer {
            timer: Timer::from_seconds(
                SPAWN_TIME_SECONDS,
                TimerMode::Repeating
            )
        })
        .add_systems(Update, (
                spawn_asteroid,
                rotate_asteroids,
            ).in_set(InGameSet::EntityUpdates),
        );
    }
}

fn spawn_asteroid(
    mut commands: Commands,
    mut spawn_timer: ResMut<SpawnTimer>,
    time: Res<Time>,
    scene_assets: Res<SceneAssets>,
) {
    spawn_timer.timer.tick(time.delta());
    if !spawn_timer.timer.just_finished() {
        return;
    }

    let mut rng = rand::rng();

    let translation = Vec3::new(
        rng.random_range(SPAWN_RANGE_X),
        0.0,
        rng.random_range(SPAWN_RANGE_Z),
    );

    let mut random_unit_vector = 
        || Vec3::new(
            rng.random_range(-1.0..1.0),
            0.0,
            rng.random_range(-1.0..1.0)
        ).normalize_or_zero();
    
    let velocity = random_unit_vector() * VELOCITY_SCALAR;
    let acceleration = random_unit_vector() * ACCELERATION_SCALAR;

    commands.spawn((
        MovingObjectBundle {
            velocity: Velocity { value: velocity },
            acceleration: Acceleration { value: acceleration },
            collider: Collider::new(RADIUS),
            model: SceneBundle {
                scene: SceneRoot(scene_assets.asteroid.clone()),
                transform: Transform::from_translation(translation),
            },
        },
        Asteroid,
    ));
}

fn rotate_asteroids(
    mut query: Query<&mut Transform, With<Asteroid>>,
    time: Res<Time>,
) {
    for mut transform in query.iter_mut() {
        transform.rotate_local_z(ROTATE_SPEED * time.delta_secs());
    }
}
