use bevy::prelude::*;

use crate::{
    asset_loader::SceneAssets, asteroids::Asteroid, health::Health, state::GameState
};

#[derive(Component, Debug)]
pub struct Score {
    pub value: i32,
}

impl Score {
    pub fn new(value: i32) -> Self {
        Self { value }
    }
}

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(PostStartup, spawn_score)
        .add_systems(OnEnter(GameState::GameOver), reset_score)
        .add_systems(Update, update_score);
    }
}

fn spawn_score(
    mut commands: Commands,
    scene_assets: Res<SceneAssets>,
) {
    commands.spawn((
        Text::new("Score: "),
        TextFont {
            // This font is loaded and will be used instead of the default font.
            font: scene_assets.font.clone(),
            font_size: 33.0,
            ..default()
        },
    ))
    .with_child((
        TextSpan::default(),
        (
            // "default_font" feature is unavailable, load a font to use instead.
            TextFont {
                font: scene_assets.font.clone(),
                font_size: 33.0,
                ..Default::default()
            },
        ),
        Score::new(0),
    ));
}

fn update_score(
    mut query: Query<(&mut TextSpan, &mut Score), With<Score>>,
    asteroid_query: Query<&Health, With<Asteroid>>,
) {
    let Ok((
        mut span,
        mut score
    )) = query.single_mut() else {
        return;
    };

    for health in asteroid_query.iter() {
        if health.value <= 0.0 {
            score.value += 1;
        }
    }

    **span = format!("{:}", score.value);
}

fn reset_score(
    mut query: Query<(&mut TextSpan, &mut Score), With<Score>>,
) {
    for (mut span, mut score) in &mut query {
        score.value = 0;
        **span = format!("{:}", score.value);
    }
}
