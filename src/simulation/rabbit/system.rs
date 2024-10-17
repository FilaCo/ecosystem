use bevy::prelude::*;

use crate::prelude::*;

pub fn spawn_rabbit(mut commands: Commands, assets: Res<EsAssets>) {
    commands
        .spawn(Rabbit)
        .insert(OnSimulationScreen)
        .insert(SceneBundle {
            scene: assets.models.get("Scene1").unwrap().clone(),
            ..default()
        });
}
