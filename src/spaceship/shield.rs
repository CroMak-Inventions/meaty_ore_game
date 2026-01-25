use bevy::prelude::*;
use crate::{
    collision_detection::Collider,
    health::Health,
};
use super::{ShieldController, ShieldRequest, ShieldState, Spaceship, SPACESHIP_RADIUS};


pub fn register(app: &mut App) {
    app.add_systems(
        Update,
        consume_shield_request.in_set(InGameSet::UserInput), // same phase for now
    );
}

fn consume_shield_request(
    mut commands: Commands,
    mut q: Query<(Entity, &mut ShieldController), (With<Spaceship>, With<ShieldRequest>)>,
) {
    let Ok((ship_entity, mut controller)) = q.single_mut() else {
        return;
    };

    // For Step 4: just update state machine skeleton.
    match controller.state {
        ShieldState::Ready => {
            controller.state = ShieldState::Active;
            // Step 5 will spawn the actual shield entity.
            info!("Shield requested: transitioning Ready -> Active");
        }
        ShieldState::Active => {
            // If we later do toggle, we’ll handle it here.
            info!("Shield requested while Active (toggle TBD)");
        }
        ShieldState::Cooldown => {
            info!("Shield requested during Cooldown (ignored)");
        }
    }

    // Always consume the one-frame intent marker
    commands.entity(ship_entity).remove::<ShieldRequest>();
}