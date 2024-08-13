use bevy::DefaultPlugins;
use bevy::diagnostic::*;
use bevy::prelude::*;
use bevy_atmosphere::prelude::*;
use bevy_editor_pls::EditorPlugin;

use crate::prelude::*;

pub struct EsPlugin;

impl EsPlugin {
    pub fn new() -> Self {
        Self {}
    }
}

impl Plugin for EsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins)
            .add_plugins(AtmospherePlugin)
            .add_plugins(EsAppPlugin)
            .add_plugins(EsMenuPlugin)
            .add_plugins(EsWindowPlugin);

        if cfg!(debug_assertions) {
            app.add_plugins((
                FrameTimeDiagnosticsPlugin,
                LogDiagnosticsPlugin::default(),
                EntityCountDiagnosticsPlugin,
                // EditorPlugin::default(),
            ));
        }
    }
}

impl Default for EsPlugin {
    fn default() -> Self {
        Self::new()
    }
}
