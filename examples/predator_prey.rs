use bevy::DefaultPlugins;
use bevy::prelude::{App, Component};

use bevy_ecosystem_simulation::prelude::EsPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EsPlugin)
        .run();
}

#[derive(Component, Debug)]
struct Rabbit;
