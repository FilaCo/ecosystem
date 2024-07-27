use crate::feature::EsFeaturePlugin;
use crate::util::EsUtilPlugin;
use bevy::prelude::{App, Plugin};

pub struct EsPlugin;

impl EsPlugin {
    pub fn new() -> Self {
        Self {}
    }
}

impl Plugin for EsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((EsFeaturePlugin, EsUtilPlugin));
    }
}

impl Default for EsPlugin {
    fn default() -> Self {
        Self::new()
    }
}
