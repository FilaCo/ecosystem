use bevy::dev_tools::ui_debug_overlay::DebugUiPlugin;
use bevy::prelude::*;

use crate::prelude::*;

use super::system::*;

pub struct EsMenuPlugin;

impl Plugin for EsMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<EsMenuState>()
            .add_systems(
                OnEnter(EsAppState::MenuRunning),
                (setup_menu, setup_menu_camera),
            )
            .add_systems(
                OnExit(EsAppState::MenuRunning),
                despawn_by_component::<EsMenuCamera>, // TODO: OnMenuScreens?
            )
            .add_systems(OnEnter(EsMenuState::Main), setup_main_menu)
            .add_systems(
                OnExit(EsMenuState::Main),
                despawn_by_component::<OnMainMenuScreen>,
            )
            .add_systems(Update, handle_button_interactions);

        if cfg!(debug_assertions) {
            app.add_plugins(DebugUiPlugin)
                .add_systems(Update, toggle_overlay);
        }
    }
}
