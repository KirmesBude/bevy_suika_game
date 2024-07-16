use crate::fruits::Fruit;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin)
            .add_systems(Startup, setup)
            .add_systems(Update, update_fps_text)
            .add_systems(Update, update_score_text);
    }
}

#[derive(Component)]
struct FpsText;

#[derive(Component)]
struct ScoreText;

fn setup(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_section(
            "FPS: ",
            TextStyle {
                font: default(),
                font_size: 20.0,
                color: Color::WHITE,
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Px(5.0),
            ..default()
        }),
        FpsText,
    ));

    commands.spawn((
        TextBundle::from_section(
            "Score: ",
            TextStyle {
                font: default(),
                font_size: 20.0,
                color: Color::WHITE,
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(20.0),
            left: Val::Px(5.0),

            ..default()
        }),
        ScoreText,
    ));
}

fn update_fps_text(diagnostics: Res<DiagnosticsStore>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                text.sections[0].value = format!("FPS: {value:.2}");
            }
        }
    }
}

fn update_score_text(fruits: Query<&Fruit>, mut query: Query<&mut Text, With<ScoreText>>) {
    let score: u32 = fruits.iter().map(|fruit| fruit.score()).sum();

    for mut text in &mut query {
        text.sections[0].value = format!("Score: {score}");
    }
}
