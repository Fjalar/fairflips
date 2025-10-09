use bevy::prelude::*;

use crate::{AppSystems, PausableSystems, game::hand::Hand};

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        input
            .in_set(AppSystems::RecordInput)
            .in_set(PausableSystems),
    );
}

fn input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    touch: Res<Touches>,
    mut hand_sprite: Single<&mut Sprite, With<Hand>>,
    mut commands: Commands,
) {
    if keyboard.just_pressed(KeyCode::Space)
        || mouse.just_pressed(MouseButton::Left)
        || touch.any_just_pressed()
    {
        commands.trigger(Flip);
    };
    let Some(atlas) = hand_sprite.texture_atlas.as_mut() else {
        return;
    };

    if keyboard.pressed(KeyCode::Space)
        || mouse.pressed(MouseButton::Left)
        || touch.iter().next().is_some()
    {
        atlas.index = 1;
    } else {
        atlas.index = 0;
    }
}

#[derive(Event)]
pub struct Flip;
