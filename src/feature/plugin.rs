use bevy::prelude::{App, Plugin};

pub struct EsFeaturePlugin;

impl EsFeaturePlugin {
    pub fn new() -> Self {
        Self {}
    }
}

impl Plugin for EsFeaturePlugin {
    fn build(&self, app: &mut App) {}
}

impl Default for EsFeaturePlugin {
    fn default() -> Self {
        Self::new()
    }
}
