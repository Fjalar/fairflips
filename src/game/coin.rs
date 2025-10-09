use bevy::prelude::*;

use crate::game::{animation::AnimationTimer, gameplay_assets::GameplayAssets};

pub fn coin(
    gameplay_assets: &GameplayAssets,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) -> impl Bundle {
    // A texture atlas is a way to split a single image into a grid of related images.
    // You can learn more in this example: https://github.com/bevyengine/bevy/blob/latest/examples/2d/texture_atlas.rs
    let layout =
        TextureAtlasLayout::from_grid(UVec2::splat(128), 2, 1, Some(UVec2::splat(1)), None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let mut timer = Timer::from_seconds(1.0, TimerMode::Once);

    timer.pause();

    (
        Name::new("Coin"),
        Coin {
            currently_flipping: false,
        },
        AnimationTimer(timer),
        Sprite::from_atlas_image(
            gameplay_assets.coin_image.clone(),
            TextureAtlas {
                layout: texture_atlas_layout,
                index: 0,
            },
        ),
        Transform::from_scale(Vec2::splat(1.0).extend(1.0)),
    )
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub(crate) struct Coin {
    pub(crate) currently_flipping: bool,
}
