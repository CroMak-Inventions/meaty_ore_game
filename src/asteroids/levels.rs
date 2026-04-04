use std::{
    ops::Range, str::FromStr, sync::LazyLock
};


pub struct AsteroidSpawnProperties {
    pub level: usize,
    pub scene_name: String,
    pub velocity_scalar: f32,
    pub acceleration_scalar: f32,
    pub spawn_range_x: Range<f32>,
    pub spawn_range_z: Range<f32>,
    pub max_rotate_speed: f32,
    pub radius: f32,
    pub health: f32,
    pub collision_damage: f32,
}

impl AsteroidSpawnProperties {
    pub fn new(
        level: usize,
        scene_name: String,
        velocity_scalar: f32,
        acceleration_scalar: f32,
        spawn_range_x: Range<f32>,
        spawn_range_z: Range<f32>,
        max_rotate_speed: f32,
        radius: f32,
        health: f32,
        collision_damage: f32,
    ) -> Self {
        Self {
            level,
            scene_name,
            velocity_scalar,
            acceleration_scalar,
            spawn_range_x,
            spawn_range_z,
            max_rotate_speed,
            radius,
            health,
            collision_damage,
        }
    }
}


// This is an array of properties we will use to spawn asteroids at
// different levels.  This allows us to split larger asteroids into
// progressively smaller asteroids.
pub static ASTEROID_LEVEL_PROPS: [LazyLock<AsteroidSpawnProperties>; 3] = [
    LazyLock::new(|| {
        AsteroidSpawnProperties::new(
            0,  // standard small asteroid
            String::from("asteroid"),
            5.0,
            0.75,
            -25.0..25.0,
            -25.0..25.0,
            3.0,
            1.5,
            20.0,
            35.0,
        )
    }),
    LazyLock::new(|| {
        AsteroidSpawnProperties::new(
            1,  // Medium asteroid
            String::from("asteroid_medium"),
            5.0,
            0.5,
            -25.0..25.0,
            -25.0..25.0,
            2.5,
            3.0,
            60.0,
            70.0,
        )
    }),
    LazyLock::new(|| {
        AsteroidSpawnProperties::new(
            2,  // Big asteroid
            String::from("asteroid_big"),
            5.0,
            0.5,
            -25.0..25.0,
            -25.0..25.0,
            2.0,
            4.2,
            100.0,
            140.0,
        )
    }),
];