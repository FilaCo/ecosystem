use bevy::prelude::*;

use crate::menu::state::EsMenuState;

pub struct EsMenuPlugin;

impl Plugin for EsMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<EsMenuState>();
    }
}
