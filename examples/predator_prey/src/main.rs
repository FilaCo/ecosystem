use bevy::prelude::App;
use bevy::DefaultPlugins;
use bevy_ecosystem_simulation::prelude::EsPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EsPlugin)
        .run();
}
