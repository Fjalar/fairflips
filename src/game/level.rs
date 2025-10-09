//! Spawn the main level.

use bevy::prelude::*;

use crate::{
    game::{
        coin,
        counters::{CoinCounter, counter_ui},
        gameplay_assets::GameplayAssets,
        hand,
    },
    screens::Screen,
};

/// A system that spawns the main level.
pub fn spawn_level(
    mut commands: Commands,
    gameplay_assets: Res<GameplayAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    counter: Res<CoinCounter>,
) {
    commands.spawn((
        Name::new("Level"),
        Transform::default(),
        Visibility::default(),
        DespawnOnExit(Screen::Gameplay),
        children![
            counter_ui(counter),
            hand::hand(&gameplay_assets, &mut texture_atlas_layouts),
            coin::coin(&gameplay_assets, &mut texture_atlas_layouts),
        ],
    ));
}
