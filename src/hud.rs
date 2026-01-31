use bevy::prelude::*;

use crate::{
    asset_loader::SceneAssets,
    health::Health,
    spaceship::{Shield, ShieldController, ShieldState, Spaceship},
    schedule::InGameSet,
};

const SHIP_HP_MAX: f32 = 100.0;   // keep in sync with SPACESHIP_HEALTH
const SHIELD_HP_MAX: f32 = 60.0;  // keep in sync with SHIELD_HP

#[derive(Component, Debug)]
pub struct ShipBarFill;

#[derive(Component, Debug)]
pub struct ShieldBarFill;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_hud_bars);
        app.add_systems(
            Update,
            update_hud_bars.in_set(InGameSet::EntityUpdates),
        );
    }
}

fn spawn_hud_bars(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    // Layout constants (tune later)
    let bar_w = 100.0;
    let bar_h = 6.0;
    let gap_y = 2.0;

    // Root container (top-left)
    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            left: Val::Px(130.0),
            top: Val::Px(16.0),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(gap_y),
            ..default()
        })
        .with_children(|root| {
            // Ship row
            root.spawn(Node {
                display: Display::Flex,
                align_items: AlignItems::Center,
                column_gap: Val::Px(8.0),
                ..default()
            })
            .with_children(|row| {
                row.spawn((
                    Text::new("SHIP"),
                    TextFont {
                        font: scene_assets.font.clone(),
                        font_size: 14.0,
                        ..default()
                    },
                ));
                // Bar frame
                row.spawn((
                    Node {
                        width: Val::Px(bar_w),
                        height: Val::Px(bar_h),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.15)),
                ))
                .with_children(|frame| {
                    // Fill (starts full)
                    frame.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            ..default()
                        },
                        BackgroundColor(Color::srgba(0.2, 1.0, 0.2, 0.9)), // green-ish
                        ShipBarFill,
                    ));
                });
            });

            // Shield row
            root.spawn(Node {
                display: Display::Flex,
                align_items: AlignItems::Center,
                column_gap: Val::Px(8.0),
                ..default()
            })
            .with_children(|row| {
                row.spawn((
                    Text::new("SHLD"),
                    TextFont {
                        font: scene_assets.font.clone(),
                        font_size: 14.0,
                        ..default()
                    },
                ));

                // Bar frame
                row.spawn((
                    Node {
                        width: Val::Px(bar_w),
                        height: Val::Px(bar_h),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.15)),
                ))
                .with_children(|frame| {
                    // Fill (starts full for now)
                    frame.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            ..default()
                        },
                        BackgroundColor(Color::srgba(0.3, 0.8, 1.0, 0.9)), // cyan/blue-ish
                        ShieldBarFill,
                    ));
                });
            });
        });
}


fn update_hud_bars(
    ship_q: Query<(Entity, &Health, &ShieldController), With<Spaceship>>,
    shield_q: Query<(&Health, &Shield)>,
    mut ship_fill_q: Query<&mut Node, (With<ShipBarFill>, Without<ShieldBarFill>)>,
    mut shield_fill_q: Query<
        (&mut Node, &mut BackgroundColor),
        (With<ShieldBarFill>, Without<ShipBarFill>),
    >,
) {
    // If ship is gone → empty both bars and bail.
    let Ok((ship_e, ship_health, controller)) = ship_q.single() else {
        if let Ok(mut ship_fill) = ship_fill_q.single_mut() {
            ship_fill.width = Val::Px(0.0);
        }
        if let Ok((mut shield_fill, _color)) = shield_fill_q.single_mut() {
            shield_fill.width = Val::Px(0.0);
        }
        return;
    };

    // --- Ship bar ---
    let Ok(mut ship_fill) = ship_fill_q.single_mut() else { return; };
    let t_ship = (ship_health.value / SHIP_HP_MAX).clamp(0.0, 1.0);
    ship_fill.width = Val::Percent(t_ship * 100.0);

    // --- Shield bar ---
    let Ok((mut shield_fill, mut shield_color)) = shield_fill_q.single_mut() else { return; };

    match controller.state {
        ShieldState::Active => {
            // find shield that belongs to this ship
            if let Some((shield_health, _)) = shield_q.iter().find(|(_, s)| s.ship == ship_e) {
                let t = (shield_health.value / SHIELD_HP_MAX).clamp(0.0, 1.0);
                shield_fill.width = Val::Percent(t * 100.0);
                *shield_color = BackgroundColor(Color::srgba(0.3, 0.8, 1.0, 0.9));
            } else {
                shield_fill.width = Val::Percent(0.0);
                *shield_color = BackgroundColor(Color::srgba(0.3, 0.8, 1.0, 0.9));
            }
        }
        ShieldState::Cooldown => {
            shield_fill.width = Val::Percent(0.0);
            *shield_color = BackgroundColor(Color::srgba(0.3, 0.8, 1.0, 0.35));
        }
        ShieldState::Ready => {
            shield_fill.width = Val::Percent(100.0);
            *shield_color = BackgroundColor(Color::srgba(0.3, 0.8, 1.0, 0.75));
        }
    }
}