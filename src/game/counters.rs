use crate::{
    PausableSystems,
    game::level::{HighScoreUi, StreakUi},
};
use bevy::{color::palettes::css::YELLOW, prelude::*};
use bevy_pkv::prelude::*;
use serde::{Deserialize, Serialize};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, observe_coin.in_set(PausableSystems))
        .init_persistent_resource::<CoinCounter>();
}

#[derive(Resource, Default, Serialize, Deserialize)]
pub(crate) struct CoinCounter {
    pub(crate) last_was_heads: bool,
    pub(crate) streak: u32,
    pub(crate) highest_streak: u32,
}

fn observe_coin(
    mut commands: Commands,
    highscore: Single<Entity, With<HighScoreUi>>,
    streak: Single<Entity, With<StreakUi>>,
    counter: Res<CoinCounter>,
) {
    if counter.is_changed() {
        let mut highscore_entity = commands.entity(highscore.entity());

        highscore_entity.insert(Text::new(
            format!("High Score: {}", counter.highest_streak,),
        ));

        if counter.highest_streak == counter.streak {
            highscore_entity.insert(TextColor(YELLOW.into()));
            // highscore_entity.insert(TextShadow::default());
        } else {
            highscore_entity.insert(TextColor::WHITE);
            // highscore_entity.remove::<TextShadow>();
        }

        let mut streak_entity = commands.entity(streak.entity());
        if counter.streak > 0 {
            streak_entity.insert(TextColor(YELLOW.into()));
            // streak_entity.insert(TextShadow::default());
        } else {
            streak_entity.insert(TextColor::WHITE);
            // streak_entity.remove::<TextShadow>();
        }

        streak_entity.insert(Text::new(format!("Streak: {}", counter.streak,)));
    }
}
