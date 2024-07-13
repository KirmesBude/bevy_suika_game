#![allow(clippy::unnecessary_cast)]

use asset_loading::{AssetLoadingPlugin, FruitAssets, UiAssets};
use avian2d::{math::Vector, prelude::*};
use bevy::{input::mouse::MouseButtonInput, prelude::*, window::PrimaryWindow};

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
            PhysicsPlugins::default().with_length_unit(10.0),
            FrameTimeDiagnosticsPlugin,
            AssetLoadingPlugin,
            EntropyPlugin::<WyRand>::default(),
        ))
        .init_resource::<NextFruit>()
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
        .add_systems(Update, update_score_text)
        .add_systems(Update, pause_button)
        .add_systems(Update, step_button.run_if(in_state(AppState::Paused)))
        .add_systems(
            Update,
            (
                cloud_to_mouse_x,
                spawn_ball_at_cloud,
                remove_new_fruit,
                game_over,
                merge_fruit,
                show_next_fruit,
            )
                .run_if(in_state(AppState::Running)),
        )
        .add_systems(OnEnter(AppState::Running), (spawn_cloud, spawn_next_fruit));
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

#[derive(Component)]
struct ScoreText;

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

#[derive(Component)]
struct NewFruit;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, XpbdExamplePlugin))
        .insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.1)))
        .insert_resource(SubstepCount(6))
        .insert_resource(Gravity(Vector::NEG_Y * 1000.0))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default()).insert(MainCamera);

    let square_sprite = Sprite {
        color: Color::srgb(0.7, 0.7, 0.8),
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
        Collider::rectangle(50.0, 50.0),
    ));
    // Left wall
    commands.spawn((
        SpriteBundle {
            sprite: square_sprite.clone(),
            transform: Transform::from_xyz(-200.0, 0.0, 0.0).with_scale(Vec3::new(1.0, 9.0, 1.0)),
            ..default()
        },
        RigidBody::Static,
        Collider::rectangle(50.0, 50.0),
    ));
    // Right wall
    commands.spawn((
        SpriteBundle {
            sprite: square_sprite,
            transform: Transform::from_xyz(200.0, 0.0, 0.0).with_scale(Vec3::new(1.0, 9.0, 1.0)),
            ..default()
        },
        RigidBody::Static,
        Collider::rectangle(50.0, 50.0),
    ));
}

fn spawn_cloud(mut commands: Commands, ui_assets: Res<UiAssets>) {
    commands
        .spawn(SpriteBundle {
            transform: Transform::from_xyz(0.0, 280.0, 0.0).with_scale(Vec3::splat(4.0)),
            texture: ui_assets.texture.clone_weak(),
            ..Default::default()
        })
        .insert(ui_assets.cloud())
        .insert(Cloud);
}

fn spawn_ball_at_cloud(
    mut commands: Commands,
    mut mousebtn_evr: EventReader<MouseButtonInput>,
    q_cloud: Query<&GlobalTransform, With<Cloud>>,
    fruit_assets: Res<FruitAssets>,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
    new_fruits: Query<(), With<NewFruit>>,
    mut next_fruit: ResMut<NextFruit>,
) {
    use bevy::input::ButtonState;

    if !new_fruits.is_empty() {
        return;
    }

    for ev in mousebtn_evr.read() {
        if ev.button == MouseButton::Left && ev.state == ButtonState::Pressed {
            let cloud_transform = q_cloud.single();
            let transform = Transform::from_xyz(cloud_transform.translation().x, 280.0, 0.0)
                .with_scale(Vec3::splat(4.0));

            commands
                .spawn(next_fruit.0.bundle(&fruit_assets, transform))
                .insert(NewFruit);

            // Compute new fruit
            let index = rng.next_u32() as usize;
            next_fruit.0 = Fruit::from_index(index % 5);
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

fn remove_new_fruit(
    mut commands: Commands,
    fruits: Query<Entity, (With<Fruit>, With<NewFruit>)>,
    mut collision_event_reader: EventReader<Collision>,
) {
    for Collision(contacts) in collision_event_reader.read() {
        let new_fruit = match (fruits.get(contacts.entity1), fruits.get(contacts.entity2)) {
            (Ok(fruit), Err(_)) => fruit,
            (Err(_), Ok(fruit)) => fruit,
            _ => continue,
        };

        commands.entity(new_fruit).remove::<NewFruit>();
    }
}

fn game_over(fruits: Query<&GlobalTransform, (With<Fruit>, Without<NewFruit>)>) {
    if fruits
        .iter()
        .any(|transform| transform.translation().y > 250.0)
    {
        println!("Game Over");
    }
}

#[derive(Component)]
struct Cloud;

#[derive(Component)]
struct MainCamera;

fn cloud_to_mouse_x(
    mut q_cloud: Query<&mut Transform, With<Cloud>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so Query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // There is only one primary window, so we can similarly get it from the query:
    let window = q_window.single();

    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        let mut transform = q_cloud.single_mut();
        transform.translation.x = world_position.x.clamp(-130.0, 130.0);
    }
}

#[derive(Resource, Default)]
struct NextFruit(Fruit);

#[derive(Component)]
struct NextFruitUi;

fn spawn_next_fruit(
    mut commands: Commands,
    fruit_assets: Res<FruitAssets>,
    ui_assets: Res<UiAssets>,
) {
    let transform = Transform::from_xyz(420.0, -200.0, 0.0).with_scale(Vec3::splat(4.0));
    commands
        .spawn(SpriteBundle {
            transform,
            texture: fruit_assets.texture.clone_weak(),
            ..Default::default()
        })
        .insert(fruit_assets.texture_atlas(&Fruit::default()))
        .insert(NextFruitUi);

    commands
        .spawn(SpriteBundle {
            transform,
            texture: ui_assets.texture.clone_weak(),
            ..Default::default()
        })
        .insert(ui_assets.bubble());
}

fn show_next_fruit(
    mut q_next_fruit_ui: Query<&mut TextureAtlas, With<NextFruitUi>>,
    next_fruit: Res<NextFruit>,
) {
    if next_fruit.is_changed() {
        let mut texture_atlas = q_next_fruit_ui.single_mut();
        texture_atlas.index = FruitAssets::index(&next_fruit.0);
    }
}
