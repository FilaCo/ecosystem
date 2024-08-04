use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::camera::component::EsCameraAction;
use crate::camera::system::{grab_cursor, handle_camera_action, setup_camera};

pub struct EsCameraPlugin;

impl Plugin for EsCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<EsCameraAction>::default())
            .add_systems(Startup, setup_camera)
            .add_systems(Update, (handle_camera_action, grab_cursor));
    }
}
