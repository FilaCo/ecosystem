use bevy::prelude::*;

use crate::prelude::*;

use super::system::*;

pub struct EsEnvironmentPlugin;

impl Plugin for EsEnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(EsSimulationState::Running),
            (setup_terrain, setup_lighting),
        );
    }
}
