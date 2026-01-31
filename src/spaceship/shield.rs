use bevy::prelude::*;
use bevy::pbr::{MeshMaterial3d, StandardMaterial};
use bevy::render::alpha::AlphaMode;
use bevy::platform::collections::HashMap;

use crate::{asset_loader::SceneAssets, collision_detection::Collider, health::Health, schedule::InGameSet};
use super::{ShieldController, ShieldRequest, ShieldState, Spaceship, SPACESHIP_RADIUS};


#[derive(Component, Debug)]
pub struct Shield {
    pub ship: Entity,
}

#[derive(Component, Debug, Default)]
pub struct ShieldMaterialCache {
    pub handles: Vec<Handle<StandardMaterial>>,
}

#[derive(Component, Debug)]
pub struct ShieldHitCooldown {
    pub timer: Timer,
}

#[derive(Message, Debug)]
pub struct ShieldReadyEvent {
    pub ship: Entity,
}

const SHIELD_RADIUS: f32 = SPACESHIP_RADIUS * 2.0;
const SHIELD_VISUAL_SCALE: f32 = SHIELD_RADIUS; // because model diameter is 2.0
const SHIELD_HP: f32 = 60.0;
const SHIELD_HIT_COOLDOWN_SECS: f32 = 0.40;
const SHIELD_DECAY: f32 = 4.0;  // HP per second.
const SHIELD_BASE_ALPHA: f32 = 0.35; // tune: 0.25–0.45 feels good
const SHIELD_MIN_ALPHA: f32 = 0.03;  // don’t go fully invisible until dead

pub fn register(app: &mut App) {
    app.add_message::<ShieldReadyEvent>()
    .add_systems(Update,
        (
            consume_shield_request,
            shield_follow_ship,
            tick_shield_hit_cooldowns,
            shield_cache_materials,
            shield_apply_alpha_from_health,
            shield_decay_health,
            shield_death_starts_cooldown,
            tick_shield_cooldown,
        )
        .in_set(InGameSet::UserInput)
    );
}

fn consume_shield_request(
    mut commands: Commands,
    scene_assets: Res<SceneAssets>,
    mut q: Query<(Entity, &mut ShieldController), (With<Spaceship>, With<ShieldRequest>)>,
) {
    let Ok((ship_entity, mut controller)) = q.single_mut() else { return; };

    let mut hit_cd = Timer::from_seconds(SHIELD_HIT_COOLDOWN_SECS, TimerMode::Once);
    hit_cd.set_elapsed(hit_cd.duration());  // start "ready to be hit"

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
    mut q: Query<(Entity, &mut ShieldController), With<Spaceship>>,
    mut shield_ready_writer: MessageWriter<ShieldReadyEvent>,
) {
    let Ok((ship_e, mut controller)) = q.single_mut() else { return; };

    if controller.state != ShieldState::Cooldown {
        return;
    }

    controller.cooldown.tick(time.delta());

    if controller.cooldown.just_finished() {
        controller.state = ShieldState::Ready;
        info!("Shield cooldown complete: Cooldown -> Ready (ship={:?}, elapsed={:?})",
              ship_e, controller.cooldown.elapsed());
        shield_ready_writer.write(ShieldReadyEvent { ship: ship_e });
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

fn shield_cache_materials(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    shields: Query<Entity, (With<Shield>, Without<ShieldMaterialCache>)>,
    // entities spawned by glTF that carry the material
    mut mat_entities: Query<(Entity, &mut MeshMaterial3d<StandardMaterial>)>,
    child_of_q: Query<(Entity, &ChildOf)>,
) {
    for shield_entity in shields.iter() {
        // Build child -> parent map once
        let mut parent_map: HashMap<Entity, Entity> = HashMap::new();
        for (child_e, child_of) in child_of_q.iter() {
            parent_map.insert(child_e, child_of.parent());
        }

        let is_descendant_of_shield = |mut e: Entity| -> bool {
            while let Some(p) = parent_map.get(&e).copied() {
                if p == shield_entity {
                    return true;
                }
                e = p;
            }
            false
        };

        let mut cloned_handles: Vec<Handle<StandardMaterial>> = Vec::new();

        for (e, mut mat3d) in mat_entities.iter_mut() {
            if !is_descendant_of_shield(e) {
                continue;
            }

            // Clone the material so we don't affect other glTF instances.
            let Some(orig) = materials.get(mat3d.0.id()).cloned() else {
                continue;
            };

            let mut new_mat = orig;
            new_mat.alpha_mode = AlphaMode::Blend;

            // Set initial alpha (full health)
            let base = new_mat.base_color.to_srgba();
            new_mat.base_color = Color::srgba(base.red, base.green, base.blue, SHIELD_BASE_ALPHA);

            let new_handle = materials.add(new_mat);
            mat3d.0 = new_handle.clone();
            cloned_handles.push(new_handle);
        }

        commands
            .entity(shield_entity)
            .insert(ShieldMaterialCache { handles: cloned_handles });
    }
}

fn shield_apply_alpha_from_health(
    mut materials: ResMut<Assets<StandardMaterial>>,
    q: Query<(&Health, &ShieldMaterialCache), With<Shield>>,
) {
    for (health, cache) in q.iter() {
        let t = (health.value / SHIELD_HP).clamp(0.0, 1.0);
        let alpha = (SHIELD_MIN_ALPHA + (SHIELD_BASE_ALPHA - SHIELD_MIN_ALPHA) * t).clamp(0.0, 1.0);

        for h in cache.handles.iter() {
            if let Some(mat) = materials.get_mut(h.id()) {
                let base = mat.base_color.to_srgba();
                mat.base_color = Color::srgba(base.red, base.green, base.blue, alpha);
                // keep blend (some materials may get overwritten by glTF defaults)
                mat.alpha_mode = AlphaMode::Blend;
            }
        }
    }
}

fn shield_decay_health(
    mut shield_q: Query<&mut Health, With<Shield>>,
    time: Res<Time>,
) {
    // In the interest of fairness, we would like the shield to not be
    // alive forever even if it doesn't get hit.  So we will make its health
    // decay over time.  We would like the shield to last about 10-15 seconds.
    for mut health in shield_q.iter_mut() {
        let decay_value = SHIELD_DECAY * time.delta_secs();
        if health.value > 0.0 {
            health.value -= health.value.min(decay_value);
        }
    }
}
