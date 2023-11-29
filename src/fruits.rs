use bevy::prelude::*;
use bevy_xpbd_2d::components::RigidBody;

use crate::asset_loading::FruitAssets;

#[derive(Debug, Component, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
pub enum Fruit {
    Cherry,
    Strawberry,
    Grapes,
    Dekopon,
    Persimmon,
    Apple,
    Pear,
    Peach,
    Pineapple,
    Melon,
    Watermelon,
}

impl Fruit {
    pub fn bundle(self, fruit_assets: &FruitAssets, transform: Transform) -> impl Bundle {
        (
            SpriteSheetBundle {
                sprite: FruitAssets::texture_atlas_sprite(&self),
                texture_atlas: fruit_assets.texture_atlas.clone_weak(),
                transform,
                ..Default::default()
            },
            RigidBody::Dynamic,
            FruitAssets::collider(&self),
            self,
        )
    }

    pub fn next(self) -> Option<Self> {
        match self {
            Fruit::Cherry => Some(Fruit::Strawberry),
            Fruit::Strawberry => Some(Fruit::Grapes),
            Fruit::Grapes => Some(Fruit::Dekopon),
            Fruit::Dekopon => Some(Fruit::Persimmon),
            Fruit::Persimmon => Some(Fruit::Apple),
            Fruit::Apple => Some(Fruit::Pear),
            Fruit::Pear => Some(Fruit::Peach),
            Fruit::Peach => Some(Fruit::Pineapple),
            Fruit::Pineapple => Some(Fruit::Melon),
            Fruit::Melon => Some(Fruit::Watermelon),
            Fruit::Watermelon => None,
        }
    }
}
