use bevy::prelude::App;

use ecosystem::prelude::EsPlugin;

fn main() {
    App::new().add_plugins(EsPlugin).run();
}
