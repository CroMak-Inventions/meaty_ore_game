use std::f32::consts::PI;

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

pub mod levels;
use levels::ASTEROID_SIZE_PROPS;

// Normal meteor constant values
const SPAWN_TIME_SECONDS: f32 = 4.0;

#[derive(Component, Debug)]
pub struct Asteroid {
    pub level: usize
}

impl Asteroid {
    pub fn new(level: usize) -> Self {
        Self { level }
    }
}

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

#[derive(Message, Debug)]
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

#[derive(Message, Debug)]
pub struct AsteroidSpawnChildrenEvent {
    pub xform: Transform,
    pub velocity: Velocity,
    pub rotation: Rotation,
    pub acceleration: Acceleration,
    pub level: usize,
}

impl AsteroidSpawnChildrenEvent {
    pub fn new(
        xform: &Transform,
        velocity: &Velocity,
        rotation: &Rotation,
        acceleration: &Acceleration,
        level: usize
    ) -> Self {
        Self {
            xform: xform.clone(),
            velocity: velocity.clone(),
            rotation: rotation.clone(),
            acceleration: acceleration.clone(),
            level
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
                split_asteroid,
            ).in_set(InGameSet::EntityUpdates),
        )
        .add_message::<AsteroidCollisionAnimationEvent>()
        .add_message::<AsteroidSpawnChildrenEvent>();
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
        //#[cfg(debug_assertions)]
        info!("New level: {:}", app_globals.level);
        
        let mut m_sizes: Vec<usize> = Vec::new();

        match app_globals.level - 1 {
            0 | 1 => m_sizes.extend([0, 0, 0, 0, 0, 0, 0, 0, 0, 0].iter()),
            2 => m_sizes.extend([1, 1, 1, 1].iter()),
            3 | 4 => m_sizes.extend([1, 1, 1, 1, 0, 0, 0, 0].iter()),
            5 => m_sizes.extend([2, 2].iter()),
            _ => m_sizes.extend([2, 1, 1, 1, 0, 0, 0, 0, 0, 0].iter()),
        }

        spawn_new_wave(&m_sizes, &mut commands, spaceship_xform, scene_assets);

        app_globals.level += 1;
    }
}


fn spawn_new_wave(
    meteor_sizes: &[usize],
    mut commands: &mut Commands,
    spaceship_xform: Single<&Transform, With<Spaceship>>,
    scene_assets: Res<SceneAssets>,
) {
    for meteor_size in meteor_sizes {
        spawn_random_asteroid(
            &mut commands,
            &spaceship_xform,
            &scene_assets,
            *meteor_size,
        );
    }
}


