use bevy::prelude::*;

use super::system::*;

pub struct EsEnvironmentPlugin;

impl Plugin for EsEnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_terrain, setup_lighting));
    }
}
