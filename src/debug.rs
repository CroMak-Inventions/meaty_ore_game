use bevy::prelude::*;

use crate::schedule::InGameSet;
use crate::movement::Velocity;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            print_position.after(InGameSet::EntityUpdates));
    }
}

fn print_position(query: Query<(Entity, &Transform), With<Velocity>>) {
    for (entity, position) in query.iter() {
        println!("Entity: {:?}, Position: {:?}", entity, position.translation);
    }
}
