use asset_loading::AssetLoadingPlugin;
use avian2d::{math::Vector, prelude::*};
use bevy::prelude::*;
use game::GamePlugin;
use ui::UiPlugin;

mod asset_loading;
mod fruits;
mod game;
mod ui;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, UiPlugin, AssetLoadingPlugin, GamePlugin))
        .insert_resource(ClearColor(Color::srgb(0.5, 0.5, 0.5)))
        .insert_resource(Gravity(Vector::NEG_Y * 1000.0))
        .add_systems(Startup, setup)
        .run();
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
pub enum AppState {
    #[default]
    Loading,
    Paused,
    Running,
}

#[derive(Component)]
struct MainCamera;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default()).insert(MainCamera);
}
