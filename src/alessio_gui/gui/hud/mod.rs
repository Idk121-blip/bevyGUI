mod components;
mod styles;
mod systems;

use systems::layout::*;

use crate::gui::hud::systems::updates::*;
use bevy::prelude::*;
use bevy_progressbar;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter Systems
            .add_plugins(bevy_progressbar::ProgressBarPlugin)
            .add_systems(PostStartup, spawn_hud)
            .add_systems(Update, text_updater);
    }
}
