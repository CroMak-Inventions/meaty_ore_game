use bevy::prelude::*;

use crate::{asset_loader::SceneAssets, collision_detection::Collider, health::Health, schedule::InGameSet};
use super::{ShieldController, ShieldRequest, ShieldState, Spaceship, SPACESHIP_RADIUS};


#[derive(Component, Debug)]
pub struct Shield {
    pub ship: Entity,
}

#[derive(Component, Debug)]
pub struct ShieldHitCooldown {
    pub timer: Timer,
}

const SHIELD_HIT_COOLDOWN_SECS: f32 = 0.20;

const SHIELD_RADIUS: f32 = SPACESHIP_RADIUS * 1.35;
const SHIELD_VISUAL_SCALE: f32 = SHIELD_RADIUS; // because model diameter is 2.0
const SHIELD_HP: f32 = 60.0;

pub fn register(app: &mut App) {
    app.add_systems(
        Update,
        (
            consume_shield_request,
            shield_follow_ship,
            tick_shield_hit_cooldowns,
            shield_death_starts_cooldown,
            reconcile_shield_controller,
            tick_shield_cooldown,
        )
        .in_set(InGameSet::EntityUpdates),
    );
}

fn consume_shield_request(
    mut commands: Commands,
    scene_assets: Res<SceneAssets>,
    mut q: Query<(Entity, &mut ShieldController), (With<Spaceship>, With<ShieldRequest>)>,
) {
    let Ok((ship_entity, mut controller)) = q.single_mut() else { return; };

    let mut hit_cd = Timer::from_seconds(SHIELD_HIT_COOLDOWN_SECS, TimerMode::Once);
    hit_cd.set_elapsed(hit_cd.duration()); // start "ready to be hit"

    match controller.state {
        ShieldState::Ready => {
            commands.spawn((
                Shield { ship: ship_entity },
                ShieldHitCooldown { timer: hit_cd },
                Health::new(SHIELD_HP),
                Collider::new(SHIELD_RADIUS),
                SceneRoot(scene_assets.shield.clone()),
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
        shield_tf.scale = Vec3::ONE * SHIELD_VISUAL_SCALE;
    }
}

fn shield_death_starts_cooldown(
    mut commands: Commands,
    shield_q: Query<(Entity, &Health, &Shield)>,
    mut ship_q: Query<&mut ShieldController, With<Spaceship>>,
) {
    for (shield_entity, health, shield) in shield_q.iter() {
        if health.value <= 0.0 {
            info!("Shield died: entity={:?}", shield_entity);
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

fn tick_shield_hit_cooldowns(
    time: Res<Time>,
    mut q: Query<&mut ShieldHitCooldown>,
) {
    for mut cd in q.iter_mut() {
        cd.timer.tick(time.delta());
    }
}

fn reconcile_shield_controller(
    mut ship_q: Query<(Entity, &mut ShieldController), With<Spaceship>>,
    shield_q: Query<&Shield>,
) {
    let Ok((ship_e, mut controller)) = ship_q.single_mut() else { return; };

    let shield_exists = shield_q.iter().any(|s| s.ship == ship_e);

    if controller.state == ShieldState::Active && !shield_exists {
        controller.state = ShieldState::Cooldown;
        controller.cooldown.reset();
        info!("Shield missing while Active -> forcing Cooldown");
    }
}