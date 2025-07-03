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


#[derive(Component, Debug)]
pub struct LastScore {
    pub value: i32,
}

impl LastScore {
    pub fn new(value: i32) -> Self {
        Self { value }
    }
}



#[derive(Component, Debug)]
pub struct HighScore {
    pub value: i32,
}

impl HighScore {
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
        Node {
            position_type: PositionType::Absolute,
            top: Val::Percent(2.0),
            left: Val::Percent(2.0),
            width: Val::Percent(20.),
            height: Val::Percent(5.),
            display: Display::Flex,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        //BackgroundColor(Color::linear_rgba(0.9843137, 0.44313726, 0.52156866, 0.1)),
        Text::new("Score: "),
        TextFont {
            // This font is loaded and will be used instead of the default font.
            font: scene_assets.font.clone(),
            font_size: 22.0,
            ..default()
        },
    ))
    .with_child((
        TextSpan::default(),
        (
            // "default_font" feature is unavailable, load a font to use instead.
            TextFont {
                font: scene_assets.font.clone(),
                font_size: 22.0,
                ..Default::default()
            },
        ),
        Score::new(0),
    ));

    commands.spawn((
        // We could also use a `UiTargetCamera` component here instead of the general `IsDefaultUiCamera`.
        Node {
            position_type: PositionType::Absolute,
            top: Val::Percent(2.0),
            left: Val::Percent(40.0),
            width: Val::Percent(24.0),
            height: Val::Percent(5.0),
            display: Display::Flex,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        //BackgroundColor(Color::linear_rgba(0.9843137, 0.44313726, 0.52156866, 0.1)),
        Text::new("High Score: "),
        TextFont {
            font: scene_assets.font.clone(),
            font_size: 33.0,
            ..Default::default()
        },
    ))
    .with_child((
        TextSpan::new("0"),
        (
            // "default_font" feature is unavailable, load a font to use instead.
            TextFont { 
                font: scene_assets.font.clone(),
                font_size: 33.0,
                ..Default::default()
            },
        ),
        HighScore::new(0),
    ));

    commands.spawn((
        // We could also use a `UiTargetCamera` component here instead of the general `IsDefaultUiCamera`.
        Node {
            position_type: PositionType::Absolute,
            top: Val::Percent(2.0),
            right: Val::Percent(2.0),
            width: Val::Percent(20.0),
            height: Val::Percent(5.0),
            display: Display::Flex,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        //BackgroundColor(Color::linear_rgba(0.9843137, 0.44313726, 0.52156866, 0.1)),
        Text::new("Last Score: "),
        TextFont {
            font: scene_assets.font.clone(),
            font_size: 22.0,
            ..Default::default()
        },
    ))
    .with_child((
        TextSpan::new("0"),
        (
            // "default_font" feature is unavailable, load a font to use instead.
            TextFont { 
                font: scene_assets.font.clone(),
                font_size: 22.0,
                ..Default::default()
            },
        ),
        LastScore::new(0),
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
    mut score_query: Query<(&mut TextSpan, &mut Score), With<Score>>,
    mut last_score_query: Query<(&mut TextSpan, &mut LastScore), (With<LastScore>, Without<Score>)>,
    mut high_score_query: Query<(&mut TextSpan, &mut HighScore), (With<HighScore>, Without<Score>, Without<LastScore>)>,
) {
    let Ok((mut score_span, mut score)) = score_query.single_mut() else {
        return;
    };

    let Ok((mut last_score_span, mut last_score)) = last_score_query.single_mut() else {
        return;
    };

    // always set the last score
    last_score.value = score.value;
    **last_score_span = format!("{:}", last_score.value);

    let Ok((mut high_score_span, mut high_score)) = high_score_query.single_mut() else {
        return;
    };

    // set the high score if we beat it
    if score.value > high_score.value {
        high_score.value = score.value;
        **high_score_span = format!("{:}", high_score.value);
    }
    
    // always zero out the current score
    score.value = 0;
    **score_span = format!("{:}", score.value);
}
