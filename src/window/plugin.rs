use bevy::prelude::*;

use crate::window::system::setup_window;

pub struct EsWindowPlugin;

impl Plugin for EsWindowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_window);
    }
}
