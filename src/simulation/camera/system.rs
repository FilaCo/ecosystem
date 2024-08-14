use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};
use bevy_atmosphere::prelude::*;
use leafwing_input_manager::axislike::DualAxisData;
use leafwing_input_manager::InputManagerBundle;
use leafwing_input_manager::prelude::ActionState;

use crate::prelude::*;
use crate::util::*;

pub fn setup_camera(mut commands: Commands) {
    commands
        .spawn(EsCamera)
        .insert(Camera3dBundle {
            transform: Transform::from_xyz(0., 10., 0.),
            ..default()
        })
        .insert(InputManagerBundle::with_map(
            EsCameraAction::default_input_map(),
        ))
        .insert(AtmosphereCamera::default())
        .insert(OnSimulationScreen);
}

pub fn grab_cursor(
    mut q_window: Query<&mut Window, With<PrimaryWindow>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    q_window.iter_mut().for_each(|mut window| {
        if input.just_pressed(KeyCode::Escape) {
            toggle_grab_cursor(&mut window);
        }
    });
}

pub fn initial_grab_cursor(mut q: Query<&mut Window, With<PrimaryWindow>>) {
    q.iter_mut()
        .for_each(|mut window| toggle_grab_cursor(&mut window));
}

pub fn handle_camera_action(
    mut q: Query<(&mut Transform, &ActionState<EsCameraAction>), With<EsCamera>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
) {
    q_window.iter().for_each(|window| {
        q.iter_mut()
            .for_each(|(mut camera_transform, camera_action)| {
                if window.cursor.grab_mode == CursorGrabMode::None {
                    return;
                }

                // handle pan
                let camera_pan_vector = camera_action
                    .axis_pair(&EsCameraAction::Pan)
                    .unwrap_or(DualAxisData::default());

                if camera_pan_vector.length_squared() > 0. {
                    let (mut yaw, mut pitch, _) = camera_transform.rotation.to_euler(EulerRot::YXZ);

                    let window_scale = window.height().min(window.width());
                    pitch -= (0.00012 * camera_pan_vector.y() * window_scale).to_radians();
                    yaw -= (0.00012 * camera_pan_vector.x() * window_scale).to_radians();

                    pitch = pitch.clamp(-1.54, 1.54);

                    // Order is important to prevent unintended roll
                    camera_transform.rotation =
                        Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
                }

                // handle move
                let mut velocity = Vec3::ZERO;

                let camera_move_vector = camera_action
                    .axis_pair(&EsCameraAction::Move)
                    .unwrap_or(DualAxisData::default());

                if camera_move_vector.length_squared() > 0. {
                    velocity += (strafe_vector(&camera_transform.rotation) * camera_move_vector.x())
                        - (forward_walk_vector(&camera_transform.rotation) * camera_move_vector.y())
                }

                if camera_action.pressed(&EsCameraAction::Ascend) {
                    velocity += Vec3::Y
                } else if camera_action.pressed(&EsCameraAction::Descend) {
                    velocity -= Vec3::Y;
                }

                velocity = velocity.normalize_or_zero();

                camera_transform.translation += velocity * time.delta_seconds() * 12.;
            });
    });
}
