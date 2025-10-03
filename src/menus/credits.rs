//! The credits menu.

use bevy::{ecs::spawn::SpawnIter, input::common_conditions::input_just_pressed, prelude::*};

use crate::{menus::Menu, theme::prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Credits), spawn_credits_menu);
    app.add_systems(
        Update,
        go_back.run_if(in_state(Menu::Credits).and(input_just_pressed(KeyCode::Escape))),
    );
}

fn spawn_credits_menu(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Credits Menu"),
        GlobalZIndex(2),
        DespawnOnExit(Menu::Credits),
        children![
            widget::header("Created by"),
            created_by(),
            widget::header("Thanks to"),
            thanks(),
            widget::header("Assets"),
            assets(),
            widget::button("Back", go_back_on_click),
        ],
        TextFont {
            font_size: 5.0,
            ..Default::default()
        },
    ));
}

fn created_by() -> impl Bundle {
    grid(vec![
        ["Fjalar", "All the bad code"],
        ["The Bevy \n2D template", "All the good code"],
    ])
}

fn thanks() -> impl Bundle {
    grid(vec![
        ["Unfair Flips\non steam", "The concept"],
        [
            "Ryan \"Gary\" Letourneau\nAKA Northern",
            "Linking and building\nthis summer",
        ],
    ])
}

fn assets() -> impl Bundle {
    grid(vec![
        ["Button SFX", "CC0 by Jaszunio15"],
        ["Sprites", "The GNU Image\nManipulation Program"],
    ])
}

fn grid(content: Vec<[&'static str; 2]>) -> impl Bundle {
    (
        Name::new("Grid"),
        Node {
            display: Display::Grid,
            row_gap: px(10),
            column_gap: px(30),
            grid_template_columns: RepeatedGridTrack::px(2, 400.0),
            ..default()
        },
        Children::spawn(SpawnIter(content.into_iter().flatten().enumerate().map(
            |(i, text)| {
                (
                    widget::label_small(text),
                    Node {
                        justify_self: if i.is_multiple_of(2) {
                            JustifySelf::End
                        } else {
                            JustifySelf::Start
                        },
                        ..default()
                    },
                )
            },
        ))),
    )
}

fn go_back_on_click(_: On<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Main);
}

fn go_back(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Main);
}
