use bevy::prelude::*;

use crate::util::toggle_grab_cursor;

pub fn setup_window(mut q: Query<&mut Window>) {
    q.iter_mut().for_each(|mut window| {
        window.title = "Ecosystem Simulation".into();
        toggle_grab_cursor(&mut window);
    });
}
