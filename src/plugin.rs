use bevy::DefaultPlugins;
use bevy::diagnostic::*;
use bevy::prelude::*;

use crate::window::plugin::EsWindowPlugin;

pub struct EsPlugin;

impl EsPlugin {
    pub fn new() -> Self {
        Self {}
    }
}

impl Plugin for EsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins).add_plugins(EsWindowPlugin);

        if cfg!(debug_assertions) {
            app.add_plugins((
                FrameTimeDiagnosticsPlugin,
                LogDiagnosticsPlugin::default(),
                EntityCountDiagnosticsPlugin,
            ));
        }
    }
}

impl Default for EsPlugin {
    fn default() -> Self {
        Self::new()
    }
}
