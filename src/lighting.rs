use bevy::prelude::*;
use bevy::light::CascadeShadowConfigBuilder;


pub struct LightingPlugin;

impl Plugin for LightingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_point_light);
    }
}

fn spawn_point_light(mut commands: Commands) {
    commands.spawn((
       DirectionalLight {
            illuminance: light_consts::lux::FULL_DAYLIGHT,
            shadows_enabled: true,
            ..default()
        },
        Transform {
            translation: Vec3::new(10.0, 80.0, 10.0),
            //rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        // The default cascade config is designed to handle large scenes.
        // As this example has a much smaller world, we can tighten the shadow
        // bounds for better visual quality.
        CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 80.0,
            ..default()
        }
        .build(),
    ));
}
