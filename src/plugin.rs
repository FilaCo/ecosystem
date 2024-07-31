use bevy::DefaultPlugins;
use bevy::diagnostic::*;
use bevy::prelude::*;

use crate::feature::EsFeaturePlugin;

pub struct EsPlugin;

impl EsPlugin {
    pub fn new() -> Self {
        Self {}
    }
}

impl Plugin for EsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                visible: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EsFeaturePlugin);

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
