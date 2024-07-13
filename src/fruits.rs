use avian2d::dynamics::rigid_body::RigidBody;
use bevy::prelude::*;
use strum::EnumCount;
use strum_macros::EnumCount;

use crate::asset_loading::FruitAssets;

#[derive(Default, Debug, Component, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, EnumCount)]
pub enum Fruit {
    #[default]
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
            SpriteBundle {
                texture: fruit_assets.texture.clone_weak(),
                transform,
                ..Default::default()
            },
            fruit_assets.texture_atlas(&self),
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

    pub fn from_index(index: usize) -> Self {
        let index = index % Self::COUNT;

        match index {
            0 => Fruit::Cherry,
            1 => Fruit::Strawberry,
            2 => Fruit::Grapes,
            3 => Fruit::Dekopon,
            4 => Fruit::Persimmon,
            5 => Fruit::Apple,
            6 => Fruit::Pear,
            7 => Fruit::Peach,
            8 => Fruit::Pineapple,
            9 => Fruit::Melon,
            10 => Fruit::Watermelon,
            _ => panic!("Unexpected Fruit from index"),
        }
    }

    pub fn score(self) -> u32 {
        match self {
            Fruit::Cherry => 1,
            Fruit::Strawberry => 3,
            Fruit::Grapes => 6,
            Fruit::Dekopon => 10,
            Fruit::Persimmon => 15,
            Fruit::Apple => 21,
            Fruit::Pear => 28,
            Fruit::Peach => 36,
            Fruit::Pineapple => 45,
            Fruit::Melon => 55,
            Fruit::Watermelon => 66,
        }
    }
}
