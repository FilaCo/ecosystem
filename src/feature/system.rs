use bevy::prelude::*;

use crate::feature::component::{FlyCam, OnGameScreen, Rabbit, Terrain};

pub fn setup_window(mut q: Query<&mut Window>) {
    q.iter_mut().for_each(|mut window| {
        window.title = "Ecosystem Simulation".into();
        window.visible = true
    });
}

pub fn spawn_rabbit(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(Rabbit)
        .insert(OnGameScreen)
        .insert(PbrBundle {
            material: materials.add(Color::WHITE),
            mesh: meshes.add(Cuboid::new(1., 1., 1.)),
            transform: Transform::from_xyz(0., 0.5, 0.),
            ..default()
        });
}

pub fn setup_environment(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Setup terrain
    commands
        .spawn(Terrain)
        .insert(OnGameScreen)
        .insert(PbrBundle {
            material: materials.add(Color::srgb_u8(0, 255, 0)),
            mesh: meshes.add(Plane3d::new(Vec3::Y, Vec2::new(50., 50.))),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        });

    // Setup light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 10.0, 4.0),
        ..default()
    });
}

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(FlyCam).insert(Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