fn spawn_random_asteroid(
    commands: &mut Commands,
    spaceship_xform: &Transform,
    scenes: &Res<SceneAssets>,
    mut level: usize,
) {
    // Big meteor constant values
    if level > 2 {
        level = 2;
    }

    let spawn_props = &ASTEROID_SIZE_PROPS[level];

    let scene: Handle<Scene>;
    if level >= 2 {
        scene = scenes.asteroid_big.clone();
    }
    else if level >= 1 {
        scene = scenes.asteroid_medium.clone();
    }
    else {
        scene = scenes.asteroid.clone();
    }

    let mut rng = rand::rng();

    let mut translation = Vec3::new(
        rng.random_range(spawn_props.spawn_range_x.clone()),
        0.0,
        rng.random_range(spawn_props.spawn_range_z.clone()),
    );

    for _i in 0..4 {
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
                rng.random_range(spawn_props.spawn_range_x.clone()),
                0.0,
                rng.random_range(spawn_props.spawn_range_z.clone()),
            );
        }
    }

    let rotation = Rotation::random(
        -spawn_props.max_rotate_speed,
        spawn_props.max_rotate_speed
    );

    let mut random_unit_vector = 
        || Vec3::new(
            rng.random_range(-1.0..1.0),
            0.0,
            rng.random_range(-1.0..1.0)
        ).normalize_or_zero();

    let velocity = random_unit_vector() * spawn_props.velocity_scalar;

    // Technically, an asteroid would not have any significant acceleration,
    // But having it gradually speed up over time adds some challenge to the
    // gameplay.
    let acceleration = random_unit_vector() * spawn_props.acceleration_scalar;

    commands.spawn((
        Name::new("asteroid"),
        MovingObjectBundle {
            velocity: Velocity { value: velocity },
            acceleration: Acceleration { value: acceleration },
            rotation: rotation,
            collider: Collider::new(spawn_props.radius),
            model: SceneBundle {
                scene: SceneRoot(scene),
                transform: Transform::from_translation(translation),
            },
        },
        Asteroid::new(spawn_props.level),
        Health::new(spawn_props.health),
        CollisionDamage::new(spawn_props.collision_damage),
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
    mut animation_event_reader: MessageReader<AsteroidCollisionAnimationEvent>,
    mut commands: Commands,
    scene_assets: Res<SceneAssets>,
) {
    let spawn_props = &ASTEROID_SIZE_PROPS[0];

    for &AsteroidCollisionAnimationEvent {
        xform,
        ref velocity,
        ref acceleration,
    } in animation_event_reader.read() {
        let mut rng = rand::rng();

        let mut debris_velocity = Velocity::from(velocity.clone());
        debris_velocity.value *= rng.random_range(0.6..1.0);

        let mut debris_xform = Transform::from_translation(xform.translation);
        debris_xform.scale *= rng.random_range(0.1..0.4);

        let rotation = Rotation::random(
            -spawn_props.max_rotate_speed / 2.0,
            spawn_props.max_rotate_speed / 2.0,
        );

        commands.spawn((
            Name::new("explosion"),
            MovingObjectBundle {
                velocity: velocity.clone(),
                acceleration: Acceleration::from(acceleration.clone()),
                rotation: rotation.clone(),
                collider: Collider::new(spawn_props.radius),
                model: SceneBundle {
                    scene: SceneRoot(scene_assets.explosion.clone()),
                    transform: debris_xform,
                },
            },
            Health::new(spawn_props.health),
            Explosion {duration: 0},
        ));

        commands.spawn((
            Name::new("asteroid_debris"),
            MovingObjectBundle {
                velocity: debris_velocity.clone(),
                acceleration: Acceleration::new(acceleration.value * 0.1),
                rotation: rotation.clone(),
                collider: Collider::new(spawn_props.radius),
                model: SceneBundle {
                    scene: SceneRoot(scene_assets.asteroid_debris.clone()),
                    transform: debris_xform,
                },
            },
            AsteroidDebris,
        ));

    }
}

fn update_explosion_animation(
    mut query: Query<(&mut Explosion, &mut Health, &mut Transform), With<Explosion>>,
    time: Res<Time>,
) {
    for (mut explosion, mut health, mut xform) in query.iter_mut() {
        explosion.duration += 1;
        health.value -= 160.0 * time.delta_secs();
        xform.scale *= 1.0 + (12.0 * time.delta_secs());
    }
}

fn split_asteroid(
    mut commands: Commands,
    scene_assets: Res<SceneAssets>,
    mut event_reader: MessageReader<AsteroidSpawnChildrenEvent>,
) {
    for &AsteroidSpawnChildrenEvent {
        ref xform,
        ref velocity,
        ref rotation,
        ref acceleration,
        level
    } in event_reader.read() {
        if level >= 1 {
            let new_level = if level >= 1 {level - 1} else {level};

            let spawn_props = &ASTEROID_SIZE_PROPS[new_level];

            let scene: Handle<Scene>;

            if new_level >= 2 {
                scene = scene_assets.asteroid_big.clone();
            }
            else if new_level >= 1 {
                scene = scene_assets.asteroid_medium.clone();
            }
            else {
                scene = scene_assets.asteroid.clone();
            }

            let new_velocity_vec = velocity.value.rotate_y(PI / 2.0).normalize();
            let new_left_velocity = Velocity::new(
                velocity.value + (new_velocity_vec * 2.0)
            );
            let new_left_acceleration = Acceleration::new(
                new_left_velocity.value.normalize() * acceleration.value.length()
            );

            let new_right_velocity = Velocity::new(
                velocity.value - (new_velocity_vec * 2.0)
            );
            let new_right_acceleration = Acceleration::new(
                new_right_velocity.value.normalize() * acceleration.value.length()
            );

            commands.spawn((
                Name::new("asteroid"),
                MovingObjectBundle {
                    velocity: new_left_velocity,
                    acceleration: new_left_acceleration,
                    rotation: rotation.clone(),
                    collider: Collider::new(spawn_props.radius),
                    model: SceneBundle {
                        scene: SceneRoot(scene.clone()),
                        transform: *xform,
                    },
                },
                Asteroid::new(new_level),
                Health::new(spawn_props.health),
                CollisionDamage::new(spawn_props.collision_damage),
            ));

            commands.spawn((
                Name::new("asteroid"),
                MovingObjectBundle {
                    velocity: new_right_velocity,
                    acceleration: new_right_acceleration,
                    rotation: rotation.clone(),
                    collider: Collider::new(spawn_props.radius),
                    model: SceneBundle {
                        scene: SceneRoot(scene.clone()),
                        transform: *xform,
                    },
                },
                Asteroid::new(new_level),
                Health::new(spawn_props.health),
                CollisionDamage::new(spawn_props.collision_damage),
            ));

        }

    }
}
