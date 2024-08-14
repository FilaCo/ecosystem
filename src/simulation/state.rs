use bevy::prelude::*;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, States)]
pub enum EsSimulationState {
    #[default]
    Disabled,
    Running,
    Paused,
}
