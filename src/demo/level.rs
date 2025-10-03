//! Spawn the main level.

use bevy::prelude::*;

use crate::{
    demo::{
        counters::{CoinCounter, counter_ui},
        player::{PlayerAssets, coin, hand},
    },
    screens::Screen,
};

/// A system that spawns the main level.
pub fn spawn_level(
    mut commands: Commands,
    player_assets: Res<PlayerAssets>,
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
            hand(&player_assets, &mut texture_atlas_layouts),
            coin(&player_assets, &mut texture_atlas_layouts),
        ],
    ));
}
