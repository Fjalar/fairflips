//! Spawn the main level.

use bevy::{color::palettes::css::BLACK, prelude::*};

use crate::{
    game::{coin, counters::CoinCounter, gameplay_assets::GameplayAssets, hand},
    screens::Screen,
};

/// A system that spawns the main level.
pub fn spawn_level(
    mut commands: Commands,
    gameplay_assets: Res<GameplayAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    counter: Res<CoinCounter>,
) {
    commands.spawn((
        Name::new("Level"),
        Transform::default(),
        Visibility::default(),
        DespawnOnExit(Screen::Gameplay),
        Node {
            width: percent(100),
            height: percent(100),
            justify_content: JustifyContent::End,
            ..Default::default()
        },
        children![
            hand::hand(&gameplay_assets, &mut texture_atlas_layouts),
            coin::coin(&gameplay_assets, &mut texture_atlas_layouts),
            (
                Node::default(),
                children![(
                    Node {
                        display: Display::Grid,
                        row_gap: px(10),
                        max_height: px(200),
                        align_content: AlignContent::Center,
                        align_items: AlignItems::Center,
                        justify_items: JustifyItems::Center,
                        ..default()
                    },
                    children![
                        (
                            Node {
                                ..Default::default()
                            },
                            BorderRadius::all(px(20)),
                            BackgroundColor(BLACK.into()),
                            children![(
                                Node {
                                    margin: UiRect::all(px(10)),
                                    ..Default::default()
                                },
                                HighScoreUi,
                                // GlobalZIndex(2),
                                Text::new(format!("High Score: {}", counter.highest_streak)),
                                TextFont::from_font_size(20.0),
                                TextLayout::new_with_justify(Justify::Center),
                            )],
                        ),
                        (
                            Node {
                                ..Default::default()
                            },
                            BorderRadius::all(px(20)),
                            BackgroundColor(BLACK.into()),
                            children![(
                                Node {
                                    margin: UiRect::all(px(10)),
                                    ..Default::default()
                                },
                                StreakUi,
                                Text::new(format!("Streak: {}", counter.streak)),
                                TextFont::from_font_size(20.0),
                                TextLayout::new_with_justify(Justify::Center),
                            )]
                        ),
                    ]
                )]
            ),
        ],
    ));
}

#[derive(Component)]
pub struct HighScoreUi;

#[derive(Component)]
pub struct StreakUi;
