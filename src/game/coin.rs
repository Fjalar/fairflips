use std::f32::consts::PI;

use bevy::prelude::*;
use rand::{Rng, seq::IndexedRandom};

use crate::{
    PausableSystems,
    audio::sound_effect,
    game::{counters::CoinCounter, gameplay_assets::GameplayAssets, input::Flip},
};

pub fn plugin(app: &mut App) {
    app.add_observer(
        |_flip: On<Flip>,
         coin_query: Single<(&mut Coin, &mut AnimationTimer)>,
         mut commands: Commands,
         gameplay_assets: Res<GameplayAssets>| {
            let (mut coin, mut timer) = coin_query.into_inner();

            if !coin.currently_flipping {
                let rng = &mut rand::rng();
                let random_step = gameplay_assets.flips.choose(rng).unwrap().clone();
                commands.spawn(sound_effect(random_step));
                coin.currently_flipping = true;
                timer.0.unpause();
            }
        },
    )
    .add_systems(Update, (update_coin.in_set(PausableSystems),));
}

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

fn update_coin(
    coin_query: Single<(&mut Sprite, &mut Transform, &mut Coin, &mut AnimationTimer)>,
    time: Res<Time>,
    mut commands: Commands,
    gameplay_assets: Res<GameplayAssets>,
    mut counters: ResMut<CoinCounter>,
) {
    let (mut sprite, mut transform, mut coin, mut timer) = coin_query.into_inner();
    timer.tick(time.delta());

    *transform = transform
        .with_translation(Vec3::new(
            0.0,
            50.0 - 350.0 * f32::abs(timer.0.elapsed_secs() - 0.5),
            1.0,
        ))
        .with_rotation(Quat::from_rotation_x(2.0 * PI * timer.0.elapsed_secs()));

    if coin.currently_flipping {
        let rng = &mut rand::rng();
        let Some(atlas) = sprite.texture_atlas.as_mut() else {
            return;
        };
        if timer.0.is_finished() {
            timer.reset();
            timer.0.pause();
            coin.currently_flipping = false;

            if rng.random_bool(0.5) {
                counters.streak += 1;
                counters.highest_streak = counters.highest_streak.max(counters.streak);
                counters.last_was_heads = true;
                atlas.index = 0;
                commands.spawn(sound_effect(gameplay_assets.heads.clone()));
            } else {
                counters.streak = 0;
                counters.last_was_heads = false;
                atlas.index = 1;
            }
        } else if ((timer.fraction() * 10.0) as u32).is_multiple_of(3) {
            if atlas.index == 0 {
                atlas.index = 1;
            } else {
                atlas.index = 0;
            }
        }
    }
}

#[derive(Component, Deref, DerefMut)]
pub(crate) struct AnimationTimer(pub(crate) Timer);
