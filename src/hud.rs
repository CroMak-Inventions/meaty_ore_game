use bevy::prelude::*;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, hud_setup);
    }
}

fn hud_setup() {
    info!("HUD: setup");
}
