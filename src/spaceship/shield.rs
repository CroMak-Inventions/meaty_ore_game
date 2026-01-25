use bevy::prelude::*;

use crate::{collision_detection::Collider, health::Health, schedule::InGameSet};
use super::{ShieldController, ShieldRequest, ShieldState, Spaceship, SPACESHIP_RADIUS};

#[derive(Component, Debug)]
pub struct Shield {
    pub ship: Entity,
}

const SHIELD_RADIUS: f32 = SPACESHIP_RADIUS * 1.35;
const SHIELD_HP: f32 = 60.0;

pub fn register(app: &mut App) {
    app.add_systems(
        Update,
        (
            consume_shield_request,
            shield_follow_ship,
            shield_death_starts_cooldown,
            tick_shield_cooldown,
        )
        .in_set(InGameSet::EntityUpdates),
    );
}


fn consume_shield_request(
    mut commands: Commands,
    mut q: Query<(Entity, &mut ShieldController), (With<Spaceship>, With<ShieldRequest>)>,
) {
    let Ok((ship_entity, mut controller)) = q.single_mut() else { return; };

    match controller.state {
        ShieldState::Ready => {
            commands.spawn((
                Shield { ship: ship_entity },
                Health::new(SHIELD_HP),
                Collider::new(SHIELD_RADIUS),
                Transform::default(),
                GlobalTransform::default(),
            ));

            controller.state = ShieldState::Active;
            info!("Shield spawned: Ready -> Active");
        }
        ShieldState::Active => {
            info!("Shield requested while Active (toggle TBD)");
        }
        ShieldState::Cooldown => {
            info!("Shield requested during Cooldown (ignored)");
        }
    }

    commands.entity(ship_entity).remove::<ShieldRequest>();
}

fn shield_follow_ship(
    ship_q: Query<(Entity, &GlobalTransform), With<Spaceship>>,
    mut shield_q: Query<(&Shield, &mut Transform)>,
) {
    let Ok((ship_e, ship_gt)) = ship_q.single() else { return; };

    // Convert GlobalTransform to a local Transform we can apply to the shield.
    let ship_tf = ship_gt.compute_transform();

    for (shield, mut shield_tf) in shield_q.iter_mut() {
        if shield.ship != ship_e {
            continue;
        }
        *shield_tf = ship_tf;
    }
}

fn shield_death_starts_cooldown(
    mut commands: Commands,
    shield_q: Query<(Entity, &Health, &Shield)>,
    mut ship_q: Query<&mut ShieldController, With<Spaceship>>,
) {
    for (shield_entity, health, shield) in shield_q.iter() {
        if health.value <= 0.0 {
            commands.entity(shield_entity).despawn();

            if let Ok(mut controller) = ship_q.get_mut(shield.ship) {
                controller.state = ShieldState::Cooldown;
                controller.cooldown.reset();
                info!("Shield died: Active -> Cooldown");
            }
        }
    }
}


fn tick_shield_cooldown(
    time: Res<Time>,
    mut q: Query<&mut ShieldController, With<Spaceship>>,
) {
    let Ok(mut controller) = q.single_mut() else {
        return;
    };

    if controller.state == ShieldState::Cooldown {
        controller.cooldown.tick(time.delta());

        if controller.cooldown.is_finished() {
            controller.state = ShieldState::Ready;
            info!("Shield cooldown complete: Cooldown -> Ready");
        }
    }
}