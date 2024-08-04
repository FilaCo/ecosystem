use std::f32::consts::PI;

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
        directional_light: DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        ..default()
    });
}
