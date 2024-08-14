use bevy::prelude::*;

use crate::prelude::*;

pub fn setup_simulation(mut simulation_state: ResMut<NextState<EsSimulationState>>) {
    simulation_state.set(EsSimulationState::Running);
}
