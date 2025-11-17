use std::ops::Range;

use bevy::prelude::*;
use rand::Rng;

use crate::{
    app_globals::AppGlobals,
    asset_loader::SceneAssets,
    collision_detection::{Collider, CollisionDamage},
    health::Health,
    movement::{
        Acceleration,
        Velocity,
        Rotation,
        MovingObjectBundle,
        SceneBundle
    },
    schedule::InGameSet,
    spaceship::{Spaceship, SPACESHIP_RADIUS},
};


const VELOCITY_SCALAR: f32 = 5.0;
const ACCELERATION_SCALAR: f32 = 1.0;
const SPAWN_RANGE_X: Range<f32> =  -25.0..25.0;
const SPAWN_RANGE_Z: Range<f32> = -25.0..25.0;
const SPAWN_TIME_SECONDS: f32 = 4.0;
const MAX_ROTATE_SPEED: f32 = 3.0;
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

#[derive(Component, Debug)]
pub struct Explosion {
    pub duration: i32,  // in frames
}

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
                spawn_asteroids,
                rotate_passive_objects::<Asteroid>,
                rotate_passive_objects::<AsteroidDebris>,
                spawn_collision_animation,
                update_explosion_animation,
            ).in_set(InGameSet::EntityUpdates),
        )
        .add_event::<AsteroidCollisionAnimationEvent>();
    }
}

fn spawn_asteroids(
    mut commands: Commands,
    spaceship_xform: Single<&Transform, With<Spaceship>>,
    asteroids: Query<Entity, With<Asteroid>>,
    scene_assets: Res<SceneAssets>,
    mut spawn_timer: ResMut<AsteroidSpawnTimer>,
    mut app_globals: ResMut<AppGlobals>,
    time: Res<Time>,
) {
    // We are setting up a game dynamic where a wave of asteroids, up to
    // about 10 or so, is spawned all at once.  Enough that it is challanging,
    // but not impossible.
    // When the player is done shooting the last asteroid, we will spawn
    // another wave.
    spawn_timer.timer.tick(time.delta());
    if !spawn_timer.timer.just_finished() {
        return;
    }

    if asteroids.iter().len() == 0 {
        // All asteroids have been cleared.  New level.
        app_globals.level += 1;
        println!("New level: {:}", app_globals.level);

        if app_globals.level % 4 == 0 {
            println!("\tBoss Level Time!!");
        }

        // Spawn a new wave of asteroids.
        for _i in 0..10 {
            // spawn an asteroid
            spawn_asteroid(&mut commands, &spaceship_xform, &scene_assets.asteroid);
        }
    }
}

fn spawn_asteroid(
    commands: &mut Commands,
    spaceship_xform: &Transform,
    asteroid: &Handle<Scene>,
) {
    let mut rng = rand::rng();

    let mut translation = Vec3::new(
        rng.random_range(SPAWN_RANGE_X),
        0.0,
        rng.random_range(SPAWN_RANGE_Z),
    );

    for _i in 0..2 {
        // It is a bit unfair to have an asteroid spawn right on top of the
        // spaceship.  So we allow a (finite) number of chances to choose
        // a different location if this happens.
        // There is still a tiny chance of this happening, but it will be
        // considerably less annoying.  Without this, it was happening
        // at least once per game.
        let distance = translation.distance(spaceship_xform.translation);

        if distance >= SPACESHIP_RADIUS * 4.0 {
            break;
        }
        else {
            translation = Vec3::new(
                rng.random_range(SPAWN_RANGE_X),
                0.0,
                rng.random_range(SPAWN_RANGE_Z),
            );
        }
    }

    let rotation = Rotation::random(
        -MAX_ROTATE_SPEED,
        MAX_ROTATE_SPEED
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
            rotation: rotation,
            collider: Collider::new(RADIUS),
            model: SceneBundle {
                scene: SceneRoot(asteroid.clone()),
                transform: Transform::from_translation(translation),
            },
        },
        Asteroid,
        Health::new(HEALTH),
        CollisionDamage::new(COLLISION_DAMAGE),
    ));
}

fn rotate_passive_objects<T: Component>(
    mut query: Query<(&mut Transform, &Rotation), With<T>>,
    time: Res<Time>,
) {
    for (mut transform, rotation) in query.iter_mut() {
        transform.rotate_local_x(rotation.x * time.delta_secs());
        transform.rotate_local_z(rotation.z * time.delta_secs());
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

        let rotation = Rotation::random(
            -MAX_ROTATE_SPEED / 2.0,
            MAX_ROTATE_SPEED / 2.0,
        );


        commands.spawn((
            MovingObjectBundle {
                velocity: debris_velocity.clone(),
                acceleration: Acceleration::from(acceleration.clone()),
                rotation: rotation.clone(),
                collider: Collider::new(RADIUS),
                model: SceneBundle {
                    scene: SceneRoot(scene_assets.asteroid.clone()),
                    transform: debris_xform,
                },
            },
            AsteroidDebris,
        ));

        commands.spawn((
            MovingObjectBundle {
                velocity: velocity.clone(),
                acceleration: Acceleration::from(acceleration.clone()),
                rotation: rotation.clone(),
                collider: Collider::new(RADIUS),
                model: SceneBundle {
                    scene: SceneRoot(scene_assets.explosion.clone()),
                    transform: debris_xform,
                },
            },
            Health::new(HEALTH),
            Explosion {duration: 0},
        ));

    }
}

fn update_explosion_animation(
    mut query: Query<(&mut Explosion, &mut Health, &mut Transform), With<Explosion>>,
    time: Res<Time>,
) {
    for (mut explosion, mut health, mut xform) in query.iter_mut() {
        explosion.duration += 1;
        health.value -= 80.0 * time.delta_secs();
        xform.scale *= 1.0 + (6.0 * time.delta_secs());
    }
}
