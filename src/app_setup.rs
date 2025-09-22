use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowMode};


pub struct AppSetupPlugin;

impl Plugin for AppSetupPlugin {
    fn build(&self, app: &mut App) {
        // System defined plugings
        app.insert_resource(ClearColor(Color::linear_rgb(0.0005,0.0,0.005)))
            .insert_resource(AmbientLight {
                color: Color::default(),
                brightness: 250.0,
                affects_lightmapped_meshes: true
            })
            .add_plugins(DefaultPlugins)
            // User defined plugins
            .add_systems(Startup, change_window_mode);
    }
}


fn change_window_mode(mut windows: Query<&mut Window, With<PrimaryWindow>>) {
    // Query returns one window typically.
    for mut window in windows.iter_mut() {
        window.mode =
            WindowMode::Fullscreen(MonitorSelection::Current, VideoModeSelection::Current);
    }
}
