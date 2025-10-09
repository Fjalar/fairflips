//! Player sprite animation.
//! This is based on multiple examples and may be very different for your game.
//! - [Sprite flipping](https://github.com/bevyengine/bevy/blob/latest/examples/2d/sprite_flipping.rs)
//! - [Sprite animation](https://github.com/bevyengine/bevy/blob/latest/examples/2d/sprite_animation.rs)
//! - [Timers](https://github.com/bevyengine/bevy/blob/latest/examples/time/timers.rs)

use std::f32::consts::PI;

use bevy::{
    input::{ButtonState, keyboard::KeyboardInput},
    prelude::*,
};
use rand::prelude::*;

use crate::{
    AppSystems, PausableSystems,
    audio::sound_effect,
    game::{coin::Coin, counters::CoinCounter, gameplay_assets::GameplayAssets, hand::Hand},
};

pub(super) fn plugin(app: &mut App) {
    // Animate and play sound effects based on controls.
    app.add_systems(
        Update,
        (
            keyboard_input
                .in_set(AppSystems::RecordInput)
                .in_set(PausableSystems),
            animate_coin.in_set(PausableSystems),
        ),
    );
}

fn keyboard_input(
    mut keyboard_messages: MessageReader<KeyboardInput>,
    mut hand_sprite: Single<&mut Sprite, With<Hand>>,
    coin_query: Single<(&mut Coin, &mut AnimationTimer)>,
    mut commands: Commands,
    player_assets: Res<GameplayAssets>,
) {
    let (mut coin, mut timer) = coin_query.into_inner();

    for message in keyboard_messages.read() {
        if message.key_code == KeyCode::Space {
            let Some(atlas) = hand_sprite.texture_atlas.as_mut() else {
                continue;
            };

            if message.state == ButtonState::Pressed && !message.repeat {
                atlas.index = 1;

                if !coin.currently_flipping {
                    let rng = &mut rand::rng();
                    let random_step = player_assets.steps.choose(rng).unwrap().clone();
                    commands.spawn(sound_effect(random_step));
                    coin.currently_flipping = true;
                    timer.0.unpause();
                }
            } else if message.state == ButtonState::Released {
                atlas.index = 0;
            };
        }
    }
}

fn animate_coin(
    coin_query: Single<(&mut Sprite, &mut Transform, &mut Coin, &mut AnimationTimer)>,
    time: Res<Time>,
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
