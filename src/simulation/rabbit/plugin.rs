use bevy::prelude::*;

use crate::prelude::*;

use super::system::*;

pub struct EsRabbitPlugin;

impl Plugin for EsRabbitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(EsSimulationState::Running), spawn_rabbit);
    }
}
