use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct ShipBarFill;

#[derive(Component, Debug)]
pub struct ShieldBarFill;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_hud_bars);
    }
}

fn spawn_hud_bars(mut commands: Commands) {
    // Layout constants (tune later)
    let bar_w = 120.0;
    let bar_h = 6.0;
    let gap_y = 2.0;

    // Root container (top-left)
    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            left: Val::Px(200.0),
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
                row.spawn(Text::new("SHIP"));

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
                row.spawn(Text::new("SHLD"));

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