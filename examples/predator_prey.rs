use bevy::DefaultPlugins;
use bevy::prelude::*;

use bevy_ecosystem_simulation::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EsPlugin)
        .add_systems(Startup, (setup_environment, setup_rabbit, setup_camera))
        .run();
}

#[derive(Component, Debug)]
struct Rabbit;

#[derive(Component, Debug)]
struct FlyCam;

#[derive(Component, Debug)]
struct Terrain;

fn setup_environment(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Terrain,
        PbrBundle {
            mesh: meshes.add(Plane3d::new(Vec3::Y, Vec2::new(50., 50.))),
            material: materials.add(Color::srgb_u8(0, 255, 0)),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
    ));

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}

fn setup_rabbit(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Rabbit,
        Agent,
        PbrBundle {
            mesh: meshes.add(Cuboid::new(1., 1., 1.)),
            material: materials.add(Color::WHITE),
            transform: Transform::from_xyz(0., 0.5, 0.),
            ..default()
        },
    ));
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        FlyCam,
        Camera3dBundle {
            transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
    ));
}
