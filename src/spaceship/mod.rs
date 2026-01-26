use bevy::{
    prelude::*,
    audio::Volume,
};

use crate::{
    ambient_sound::ThrusterSound,
    asset_loader::SceneAssets,
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
    sound_fx::ShootingSoundEvent,
    state::GameState,
};

mod shield;

const SPACESHIP_STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, -20.0);
const SPACESHIP_STARTING_VELOCITY: Vec3 = Vec3::new(0.0, 0.0, 1.0);
pub const SPACESHIP_RADIUS: f32 = 2.5;
const SPACESHIP_SIZE: f32 = 0.8;
const SPACESHIP_SPEED: f32 = 25.0;
const SPACESHIP_ROTATION_SPEED: f32 = 2.5;
const SPACESHIP_ROLL_SPEED: f32 = 2.5;
const SPACESHIP_HEALTH: f32 = 100.0;
const SPACESHIP_COLLISION_DAMAGE: f32 = 100.0;

const SHIELD_COOLDOWN_SECS: f32 = 4.0;

const MISSILE_FORWARD_SPAWN_SCALAR: f32 = 5.0;
const MISSILE_RADIUS: f32 = 0.5;
const MISSILE_SPEED: f32 = 50.0;
const MISSILE_HEALTH: f32 = 1.0;
const MISSILE_COLLISION_DAMAGE: f32 = 5.0;
const MISSILE_RATE: f32 = 4.0;  // shots per second
const MISSILE_MAX: usize = 3;  // maximum number of missiles allowed in the air


#[derive(Component, Debug)]
pub struct Spaceship;

#[derive(Component, Debug)]
pub struct SpaceshipMissile;

#[derive(Component, Debug)]
pub struct ShieldController {
    pub state: ShieldState,
    pub cooldown: Timer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShieldState {
    Ready,
    Active,
    Cooldown,
}

/// One-frame intent marker inserted by input, consumed by shield systems.
#[derive(Component, Debug)]
pub struct ShieldRequest;

#[derive(Resource, Debug)]
pub struct MissileRateTimer {
    timer: Timer,
}

pub struct SpaceshipPlugin;

pub use shield::{Shield, ShieldHitCooldown};

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MissileRateTimer {
            timer: Timer::from_seconds(
                1.0 / MISSILE_RATE,
                TimerMode::Once,
            )
        })
        .add_systems(PostStartup, spawn_spaceship)
        .add_systems(OnEnter(GameState::GameOver), spawn_spaceship)
        .add_systems(Update,
            (
                spaceship_movement_controls,
                spaceship_weapon_controls,
                spaceship_shield_controls,
                spaceship_thruster_sound_control,
            )
            .chain()
            .in_set(InGameSet::UserInput)
        )
        .add_systems(Update,
            spaceship_destroyed.in_set(InGameSet::EntityUpdates))
        .add_systems(Update,
            despawn_missles.in_set(InGameSet::DespawnEntities),
        );
        shield::register(app);
    }
}

fn spawn_spaceship(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    let spaceship_xform = Transform::from_translation(
        SPACESHIP_STARTING_TRANSLATION
    ).with_scale(
        Vec3::ONE * SPACESHIP_SIZE
    );

    let mut cooldown = Timer::from_seconds(SHIELD_COOLDOWN_SECS, TimerMode::Once);
    cooldown.set_elapsed(cooldown.duration()); // mark finished

    commands.spawn((
        MovingObjectBundle {
            velocity: Velocity::new(SPACESHIP_STARTING_VELOCITY),
            acceleration: Acceleration::new(Vec3::ZERO),
            rotation: Rotation::new(0.0, 0.0, 0.0),
            collider: Collider::new(SPACESHIP_RADIUS),
            model: SceneBundle {
                scene: SceneRoot(scene_assets.spaceship.clone()),
                transform: spaceship_xform,
            }
        },
        Spaceship,
        ShieldController {
            state: ShieldState::Ready,
            cooldown,
        },
        Health::new(SPACESHIP_HEALTH),
        CollisionDamage::new(SPACESHIP_COLLISION_DAMAGE),
    ));
}

