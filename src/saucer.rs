use bevy::{
    audio::Volume,
    prelude::*,
};
use rand::Rng;

use crate::{
    ambient_sound::SaucerSound,
    asset_loader::SceneAssets,
    asteroids::{Asteroid, SPAWN_RANGE_X, SPAWN_RANGE_Z},
    collision_detection::{Collider, CollisionDamage},
    health::Health,
    movement::{
        Acceleration,
        MovingObjectBundle,
        Rotation,
        SceneBundle,
        Velocity,
    },
    schedule::InGameSet,
    spaceship::Spaceship,
    sound_fx::SaucerShootingSoundEvent,
};


const SAUCER_STARTING_VELOCITY: Vec3 = Vec3::new(1.0, 0.0, -1.0);
pub const SAUCER_RADIUS: f32 = 2.5;
const SAUCER_SIZE: f32 = 0.7;
const SAUCER_HEALTH: f32 = 100.0;
const SAUCER_COLLISION_DAMAGE: f32 = 100.0;
const SAUCER_SPAWN_TIME_SECONDS: f32 = 45.0;

const SAUCER_MISSILE_FORWARD_SPAWN_SCALAR: f32 = 4.0;
const SAUCER_MISSILE_RADIUS: f32 = 0.5;
const SAUCER_MISSILE_SIZE: f32 = 0.10;
const SAUCER_MISSILE_RATE: f32 = 60.0;  // shots per second
const SAUCER_MISSILE_SPEED: f32 = 40.0;
const SAUCER_MISSILE_HEALTH: f32 = 1.0;
const SAUCER_MISSILE_COLLISION_DAMAGE: f32 = 7.0;


#[derive(Component, Debug)]
pub struct Saucer;

#[derive(Component, Debug)]
pub struct SaucerMissile;

#[derive(Resource, Debug)]
pub struct SaucerSpawnTimer {
    pub timer: Timer,
}

#[derive(Resource, Debug)]
pub struct SaucerMissileRateTimer {
    timer: Timer,
}


#[derive(Message, Debug)]
pub struct SaucerSpawnEvent;


pub struct SaucerPlugin;

impl Plugin for SaucerPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(SaucerSpawnTimer {
            timer: Timer::from_seconds(
                SAUCER_SPAWN_TIME_SECONDS,
                TimerMode::Repeating
            )
        })
        .insert_resource(SaucerMissileRateTimer {
            timer: Timer::from_seconds(
                1.0 / SAUCER_MISSILE_RATE,
                TimerMode::Repeating,
            )
        })
        .add_systems(Update, (
                trigger_spawn_saucer,
                handle_saucer_spawn_event,
                saucer_movement,
                saucer_sound_control,
                saucer_weapon_control,
            ).in_set(InGameSet::EntityUpdates),
        )
        .add_message::<SaucerSpawnEvent>();
    }
}


fn trigger_spawn_saucer(
    mut spawn_timer: ResMut<SaucerSpawnTimer>,
    mut saucer_spawn_event_writer: MessageWriter<SaucerSpawnEvent>,
    time: Res<Time>,
) {
    spawn_timer.timer.tick(time.delta());

    if !spawn_timer.timer.just_finished() {
        return;
    }

    saucer_spawn_event_writer.write(SaucerSpawnEvent);
}


