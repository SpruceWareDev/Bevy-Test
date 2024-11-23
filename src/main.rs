use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, init)
    .run();
}

fn init(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // Test square
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(100.0, 100.0))),
            material: materials.add(Color::hsl(1., 1., 1.)),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
    ));
}
