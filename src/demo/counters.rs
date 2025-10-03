use crate::PausableSystems;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, observe_coin.in_set(PausableSystems))
        .init_resource::<CoinCounter>();
}

#[derive(Resource, Default)]
pub(crate) struct CoinCounter {
    pub(crate) last_was_heads: bool,
    pub(crate) streak: u32,
    pub(crate) highest_streak: u32,
}

#[derive(Component)]
struct CounterUi;

pub(crate) fn counter_ui(counter: Res<CoinCounter>) -> impl Bundle {
    (
        CounterUi,
        GlobalZIndex(2),
        Text2d::new(format!(
            "High Score: {}\nStreak: {}",
            counter.highest_streak, counter.streak
        )),
        Transform::from_translation(Vec3::new(0.0, 200.0, 0.0)),
    )
}

fn observe_coin(
    mut commands: Commands,
    query: Single<Entity, With<CounterUi>>,
    counter: Res<CoinCounter>,
) {
    if counter.is_changed() {
        commands.entity(query.entity()).insert(Text2d::new(format!(
            "High Score: {}\nStreak: {}",
            counter.highest_streak, counter.streak
        )));
    }
}
