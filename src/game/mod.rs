//! Demo gameplay. All of these modules are only intended for demonstration
//! purposes and should be replaced with your own game logic.
//! Feel free to change the logic found here if you feel like tinkering around
//! to get a feeling for the template.

use bevy::prelude::*;

mod coin;
pub mod counters;
mod gameplay_assets;
mod hand;
mod input;
pub mod level;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        counters::plugin,
        gameplay_assets::plugin,
        input::plugin,
        coin::plugin,
    ));
}