fn spaceship_movement_controls(
    mut query: Query<(&mut Transform, &mut Acceleration), With<Spaceship>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let Ok((mut transform, mut acceleration)) = query.single_mut() else {
        return;
    };

    let mut rotation = 0.0;
    let mut roll = 0.0;
    let mut movement = 0.0;

    // Forward or Backward
    if keyboard_input.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]) {
        movement = -SPACESHIP_SPEED;
    }
    else if keyboard_input.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]) {
        movement = SPACESHIP_SPEED;
    }

    // Rotate left or right
    if keyboard_input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]) {
        rotation = -SPACESHIP_ROTATION_SPEED * time.delta_secs();
    }
    else if keyboard_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]) {
        rotation = SPACESHIP_ROTATION_SPEED * time.delta_secs();
    }

    // Roll left or right
    if keyboard_input.pressed(KeyCode::ShiftLeft) {
        roll = -SPACESHIP_ROLL_SPEED * time.delta_secs();
    }
    else if keyboard_input.pressed(KeyCode::ControlLeft) {
        roll = SPACESHIP_ROLL_SPEED * time.delta_secs();
    }

    // Rotate around the Y axis.
    // Ignores the Z axis rotation applied below.
    transform.rotate_y(rotation);

    // Rotate around the local Z axis.
    // The rotation is relative to the current rotation.
    transform.rotate_local_z(roll);

    // update the spaceship's velocity based on new direction
    acceleration.value = -transform.forward() * movement;
}


fn spaceship_weapon_controls(
    mut commands: Commands,
    mut rate_timer: ResMut<MissileRateTimer>,
    time: Res<Time>,
    spaceship_query: Query<&Transform, With<Spaceship>>,
    missile_query: Query<(), With<SpaceshipMissile>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut sound_event_writer: MessageWriter<ShootingSoundEvent>,
    scene_assets: Res<SceneAssets>,
) {
    rate_timer.timer.tick(time.delta());

    let Ok(spaceship_xform) = spaceship_query.single() else {
        return;
    };

    if keyboard_input.pressed(KeyCode::Space) &&
       rate_timer.timer.is_finished()
    {
        let missile_number = missile_query.iter().len();

        if missile_number < MISSILE_MAX {
            rate_timer.timer.reset();

            let mut missile_xform = Transform::from_translation(
                spaceship_xform.translation + -spaceship_xform.forward() * MISSILE_FORWARD_SPAWN_SCALAR,
            );
            missile_xform.rotate(spaceship_xform.rotation);

            commands.spawn((
                MovingObjectBundle {
                    velocity: Velocity::new(-spaceship_xform.forward() * MISSILE_SPEED),
                    acceleration: Acceleration::new(Vec3::ZERO),
                    rotation: Rotation::new(0.0, 0.0, 0.0),
                    collider: Collider::new(MISSILE_RADIUS),
                    model: SceneBundle {
                        scene: SceneRoot(scene_assets.missiles.clone()),
                        transform: missile_xform,
                    },
                },
                SpaceshipMissile,
                Health::new(MISSILE_HEALTH),
                CollisionDamage::new(MISSILE_COLLISION_DAMAGE),
            ));

            sound_event_writer.write(ShootingSoundEvent);
        }
    }
}


fn spaceship_shield_controls(
    mut commands: Commands,
    query: Query<Entity, With<Spaceship>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let Ok(spaceship) = query.single() else {
        return;
    };

    if keyboard_input.just_pressed(KeyCode::Tab) {
        commands.entity(spaceship).insert(ShieldRequest);
    }
}

fn despawn_missles(
    mut commands: Commands,
    camera_query: Query<&Projection, With<Camera>>,
    missile_query: Query<(Entity, &Transform), With<SpaceshipMissile>>,
) {
    // When a missile goes off-screen, we despawn it.
    let mut min_x: f32 = 0.0;
    let mut max_x: f32 = 0.0;
    let mut min_z: f32 = 0.0;
    let mut max_z: f32 = 0.0;

    let projection = camera_query.single().unwrap();
    let bounds = projection.get_frustum_corners(0.0, 80.0);
    for b in bounds {
        if min_x > b.x {min_x = b.x}
        if max_x < b.x {max_x = b.x}
        if min_z > b.y {min_z = b.y}
        if max_z < b.y {max_z = b.y}
    }
    
    for (entity, transform) in missile_query.iter() {
        if transform.translation.x < min_x ||
           transform.translation.x > max_x ||
           transform.translation.z < min_z ||
           transform.translation.z > max_z
        {
            //println!("despawn_missiles(): Despawn entity: {:?}", entity);
            commands.entity(entity).despawn();
        }
    }

}

fn spaceship_destroyed(
    mut next_state: ResMut<NextState<GameState>>,
    query: Query<(), With<Spaceship>>,
) {
    if query.single().is_err() {
        next_state.set(GameState::GameOver);
    }
}

fn spaceship_thruster_sound_control(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut thruster_audio: Query<&mut AudioSink, With<ThrusterSound>>,
) {
    let Ok(mut sink) = thruster_audio.single_mut() else {
        return;
    };

    if keyboard_input.any_pressed([
        KeyCode::ArrowUp,
        KeyCode::ArrowDown,
        KeyCode::KeyW,
        KeyCode::KeyS,
    ]) {
        sink.set_volume(Volume::Linear(1.0));
    } else {
        sink.set_volume(Volume::Linear(0.0));
    }
}
