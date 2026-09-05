use bevy::prelude::*;
use bevy::window::{Monitor, MonitorSelection, PrimaryWindow, WindowMode};


pub struct AppSetupPlugin;

impl Plugin for AppSetupPlugin {
    fn build(&self, app: &mut App) {
        // System defined plugings
        app.insert_resource(ClearColor(Color::linear_rgb(0.0005,0.0,0.005)))
            .insert_resource(GlobalAmbientLight {
                color: Color::default(),
                brightness: 250.0,
                affects_lightmapped_meshes: true
            })
            
            .add_plugins(DefaultPlugins)
            //.add_plugins(DefaultPlugins.set(WindowPlugin {
            //    primary_window: Some(Window {
            //        title: "Raspberry Pi App".into(),
            //        // Centers on the Primary screen rather than looking for a "Current" context
            //        position: WindowPosition::Centered(MonitorSelection::Primary),
            //        ..default()
            //    }),
            //    ..default()
            //}))

            // User defined plugins
            .add_systems(Startup, change_window_mode)
            .add_systems(PostStartup, check_current_monitor);
    }
}


fn change_window_mode(mut windows: Query<&mut Window, With<PrimaryWindow>>) {
    // Query returns one window typically.
    for mut window in windows.iter_mut() {
        window.mode =
            WindowMode::Fullscreen(MonitorSelection::Current, VideoModeSelection::Current);
    }
}

fn check_current_monitor(
    // 1. Get the Primary Window entity
    primary_window_query: Query<Entity, With<PrimaryWindow>>,
    // 2. Query for monitors and check which window they are tied to
    // (Note: Adjust target syntax if you use custom entity relationships in your version)
    monitors: Query<(&Monitor, Entity)>,
) {
    if let Ok(_window_entity) = primary_window_query.single() {
        // Under the hood, Bevy keeps track of monitors as entities.
        // You can iterate over monitors to read their name, physical size, or scale factor.
        for (monitor, _monitor_entity) in monitors.iter() {
            // Because Wayland updates this asynchronously, the data becomes populated 
            // once the window is actually drawn on a specific display.
            println!("Detected Monitor: {:?} | Scale Factor: {}", monitor.name, monitor.scale_factor);
        }
    }
}