use bevy::prelude::App;
use bevy::DefaultPlugins;
use bevy_ecosystem_simulation::prelude::EcosystemSimulationPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EcosystemSimulationPlugin)
        .run();
}
