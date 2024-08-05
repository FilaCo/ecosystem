use bevy::prelude::*;

use crate::environment::component::Terrain;

pub fn setup_terrain(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(Terrain).insert(PbrBundle {
        mesh: meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(25.))),
        material: materials.add(Color::srgb(0.3, 0.5, 0.3)),
        ..default()
    });
}

pub fn setup_lighting(mut commands: Commands) {
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_rotation(Quat::from_euler(
            EulerRot::ZYX,
            0.0,
            1.0,
            -std::f32::consts::FRAC_PI_4,
        )),
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });
}
