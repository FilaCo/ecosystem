use bevy::prelude::*;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, States)]
pub enum EsAppState {
    #[default]
    AppLoading,
    MenuRunning,
    SimulationRunning,
    AppClosing,
}
