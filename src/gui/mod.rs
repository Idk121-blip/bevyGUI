pub(crate) mod components;
pub mod hud;
pub(crate) mod system;
pub(crate) mod utils;

use bevy::prelude::*;
pub struct GuiPlugin;
impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, system::startup::setup)
            .add_plugins(hud::HudPlugin)
            .add_systems(Update, system::updates::update_map);
    }
}
