use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct EsAssets {
    #[asset(key = "font")]
    pub font: Handle<Font>,
    // #[asset(key = "materials", collection(typed, mapped))]
    // pub materials: HashMap<String, Handle<StandardMaterial>>,
    // #[asset(key = "models", collection(typed, mapped))]
    // pub models: HashMap<String, Handle<Gltf>>,
}
