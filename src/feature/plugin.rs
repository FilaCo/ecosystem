use bevy::prelude::*;

use crate::feature::system::{setup_camera, setup_environment, setup_window, spawn_rabbit};

pub struct EsFeaturePlugin;

impl EsFeaturePlugin {
    pub fn new() -> Self {
        Self {}
    }
}

impl Plugin for EsFeaturePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_window)
            .add_systems(Startup, (setup_environment, setup_camera, spawn_rabbit));
    }
}

impl Default for EsFeaturePlugin {
    fn default() -> Self {
        Self::new()
    }
}
