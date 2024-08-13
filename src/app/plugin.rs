use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::prelude::*;

pub struct EsAppPlugin;

impl Plugin for EsAppPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<EsAppState>().add_loading_state(
            LoadingState::new(EsAppState::AppLoading)
                .continue_to_state(EsAppState::MenuRunning)
                .with_dynamic_assets_file::<StandardDynamicAssetCollection>(ES_ASSETS_FILE_PATH)
                .load_collection::<EsAssets>(),
        );
    }
}
