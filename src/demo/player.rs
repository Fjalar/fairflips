//! Player-specific behavior.

use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};

use crate::asset_tracking::LoadResource;

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<PlayerAssets>();
}

/// The hand.
pub fn hand(
    player_assets: &PlayerAssets,
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

/// The player character.
pub fn coin(
    player_assets: &PlayerAssets,
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
        // Name::new("Coin"),
        Coin {
            currently_flipping: false,
        },
        AnimationTimer(timer),
        Sprite::from_atlas_image(
            player_assets.coin_image.clone(),
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
pub(crate) struct Hand;

#[derive(Component, Deref, DerefMut)]
pub(crate) struct AnimationTimer(pub(crate) Timer);

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub(crate) struct Coin {
    pub(crate) currently_flipping: bool,
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct PlayerAssets {
    #[dependency]
    hand_image: Handle<Image>,
    #[dependency]
    coin_image: Handle<Image>,
    #[dependency]
    pub steps: Vec<Handle<AudioSource>>,
}

impl FromWorld for PlayerAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            hand_image: assets.load_with_settings(
                "images/hand.png",
                |settings: &mut ImageLoaderSettings| {
                    // Use `nearest` image sampling to preserve pixel art style.
                    settings.sampler = ImageSampler::nearest();
                },
            ),
            coin_image: assets.load_with_settings(
                "images/coin.png",
                |settings: &mut ImageLoaderSettings| {
                    // Use `nearest` image sampling to preserve pixel art style.
                    settings.sampler = ImageSampler::nearest();
                },
            ),
            steps: vec![
                assets.load("audio/sound_effects/step1.ogg"),
                assets.load("audio/sound_effects/step2.ogg"),
                assets.load("audio/sound_effects/step3.ogg"),
                assets.load("audio/sound_effects/step4.ogg"),
            ],
        }
    }
}
