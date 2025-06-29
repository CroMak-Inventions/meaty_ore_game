use std::ops::Range;

use bevy::prelude::*;
use rand::Rng;

use crate::{
    asset_loader::SceneAssets,
    collision_detection::{Collider, CollisionDamage},
    health::Health,
    movement::{Acceleration, Velocity, MovingObjectBundle, SceneBundle},
    schedule::InGameSet,
};


const VELOCITY_SCALAR: f32 = 5.0;
const ACCELERATION_SCALAR: f32 = 1.0;
const SPAWN_RANGE_X: Range<f32> =  -25.0..25.0;
const SPAWN_RANGE_Z: Range<f32> = 0.0..25.0;
const SPAWN_TIME_SECONDS: f32 = 4.0;
const ROTATE_SPEED: f32 = 2.5;
const RADIUS: f32 = 1.5;
const HEALTH: f32 = 20.0;
const COLLISION_DAMAGE: f32 = 35.0;

#[derive(Component, Debug)]
pub struct Asteroid;

#[derive(Resource, Debug)]
pub struct AsteroidSpawnTimer {
    timer: Timer,
}

#[derive(Component, Debug)]
pub struct AsteroidDebris;

#[derive(Event, Debug)]
pub struct AsteroidCollisionAnimationEvent {
    pub xform: Transform,
    pub velocity: Velocity,
    pub acceleration: Acceleration,
}

impl AsteroidCollisionAnimationEvent {
    pub fn new(xform: &Transform, velocity: &Velocity, acceleration: &Acceleration) -> Self {
        Self {
            xform: xform.clone(),
            velocity: velocity.clone(),
            acceleration: acceleration.clone(),
        }
    }
}


pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AsteroidSpawnTimer {
            timer: Timer::from_seconds(
                SPAWN_TIME_SECONDS,
                TimerMode::Repeating
            )
        })
        .add_systems(Update, (
                spawn_asteroid,
                rotate_asteroids,
                spawn_collision_animation,
            ).in_set(InGameSet::EntityUpdates),
        )
        .add_event::<AsteroidCollisionAnimationEvent>();
    }
}

fn spawn_asteroid(
    mut commands: Commands,
    mut spawn_timer: ResMut<AsteroidSpawnTimer>,
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
        Health::new(HEALTH),
        CollisionDamage::new(COLLISION_DAMAGE),
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

fn spawn_collision_animation(
    mut animation_event_reader: EventReader<AsteroidCollisionAnimationEvent>,
    mut commands: Commands,
    scene_assets: Res<SceneAssets>,
) {
    for &AsteroidCollisionAnimationEvent {
        xform,
        ref velocity,
        ref acceleration,
    } in animation_event_reader.read() {
        let mut rng = rand::rng();

        let mut debris_velocity = Velocity::from(velocity.clone());
        debris_velocity.value *= rng.random_range(0.5..1.0);

        let mut debris_xform = Transform::from_translation(xform.translation);
        debris_xform.scale *= rng.random_range(0.1..0.4);

        commands.spawn((
            debris_velocity,
            Acceleration::from(acceleration.clone()),
            SceneBundle {
                scene: SceneRoot(scene_assets.asteroid.clone()),
                transform: debris_xform,
            },
            AsteroidDebris,
        ));
    }
}
