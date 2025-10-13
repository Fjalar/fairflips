//! The settings menu.
//!
//! Additional settings and accessibility options should go here.

use bevy::{audio::Volume, input::common_conditions::input_just_pressed, prelude::*};

use crate::{
    game::{coin::Coin, counters::CoinCounter, gameplay_assets::GameplayAssets},
    menus::Menu,
    screens::Screen,
    theme::prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Settings), spawn_settings_menu);
    app.add_systems(
        Update,
        go_back.run_if(in_state(Menu::Settings).and(input_just_pressed(KeyCode::Escape))),
    );

    app.add_systems(
        Update,
        update_global_volume_label.run_if(in_state(Menu::Settings)),
    );

    app.init_resource::<ChosenCoin>();
}

fn spawn_settings_menu(
    mut commands: Commands,
    gameplay_assets: Res<GameplayAssets>,
    chosen_coin: Res<ChosenCoin>,
) {
    commands.spawn((
        widget::ui_root("Settings Menu"),
        GlobalZIndex(2),
        DespawnOnExit(Menu::Settings),
        children![
            widget::header("Settings"),
            settings_grid(gameplay_assets, chosen_coin),
            widget::button("Back", go_back_on_click),
        ],
    ));
}

fn settings_grid(
    gameplay_assets: Res<GameplayAssets>,
    chosen_coin: Res<ChosenCoin>,
) -> impl Bundle {
    (
        Name::new("Settings Grid"),
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            row_gap: px(10),
            // grid_template_columns: GridTrack::px(400.0),
            // grid_template_rows: RepeatedGridTrack::auto(6),
            justify_items: JustifyItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![
            (
                widget::label("Master Volume"),
                Node {
                    justify_self: JustifySelf::Center,
                    ..default()
                }
            ),
            global_volume_widget(),
            reset_progress_widget(),
            coin_carousel_widget(gameplay_assets, chosen_coin)
        ],
    )
}

fn global_volume_widget() -> impl Bundle {
    (
        Name::new("Global Volume Widget"),
        Node {
            justify_self: JustifySelf::Start,
            ..default()
        },
        children![
            widget::button_small("-", lower_global_volume),
            (
                Name::new("Current Volume"),
                Node {
                    padding: UiRect::horizontal(px(10)),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                children![(widget::label(""), GlobalVolumeLabel)],
            ),
            widget::button_small("+", raise_global_volume),
        ],
    )
}

const MIN_VOLUME: f32 = 0.0;
const MAX_VOLUME: f32 = 3.0;

fn lower_global_volume(_: On<Pointer<Click>>, mut global_volume: ResMut<GlobalVolume>) {
    let linear = (global_volume.volume.to_linear() - 0.1).max(MIN_VOLUME);
    global_volume.volume = Volume::Linear(linear);
}

fn raise_global_volume(_: On<Pointer<Click>>, mut global_volume: ResMut<GlobalVolume>) {
    let linear = (global_volume.volume.to_linear() + 0.1).min(MAX_VOLUME);
    global_volume.volume = Volume::Linear(linear);
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct GlobalVolumeLabel;

fn update_global_volume_label(
    global_volume: Res<GlobalVolume>,
    mut label: Single<&mut Text, With<GlobalVolumeLabel>>,
) {
    let percent = 100.0 * global_volume.volume.to_linear();
    label.0 = format!("{percent:3.0}%");
}

fn reset_progress_widget() -> impl Bundle {
    (
        Name::new("Reset Progress Widget"),
        Node { ..default() },
        children![(widget::button_red("Reset Progress", reset_progress),),],
    )
}

fn reset_progress(_: On<Pointer<Click>>, mut counter: ResMut<CoinCounter>) {
    *counter = CoinCounter::default();
}

fn coin_carousel_widget(
    gameplay_assets: Res<GameplayAssets>,
    chosen_coin: Res<ChosenCoin>,
    // texture_atlas_layouts: Res<Assets<TextureAtlasLayout>>,
) -> impl Bundle {
    (
        Name::new("Coin Carousel Widget"),
        Node {
            justify_items: JustifyItems::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        children![
            widget::button_small("<", decrement_chosen_coin),
            (
                Name::new("Coin Carousel"),
                Node {
                    justify_content: JustifyContent::Center,
                    padding: UiRect::horizontal(px(10)),
                    ..default()
                },
                children![(
                    Coin {
                        currently_flipping: false
                    },
                    ImageNode::from_atlas_image(
                        gameplay_assets.coins[chosen_coin.0].clone(),
                        TextureAtlas::from(gameplay_assets.coins_atlas_layout.clone()),
                    ),
                    Node {
                        width: px(64),
                        height: px(64),
                        ..default()
                    },
                )],
            ),
            widget::button_small(">", increment_chosen_coin),
        ],
    )
}

#[derive(Resource, Deref, DerefMut, Default)]
pub struct ChosenCoin(pub usize);

fn decrement_chosen_coin(
    _: On<Pointer<Click>>,
    gameplay_assets: Res<GameplayAssets>,
    ui: Query<(&mut ImageNode,), With<Coin>>,
    sprites: Query<&mut Sprite, With<Coin>>,
    mut chosen_coin: ResMut<ChosenCoin>,
) {
    chosen_coin.0 = chosen_coin.saturating_sub(1);
    for mut img_node in ui {
        img_node.0.image = gameplay_assets.coins[chosen_coin.0].clone();
    }
    for mut sprite in sprites {
        sprite.image = gameplay_assets.coins[chosen_coin.0].clone();
    }
}

fn increment_chosen_coin(
    _: On<Pointer<Click>>,
    gameplay_assets: Res<GameplayAssets>,
    ui: Query<(&mut ImageNode,), With<Coin>>,
    sprites: Query<&mut Sprite, With<Coin>>,
    mut chosen_coin: ResMut<ChosenCoin>,
) {
    chosen_coin.0 = (chosen_coin.0 + 1).min(gameplay_assets.coins.len() - 1);
    for mut img_node in ui {
        img_node.0.image = gameplay_assets.coins[chosen_coin.0].clone();
    }
    for mut sprite in sprites {
        sprite.image = gameplay_assets.coins[chosen_coin.0].clone();
    }
}

fn go_back_on_click(
    _: On<Pointer<Click>>,
    screen: Res<State<Screen>>,
    mut next_menu: ResMut<NextState<Menu>>,
) {
    next_menu.set(if screen.get() == &Screen::Title {
        Menu::Main
    } else {
        Menu::Pause
    });
}

fn go_back(screen: Res<State<Screen>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(if screen.get() == &Screen::Title {
        Menu::Main
    } else {
        Menu::Pause
    });
}
