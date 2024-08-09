use bevy::prelude::*;

pub fn setup_window(mut q: Query<&mut Window>) {
    q.iter_mut().for_each(|mut window| {
        window.title = "Ecosystem Simulation".into();
    });
}
