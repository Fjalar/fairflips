use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};

use crate::asset_tracking::LoadResource;

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<GameplayAssets>();
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct GameplayAssets {
    #[dependency]
    pub hand_image: Handle<Image>,
    #[dependency]
    pub coin_image: Handle<Image>,
    #[dependency]
    pub flips: Vec<Handle<AudioSource>>,
    #[dependency]
    pub heads: Handle<AudioSource>,
}

impl FromWorld for GameplayAssets {
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
