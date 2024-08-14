use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::prelude::*;

use super::system::*;

pub struct EsCameraPlugin;

impl Plugin for EsCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<EsCameraAction>::default())
            .add_systems(
                OnEnter(EsSimulationState::Running),
                (setup_camera, initial_grab_cursor),
            )
            .add_systems(Update, handle_camera_action);
    }
}