fn handle_saucer_spawn_event(
    mut commands: Commands,
    mut event_reader: MessageReader<SaucerSpawnEvent>,
    spaceship_xform: Single<&Transform, With<Spaceship>>,
    scene_assets: Res<SceneAssets>,
) {
    let mut rng = rand::rng();

    let mut translation = Vec3::new(
        rng.random_range(SPAWN_RANGE_X),
        0.0,
        rng.random_range(SPAWN_RANGE_Z),
    );

    for _i in 0..2 {
        // It is a bit unfair to have a saucer spawn right on top of the
        // spaceship.  So we allow a (finite) number of chances to choose
        // a different location if this happens.
        // There is still a tiny chance of this happening, but it will be
        // considerably less annoying.  Without this, it happens about
        // once per game.
        let distance = translation.distance(spaceship_xform.translation);

        if distance >= SAUCER_RADIUS * 4.0 {
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

    for _spawn_event in event_reader.read() {
        let saucer_xform = Transform::from_translation(translation)
        .with_scale(Vec3::ONE * SAUCER_SIZE)
        .with_rotation(Quat::from_rotation_x(std::f32::consts::PI / 2.));

        let _saucer_id = commands.spawn((
            Name::new("saucer"),
            MovingObjectBundle {
                velocity: Velocity::new(SAUCER_STARTING_VELOCITY),
                acceleration: Acceleration::new(Vec3::ZERO),
                rotation: Rotation::new(
                    0.0,
                    0.0,
                    0.0
                ),
                collider: Collider::new(SAUCER_RADIUS),
                model: SceneBundle {
                    scene: SceneRoot(scene_assets.saucer.clone()),
                    transform: saucer_xform,
                }
            },
            Saucer,
            Health::new(SAUCER_HEALTH),
            CollisionDamage::new(SAUCER_COLLISION_DAMAGE),
        )).id();
        
        #[cfg(debug_assertions)]
        info!("\tSpawned Saucer ({:?})", _saucer_id);

    }
}


fn saucer_movement(
    mut query: Query<(Entity, &Transform, &Velocity, &mut Acceleration), With<Saucer>>,
    things_to_avoid: Query<(Entity, &Transform), Or<(With<Asteroid>, With<Saucer>)>>,
) {
    for (entity, saucer_xform, saucer_vel, mut saucer_accel) in query.iter_mut() {
        // We have a slight attraction to the center of the screen just so that
        // the saucer isn't constantly hugging the edges of the screen.  The
        // gameplay is just not as pleasing.
        //let attraction_to_center = (Vec3::ZERO - saucer_xform.translation) * 0.25;
        let attraction_to_center = (Vec3::ZERO - saucer_xform.translation).normalize();
        
        // We allow the asteroids to damage the saucer, so we want the saucer
        // to avoid them.  Choose the closest asteroid to the saucer and repel
        // in the opposite direction.  This isn't perfect avoidance, but we
        // want a little bit of fallibility.  We may change this if it becomes
        // undesireable in the future.
        let mut closest_distance: f32 = f32::MAX;
        let mut repellance_to_closest: Vec3 = Vec3::ZERO;
        for (obstacle_entity, obstacle_xform) in things_to_avoid.iter() {
            if obstacle_entity == entity {
                continue;  // This is us
            }
            if closest_distance > obstacle_xform.translation.distance(saucer_xform.translation) {
                closest_distance = obstacle_xform.translation.distance(saucer_xform.translation);
                repellance_to_closest = obstacle_xform.translation / closest_distance * 16.0;
            };
        }

        saucer_accel.value = attraction_to_center - repellance_to_closest;

        if saucer_vel.value.distance(Vec3::ZERO) > 20.0 {
            // we are going too fast.  Put on the brakes
            saucer_accel.value -= saucer_vel.value;
        }
    }
}


fn saucer_sound_control(
    saucers: Query<&Saucer>,
    mut saucer_audio: Query<&mut AudioSink, With<SaucerSound>>,
) {
    let Ok(mut sink) = saucer_audio.single_mut() else {
        return;
    };

    if saucers.count() > 0 {
        sink.set_volume(Volume::Linear(1.0));
    } else {
        sink.set_volume(Volume::Linear(0.0));
    }
}


fn saucer_weapon_control(
    mut commands: Commands,
    mut rate_timer: ResMut<SaucerMissileRateTimer>,
    time: Res<Time>,

    saucers: Query<&Transform, With<Saucer>>,
    spaceship_xform: Single<&Transform, With<Spaceship>>,
    mut sound_event_writer: MessageWriter<SaucerShootingSoundEvent>,
    scene_assets: Res<SceneAssets>,

) {
    rate_timer.timer.tick(time.delta());

    if rate_timer.timer.is_finished()
    {
        let mut rng = rand::rng();

        for saucer_xform in saucers.iter() {
            let missile_chance: i32 = rng.random_range(0.0..SAUCER_MISSILE_RATE) as i32;
            
            if missile_chance == SAUCER_MISSILE_RATE as i32 / 2 {
                // shoot a missile
                // This should fire at random times, but average about 1/s per saucer.
                let mut missile_xform = Transform::from_translation(
                    saucer_xform.translation
                ).looking_at(spaceship_xform.translation, Vec3::Z)
                .with_scale(Vec3::ONE * SAUCER_MISSILE_SIZE);

                missile_xform.translation += missile_xform.forward() * SAUCER_MISSILE_FORWARD_SPAWN_SCALAR;

                commands.spawn((
                    Name::new("saucer_missile"),
                    MovingObjectBundle {
                        velocity: Velocity::new(missile_xform.forward() * SAUCER_MISSILE_SPEED),
                        acceleration: Acceleration::new(Vec3::ZERO),
                        rotation: Rotation::new(0.0, 0.0, 0.0),
                        collider: Collider::new(SAUCER_MISSILE_RADIUS),
                        model: SceneBundle {
                            scene: SceneRoot(scene_assets.saucer_missile.clone()),
                            transform: missile_xform,
                        },
                    },
                    SaucerMissile,
                    Health::new(SAUCER_MISSILE_HEALTH),
                    CollisionDamage::new(SAUCER_MISSILE_COLLISION_DAMAGE),
                ));

                sound_event_writer.write(SaucerShootingSoundEvent);
            }
        }
    }
}
