use bevy::{
    prelude::*,
    app::AppExit,
    audio::Volume,
};

use crate::{
    ambient_sound::ThrusterSound,
    asset_loader::SceneAssets,
    state::GameState
};

#[derive(Component, Debug)]
pub struct GameOverDlg;


pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(PostStartup, spawn_game_over_dlg)
        .add_systems(OnEnter(GameState::GameOver), (
            show_game_over_dlg,
            mute_thruster_sound,
        ))
        .add_systems(OnEnter(GameState::StartGame),
            hide_game_over_dlg
        )
        .add_systems(Update,
            hit_any_key.run_if(in_state(GameState::GameOver)),
        )
        .add_systems(OnEnter(GameState::QuitGame),
        quit_game
        );
    }
}

fn spawn_game_over_dlg(
    mut commands: Commands,
    scene_assets: Res<SceneAssets>,
) {
    commands.spawn((
        Node {
            left: Val::Px(0.0),
            top: Val::Percent(-100.0),
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        BackgroundColor(Color::linear_rgba(0.5, 0.5, 0.5, 0.1)),
        GameOverDlg,
    ))
    .with_children(|builder| {
        builder.spawn((
            Node {
                padding: UiRect::axes(Val::Px(5.), Val::Px(1.)),
                ..default()
            },
            //BackgroundColor(Color::linear_rgb(0.5, 0.5, 0.5)),
        ))
        .with_child((
            Text::new("Game Over!"),
            // "default_font" feature is unavailable, load a font to use instead.
            TextFont { 
                font: scene_assets.font.clone(),
                font_size: 44.0,
                ..Default::default()
            },
        ));

        builder.spawn((
            Node {
                padding: UiRect::axes(Val::Px(5.), Val::Px(1.)),
                ..default()
            },
            //BackgroundColor(Color::linear_rgb(0.5, 0.5, 0.5)),
        ))
        .with_child((
            Text::new("Press <Enter> to start new game."),
            TextFont { 
                font: scene_assets.font.clone(),
                font_size: 22.0,
                ..Default::default()
            },
        ));

        builder.spawn((
            Node {
                padding: UiRect::axes(Val::Px(5.), Val::Px(1.)),
                ..default()
            },
            //BackgroundColor(Color::linear_rgb(0.5, 0.5, 0.5)),
        ))
        .with_child((
            Text::new("Press Q to quit."),
            TextFont { 
                font: scene_assets.font.clone(),
                font_size: 22.0,
                ..Default::default()
            },
        ));

    });
}

fn mute_thruster_sound(
    mut thruster_audio: Query<&mut AudioSink, With<ThrusterSound>>,
) {
    // Quite often the player is trying to dodge an asteroid and gets killed.
    // So the thruster volume gets stuck in the "on" position
    let Ok(mut sink) = thruster_audio.single_mut() else {
        return;
    };

    sink.set_volume(Volume::Linear(0.0));
}

fn show_game_over_dlg(
    mut game_over_dlg: Single<&mut Node, With<GameOverDlg>>,
) {
    game_over_dlg.top = Val::Percent(0.0);
}

fn hide_game_over_dlg(
    mut game_over_dlg: Single<&mut Node, With<GameOverDlg>>,
) {
    game_over_dlg.top = Val::Percent(-100.0);
}

fn hit_any_key(
    mut game_state: ResMut<NextState<GameState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.pressed(KeyCode::Enter) {
        game_state.set(GameState::StartGame);
    }
    else if keyboard_input.pressed(KeyCode::KeyQ) {
        game_state.set(GameState::QuitGame);
    }
}

fn quit_game(
    mut app_exit_events: ResMut<Messages<AppExit>>
) {
    app_exit_events.write(AppExit::Success);
}
