use bevy::prelude::*;

use crate::prelude::*;
use crate::simulation::system::*;

pub struct EsSimulationPlugin;

impl Plugin for EsSimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((EsCameraPlugin, EsEnvironmentPlugin, EsRabbitPlugin))
            .init_state::<EsSimulationState>()
            .add_systems(OnEnter(EsAppState::SimulationRunning), setup_simulation);
    }
}
