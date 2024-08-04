use bevy::prelude::*;

use crate::environment::system::{setup_lighting, setup_terrain};

pub struct EsEnvironmentPlugin;

impl Plugin for EsEnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_terrain, setup_lighting));
    }
}
