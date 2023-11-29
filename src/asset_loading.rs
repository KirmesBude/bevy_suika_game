use bevy::prelude::*;
use bevy_asset_loader::{
    asset_collection::AssetCollection,
    loading_state::{LoadingState, LoadingStateAppExt},
};
use bevy_titan::SpriteSheetLoaderPlugin;
use bevy_xpbd_2d::{components::Collider, math::Scalar};

use crate::{fruits::Fruit, AppState};

pub struct AssetLoadingPlugin;

impl Plugin for AssetLoadingPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(SpriteSheetLoaderPlugin)
            .add_state::<AppState>()
            .add_loading_state(
                LoadingState::new(AppState::Loading).continue_to_state(AppState::Running),
            )
            .add_collection_to_loading_state::<_, FruitAssets>(AppState::Loading);
    }
}

#[derive(Debug, AssetCollection, Resource)]
pub struct FruitAssets {
    #[asset(path = "fruit.titan")]
    pub texture_atlas: Handle<TextureAtlas>,
}

impl FruitAssets {
    fn radius(fruit: &Fruit) -> f32 {
        match fruit {
            Fruit::Cherry => 4.0,
            Fruit::Strawberry => 6.0,
            Fruit::Grapes => 8.0,
            Fruit::Dekopon => 10.0,
            Fruit::Persimmon => 12.0,
            Fruit::Apple => 14.0,
            Fruit::Pear => 16.0,
            Fruit::Peach => 18.0,
            Fruit::Pineapple => 20.0,
            Fruit::Melon => 22.0,
            Fruit::Watermelon => 24.0,
        }
    }

    pub fn collider(fruit: &Fruit) -> Collider {
        Collider::ball(Self::radius(fruit) as Scalar)
    }

    pub fn texture_atlas_sprite(fruit: &Fruit) -> TextureAtlasSprite {
        let index = Self::index(fruit);

        TextureAtlasSprite::new(index)
    }

    fn index(fruit: &Fruit) -> usize {
        match fruit {
            Fruit::Cherry => 0,
            Fruit::Strawberry => 1,
            Fruit::Grapes => 2,
            Fruit::Dekopon => 3,
            Fruit::Persimmon => 4,
            Fruit::Apple => 5,
            Fruit::Pear => 6,
            Fruit::Peach => 7,
            Fruit::Pineapple => 8,
            Fruit::Melon => 9,
            Fruit::Watermelon => 10,
        }
    }
}
