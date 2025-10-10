use bevy::prelude::*;

use crate::game::coin::AnimationTimer;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, tick_popup);
}

pub fn popup(text: impl Into<String>) -> impl Bundle {
    (
        Popup,
        AnimationTimer(Timer::from_seconds(1.0, TimerMode::Once)),
        Text2d::new(text),
        TextColor(Color::WHITE),
        TextFont::from_font_size(80.0),
        Transform::from_translation(Vec3::Y * 200.0).with_scale(Vec2::splat(0.5).extend(1.0)),
    )
}

fn tick_popup(
    time: Res<Time>,
    mut commands: Commands,
    query: Query<(Entity, &mut AnimationTimer, &mut TextColor, &mut Transform), With<Popup>>,
) {
    for (e, mut timer, mut color, mut transform) in query {
        timer.tick(time.delta());
        if timer.is_finished() {
            commands.entity(e).despawn();
            return;
        }

        color.set_alpha(timer.fraction_remaining());

        transform.translation.y += 0.2;
        transform.scale += 0.005;
    }
}

#[derive(Component)]
struct Popup;
