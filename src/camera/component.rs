use bevy::input::keyboard::NativeKeyCode;
use bevy::prelude::*;
use leafwing_input_manager::Actionlike;
use leafwing_input_manager::prelude::*;

#[derive(Component, Debug)]
pub struct EsCamera;

#[derive(Actionlike, Clone, Copy, Debug, Eq, Hash, PartialEq, Reflect)]
pub enum EsCameraAction {
    Pan,
    Move,
    Ascend,
    Descend,
    // TODO: actually not a camera action
    GrabCursor,
}

impl EsCameraAction {
    pub fn default_input_map() -> InputMap<Self> {
        let mut input_map = InputMap::default();

        // default mouse-keyboard bindings
        input_map
            .insert(Self::Pan, DualAxis::mouse_motion())
            .insert(Self::Move, VirtualDPad::wasd())
            .insert(Self::Ascend, KeyCode::Space)
            .insert(Self::Descend, KeyCode::ShiftLeft)
            .insert(Self::GrabCursor, KeyCode::Escape);

        input_map
    }
}
