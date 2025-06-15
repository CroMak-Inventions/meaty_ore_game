use bevy::prelude::*;
use bevy::render::camera::CameraProjection;

use crate::collision_detection::Collider;

use super::asset_loader::SceneAssets;
use super::movement::{Acceleration, Velocity, MovingObjectBundle, SceneBundle};


const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, -20.0);
const STARTING_VELOCITY: Vec3 = Vec3::new(0.0, 0.0, 1.0);
const SPACESHIP_SPEED: f32 = 25.0;
const SPACESHIP_ROTATION_SPEED: f32 = 2.5;
const SPACESHIP_ROLL_SPEED: f32 = 2.5;
const SPACESHIP_RADIUS: f32 = 2.5;

const MISSILE_SPEED: f32 = 50.0;
const MISSILE_RATE: f32 = 4.0;  // shots per second
const MISSILE_MAX: usize = 3;  // maximum number of missiles allowed in the air
const MISSILE_FORWARD_SPAWN_SCALAR: f32 = 7.5;
const MISSILE_RADIUS: f32 = 0.5;


#[derive(Component, Debug)]
pub struct Spaceship;

#[derive(Component, Debug)]
pub struct SpaceshipMissile;

#[derive(Resource, Debug)]
pub struct MissileRateTimer {
    timer: Timer,
}


pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MissileRateTimer {
            timer: Timer::from_seconds(
                1.0 / MISSILE_RATE,
                TimerMode::Once,
            )
        })
        .add_systems(PostStartup, spawn_spaceship)
        .add_systems(Update, (
                spaceship_movement_controls,
                spawn_missles,
                despawn_missles
        ));
    }
}

fn spawn_spaceship(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands.spawn((
        MovingObjectBundle {
            velocity: Velocity::new(STARTING_VELOCITY),
            acceleration: Acceleration::new(Vec3::ZERO),
            collider: Collider::new(SPACESHIP_RADIUS),
            model: SceneBundle {
                scene: SceneRoot(scene_assets.spaceship.clone()),
                transform: Transform::from_translation(STARTING_TRANSLATION),
            }
        },
        Spaceship
    ));
}

fn spaceship_movement_controls(
    mut query: Query<(&mut Transform, &mut Velocity), With<Spaceship>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let (mut transform, mut velocity) = query.single_mut().unwrap();
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
    velocity.value = -transform.forward() * movement;
}


fn spawn_missles(
    mut commands: Commands,
    mut rate_timer: ResMut<MissileRateTimer>,
    time: Res<Time>,
    query: Query<&Transform, With<Spaceship>>,
    missile_query: Query<(), With<SpaceshipMissile>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    scene_assets: Res<SceneAssets>,
) {
    rate_timer.timer.tick(time.delta());

    let transform = query.single().unwrap();
    let mut missile_xform = Transform::from_translation(
        transform.translation + -transform.forward() * MISSILE_FORWARD_SPAWN_SCALAR,
    );
    missile_xform.rotate(transform.rotation);

    if keyboard_input.pressed(KeyCode::Space) &&
       rate_timer.timer.finished()
    {
        let missile_number = missile_query.iter().len();
        println!("number of missiles spawned: {:?}", missile_number);

        if missile_number < MISSILE_MAX {
            rate_timer.timer.reset();

            commands.spawn((
                MovingObjectBundle {
                    velocity: Velocity::new(-transform.forward() * MISSILE_SPEED),
                    acceleration: Acceleration::new(Vec3::ZERO),
                    collider: Collider::new(MISSILE_RADIUS),
                    model: SceneBundle {
                        scene: SceneRoot(scene_assets.missiles.clone()),
                        transform: missile_xform,
                    },
                },
                SpaceshipMissile,
            ));
        }

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
            println!("bounds: [{:?}, {:?}], [{:?}, {:?}]", min_x, max_x, min_z, max_z);
            println!("Despawn entity: {:?}", entity);
            commands.entity(entity).despawn();
        }
    }

}