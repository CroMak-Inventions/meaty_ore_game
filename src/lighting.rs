use bevy::{
    camera::PhysicalCameraParameters,
    prelude::*,
    light::CascadeShadowConfigBuilder,
};

#[derive(Resource, Default, Deref, DerefMut)]
struct Parameters(PhysicalCameraParameters);

pub struct LightingPlugin;

impl Plugin for LightingPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Parameters(PhysicalCameraParameters {
                aperture_f_stops: 1.0,
                shutter_speed_s: 1.0 / 125.0,
                sensitivity_iso: 100.0,
                sensor_height: 0.01866,
            }))
            .add_systems(PostStartup, spawn_point_light);
    }
}

fn spawn_point_light(mut commands: Commands) {
    commands.spawn((
        Name::new("lighting"),
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
