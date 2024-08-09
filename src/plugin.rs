use bevy::DefaultPlugins;
use bevy::diagnostic::*;
use bevy::prelude::*;
use bevy_atmosphere::prelude::*;
use bevy_editor_pls::EditorPlugin;

use crate::app::plugin::EsAppPlugin;
use crate::camera::plugin::EsCameraPlugin;
use crate::environment::plugin::EsEnvironmentPlugin;
use crate::menu::plugin::EsMenuPlugin;
use crate::window::plugin::EsWindowPlugin;

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
            // .add_plugins(EsCameraPlugin)
            // .add_plugins(EsEnvironmentPlugin)
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
