#![allow(clippy::unnecessary_cast)]

use asset_loading::{AssetLoadingPlugin, FruitAssets};
use bevy::{input::mouse::MouseButtonInput, prelude::*};
use bevy_xpbd_2d::{math::*, prelude::*};

use std::time::Duration;

use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};

use bevy_prng::WyRand;
use bevy_rand::prelude::*;
use rand_core::RngCore;

use crate::fruits::Fruit;

mod asset_loading;
mod fruits;

#[derive(Default)]
pub struct XpbdExamplePlugin;

impl Plugin for XpbdExamplePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            PhysicsPlugins::default(),
            FrameTimeDiagnosticsPlugin,
            AssetLoadingPlugin,
            EntropyPlugin::<WyRand>::default(),
        ))
        .add_systems(Startup, setup_plugin)
        .add_systems(
            OnEnter(AppState::Paused),
            |mut time: ResMut<Time<Physics>>| time.pause(),
        )
        .add_systems(
            OnExit(AppState::Paused),
            |mut time: ResMut<Time<Physics>>| time.unpause(),
        )
        .add_systems(Update, update_fps_text)
        .add_systems(Update, pause_button)
        .add_systems(Update, step_button.run_if(in_state(AppState::Paused)))
        .add_systems(
            Update,
            (spawn_ball_at_cursor_x, merge_fruit).run_if(in_state(AppState::Running)),
        );
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
pub enum AppState {
    #[default]
    Loading,
    Paused,
    Running,
}

fn pause_button(
    current_state: ResMut<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::KeyP) {
        let new_state = match current_state.get() {
            AppState::Paused => AppState::Running,
            AppState::Running => AppState::Paused,
            AppState::Loading => AppState::Loading,
        };
        next_state.set(new_state);
    }
}

fn step_button(mut time: ResMut<Time<Physics>>, keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::Enter) {
        time.advance_by(Duration::from_secs_f64(1.0 / 60.0));
    }
}

#[derive(Component)]
struct FpsText;

fn setup_plugin(mut commands: Commands) {
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
}

fn update_fps_text(diagnostics: Res<DiagnosticsStore>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                // Update the value of the second section
                text.sections[0].value = format!("FPS: {value:.2}");
            }
        }
    }
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, XpbdExamplePlugin))
        .insert_resource(ClearColor(Color::rgb(0.05, 0.05, 0.1)))
        .insert_resource(SubstepCount(6))
        .insert_resource(Gravity(Vector::NEG_Y * 1000.0))
        .add_systems(Startup, setup)
        .run();
}

#[derive(Component)]
struct Marble;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    let square_sprite = Sprite {
        color: Color::rgb(0.7, 0.7, 0.8),
        custom_size: Some(Vec2::splat(50.0)),
        ..default()
    };

    // Floor
    commands.spawn((
        SpriteBundle {
            sprite: square_sprite.clone(),
            transform: Transform::from_xyz(0.0, -50.0 * 5.0, 0.0)
                .with_scale(Vec3::new(9.0, 1.0, 1.0)),
            ..default()
        },
        RigidBody::Static,
        Collider::cuboid(50.0, 50.0),
    ));
    // Left wall
    commands.spawn((
        SpriteBundle {
            sprite: square_sprite.clone(),
            transform: Transform::from_xyz(-200.0, 0.0, 0.0).with_scale(Vec3::new(1.0, 9.0, 1.0)),
            ..default()
        },
        RigidBody::Static,
        Collider::cuboid(50.0, 50.0),
    ));
    // Right wall
    commands.spawn((
        SpriteBundle {
            sprite: square_sprite,
            transform: Transform::from_xyz(200.0, 0.0, 0.0).with_scale(Vec3::new(1.0, 9.0, 1.0)),
            ..default()
        },
        RigidBody::Static,
        Collider::cuboid(50.0, 50.0),
    ));
}

fn spawn_ball_at_cursor_x(
    mut commands: Commands,
    mut mousebtn_evr: EventReader<MouseButtonInput>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    fruit_assets: Res<FruitAssets>,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
) {
    use bevy::input::ButtonState;

    for ev in mousebtn_evr.read() {
        if ev.button == MouseButton::Left && ev.state == ButtonState::Pressed {
            let window = windows.get(ev.window).unwrap();
            let (camera, camera_transform) = cameras.get_single().unwrap();

            if let Some(world_position) = window
                .cursor_position()
                .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
                .map(|ray| ray.origin.truncate())
            {
                eprintln!("World coords: {}/{}", world_position.x, world_position.y);
                let transform =
                    Transform::from_xyz(world_position.x, 280.0, 0.0).with_scale(Vec3::splat(4.0));

                let index = rng.next_u32() as usize;
                commands.spawn(Fruit::from_index(index).bundle(&fruit_assets, transform));
            }
        }
    }
}

fn merge_fruit(
    mut commands: Commands,
    fruits: Query<(Entity, &GlobalTransform, &Fruit)>,
    mut collision_event_reader: EventReader<Collision>,
    fruit_assets: Res<FruitAssets>,
) {
    for Collision(contacts) in collision_event_reader.read() {
        if let (Ok(fruit_a), Ok(fruit_b)) =
            (fruits.get(contacts.entity1), fruits.get(contacts.entity2))
        {
            if fruit_a.2 == fruit_b.2 {
                if let Some(next) = fruit_a.2.next() {
                    commands.get_entity(fruit_a.0).map(|mut e| {
                        e.despawn();
                        Some(())
                    });
                    commands.get_entity(fruit_b.0).map(|mut e| {
                        e.despawn();
                        Some(())
                    });
                    let translation = (fruit_a.1.translation() + fruit_b.1.translation()) / 2.0;
                    let transform =
                        Transform::from_translation(translation).with_scale(Vec3::splat(4.0));

                    commands.spawn(next.bundle(&fruit_assets, transform));
                }
            }
        }
    }
}
