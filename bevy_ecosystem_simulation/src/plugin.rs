use bevy::app::App;
use bevy::prelude::Plugin;

pub struct EcosystemSimulationPlugin;

impl EcosystemSimulationPlugin {
    pub fn new() -> Self {
        Self {}
    }
}

impl Plugin for EcosystemSimulationPlugin {
    fn build(&self, app: &mut App) {}
}

impl Default for EcosystemSimulationPlugin {
    fn default() -> Self {
        Self::new()
    }
}
