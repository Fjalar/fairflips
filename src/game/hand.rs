use bevy::prelude::*;

use crate::game::gameplay_assets::GameplayAssets;

pub fn hand(
    player_assets: &GameplayAssets,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) -> impl Bundle {
    // A texture atlas is a way to split a single image into a grid of related images.
    // You can learn more in this example: https://github.com/bevyengine/bevy/blob/latest/examples/2d/texture_atlas.rs
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(64), 2, 1, Some(UVec2::splat(1)), None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    (
        Name::new("Hand"),
        Hand,
        Sprite::from_atlas_image(
            player_assets.hand_image.clone(),
            TextureAtlas {
                layout: texture_atlas_layout,
                index: 0,
            },
        ),
        Transform::from_scale(Vec2::splat(4.0).extend(1.0))
            .with_translation(Vec3::new(20.0, -175.0, 0.0))
            .with_rotation(Quat::from_rotation_z(0.1)),
    )
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub(crate) struct Hand;
