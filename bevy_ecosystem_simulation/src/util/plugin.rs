use bevy::prelude::{App, Plugin};

pub struct EsUtilPlugin;

impl EsUtilPlugin {
    pub fn new() -> Self {
        Self {}
    }
}

impl Plugin for EsUtilPlugin {
    fn build(&self, app: &mut App) {}
}

impl Default for EsUtilPlugin {
    fn default() -> Self {
        Self::new()
    }
}
