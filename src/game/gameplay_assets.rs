use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};

use crate::asset_tracking::LoadResource;

pub fn plugin(app: &mut App) {
    app.load_resource::<GameplayAssets>();
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct GameplayAssets {
    #[dependency]
    pub hand_image: Handle<Image>,
    #[dependency]
    pub coins: Vec<Handle<Image>>,
    #[dependency]
    pub coins_atlas_layout: Handle<TextureAtlasLayout>,
    #[dependency]
    pub flips: Vec<Handle<AudioSource>>,
    #[dependency]
    pub heads: Handle<AudioSource>,
}

impl FromWorld for GameplayAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();

        let layout =
            TextureAtlasLayout::from_grid(UVec2::splat(128), 2, 1, Some(UVec2::splat(1)), None);
        let texture_atlas_layout = assets.add(layout);

        Self {
            hand_image: assets.load_with_settings(
                "images/hand.png",
                |settings: &mut ImageLoaderSettings| {
                    // Use `nearest` image sampling to preserve pixel art style.
                    settings.sampler = ImageSampler::nearest();
                },
            ),
            coins: vec![
                assets.load_with_settings(
                    "images/coin.png",
                    |settings: &mut ImageLoaderSettings| {
                        // Use `nearest` image sampling to preserve pixel art style.
                        settings.sampler = ImageSampler::nearest();
                    },
                ),
                assets.load_with_settings(
                    "images/coin_old.png",
                    |settings: &mut ImageLoaderSettings| {
                        // Use `nearest` image sampling to preserve pixel art style.
                        settings.sampler = ImageSampler::nearest();
                    },
                ),
            ],
            coins_atlas_layout: texture_atlas_layout,
            flips: vec![
                assets.load("audio/sound_effects/step1.ogg"),
                assets.load("audio/sound_effects/step2.ogg"),
                assets.load("audio/sound_effects/step3.ogg"),
                assets.load("audio/sound_effects/step4.ogg"),
            ],
            heads: assets.load("audio/sound_effects/heads.ogg"),
        }
    }
}
