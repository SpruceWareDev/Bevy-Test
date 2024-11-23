use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

const PLAYER_SPEED: f32 = 100.0;

#[derive(Component)]
pub struct Player;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, init)
    .add_systems(Update, move_player)
    .run();
}

fn init(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // Test player
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(50.0, 60.0))),
            material: materials.add(Color::hsl(1., 1., 1.)),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
        Player,
    ));
}

fn move_player(
    mut query: Query<&mut Transform, With<Player>>,
    key_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let mut player_transform = query.single_mut();
    
    if key_input.pressed(KeyCode::ArrowLeft) {
        player_transform.translation.x -= PLAYER_SPEED * time.delta_seconds();
    } else if key_input.pressed(KeyCode::ArrowRight) {
        player_transform.translation.x += PLAYER_SPEED * time.delta_seconds();
    }
    if key_input.pressed(KeyCode::ArrowUp) {
        player_transform.translation.y += PLAYER_SPEED * time.delta_seconds();
    } else if key_input.pressed(KeyCode::ArrowDown) {
        player_transform.translation.y -= PLAYER_SPEED * time.delta_seconds();
    }
}