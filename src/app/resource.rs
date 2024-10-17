use bevy::prelude::*;
use bevy::utils::hashbrown::HashMap;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Debug, Resource)]
pub struct EsAssets {
    #[asset(key = "font")]
    pub font: Handle<Font>,

    ///
    /// Scene0 - Fox
    ///
    /// Scene1 - Rabbit
    ///
    #[asset(
        paths("world.glb#Scene0", "world.glb#Scene1"),
        collection(mapped, typed)
    )]
    pub models: HashMap<AssetLabel, Handle<Scene>>,
}
