use bevy::prelude::*;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, States)]
pub enum EsMenuState {
    #[default]
    Disabled,
    Main,
    Settings,
}
