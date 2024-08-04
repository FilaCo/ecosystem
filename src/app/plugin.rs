use bevy::prelude::*;

use crate::app::state::EsAppState;

pub struct EsAppPlugin;

impl Plugin for EsAppPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<EsAppState>();
    }
}
