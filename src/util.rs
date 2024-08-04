use bevy::prelude::*;
use bevy::window::CursorGrabMode;

pub fn despawn_by_component<T: Component>(mut commands: Commands, q: Query<Entity, With<T>>) {
    q.iter().for_each(|entity_id| {
        commands.entity(entity_id).despawn_recursive();
    });
}

pub fn toggle_grab_cursor(window: &mut Window) {
    match window.cursor.grab_mode {
        CursorGrabMode::None => {
            window.cursor.grab_mode = CursorGrabMode::Confined;
            window.cursor.visible = false;
        }
        _ => {
            window.cursor.grab_mode = CursorGrabMode::None;
            window.cursor.visible = true;
        }
    }
}

pub fn forward_vector(rotation: &Quat) -> Vec3 {
    rotation.mul_vec3(Vec3::Z).normalize()
}

pub fn forward_walk_vector(rotation: &Quat) -> Vec3 {
    let f = forward_vector(rotation);

    Vec3::new(f.x, 0.0, f.z).normalize()
}

pub fn strafe_vector(rotation: &Quat) -> Vec3 {
    // Rotate it 90 degrees to get the strafe direction
    Quat::from_rotation_y(90.0f32.to_radians())
        .mul_vec3(forward_walk_vector(rotation))
        .normalize()
}
