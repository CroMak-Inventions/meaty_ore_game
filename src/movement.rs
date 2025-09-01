use rand::Rng;

use bevy::{
    prelude::*,
    render::camera::CameraProjection
};

use crate::{
    asteroids::Asteroid,
    collision_detection::Collider,
    schedule::InGameSet,
    spaceship::Spaceship
};


#[derive(Component, Debug, Clone)]
pub struct Velocity {
    pub value: Vec3,
}

impl Velocity {
    pub fn new(value: Vec3) -> Self {
        Self {value}
    }
}

#[derive(Component, Debug, Clone)]
pub struct Acceleration {
    pub value: Vec3,
}

impl Acceleration {
    pub fn new(value: Vec3) -> Self {
        Self {value}
    }
}

#[derive(Component, Debug, Clone)]
pub struct Rotation {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Rotation {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {x, y, z}
    }

    pub fn random(min_range: f32, max_range: f32) -> Self {
        let mut rng = rand::rng();

        Self {
            x: rng.random_range(min_range..max_range),
            y: rng.random_range(min_range..max_range),
            z: rng.random_range(min_range..max_range),
        }
    }
}


#[derive(Bundle)]
pub struct SceneBundle {
    pub scene: SceneRoot,
    pub transform: Transform
}

#[derive(Bundle)]
pub struct MovingObjectBundle {
    pub velocity: Velocity,
    pub acceleration: Acceleration,
    pub rotation: Rotation,
    pub collider: Collider,
    pub model: SceneBundle,
}


pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
                update_velocity,
                update_position,
                wrap_position::<Asteroid>,
                wrap_position::<Spaceship>,
            )
            .chain()
            .in_set(InGameSet::EntityUpdates),
        );
    }
}

fn update_velocity(mut query: Query<(&Acceleration, &mut Velocity)>, time: Res<Time>) {
    for (acceleration, mut velocity) in query.iter_mut() {
        velocity.value += acceleration.value * time.delta_secs();
    }
}

fn update_position(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (velocity, mut position) in query.iter_mut() {
        position.translation += velocity.value * time.delta_secs();
    }
}

fn wrap_position<T: Component>(
    camera_query: Query<&Projection, With<Camera>>,
    mut query: Query<&mut Transform, With<T>>,
) {
    // Wrap the positions of the objects so that they don't just go off
    // into infinity, but wrap to the other side of the screen.

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
    
    for mut transform in query.iter_mut() {
        if transform.translation.x < min_x {
            transform.translation.x = max_x - (min_x - transform.translation.x);
            continue;
        }

        if transform.translation.x > max_x {
            transform.translation.x = min_x + transform.translation.x - max_x;
            continue;
        }

        if transform.translation.z < min_z {
            transform.translation.z = max_z - (min_z - transform.translation.z);
            continue;
        }

        if transform.translation.z > max_z {
            transform.translation.z = min_z + transform.translation.z - max_z;
            continue;
        }
    }
}
