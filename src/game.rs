use bevy::{input::{mouse::MouseButtonInput, ButtonState}, prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}, window::PrimaryWindow};

use libm::{atan2f, cosf, sinf};

const PLAYER_SPEED: f32 = 150.0;
const REGULAR_BULLET_SPEED: f32 = 200.0;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Health(u8);

#[derive(Component)]
pub struct Projectile {
    speed: f32,
    dx: f32,
    dy: f32,
}

#[derive(Component)]
pub struct Bullet;

pub fn init(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
     // Test player
     commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(50.0, 60.0))),
            material: materials.add(Color::hsl(1., 1., 1.)),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
        Player,
        Health(100),
    ));
}

pub fn move_player(
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

pub fn shoot(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<&Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut mouse_input_events: EventReader<MouseButtonInput>,
) {
    let player_transform = query.single();
    
    for _ in mouse_input_events.read().filter(|input_event| input_event.state == ButtonState::Pressed) {
        if let Some(position) = window_query.single().cursor_position() {
            // Get player position.
            let player_x = player_transform.translation.x;
            let player_y = player_transform.translation.y;
            
            // Calculate difference between player position and mouse position.
            let diff_x = position.x - player_x;
            let diff_y = position.y - (player_y).abs();

            // Calculate angle between mouse and player.
            let angle = atan2f(diff_x, diff_y);

            // Calculate delta x and delta y for bullet.
            let bullet_dx = sinf(angle);
            let bullet_dy = -cosf(angle);

            // Spawn bullet.
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: Mesh2dHandle(meshes.add(Rectangle::new(10.0, 10.0))),
                    material: materials.add(Color::hsl(0.5, 1., 1.)),
                    transform: Transform::from_xyz(player_x, player_y, 0.),
                    ..default()
                },
                Projectile {
                    speed: REGULAR_BULLET_SPEED,
                    dx: bullet_dx,
                    dy: bullet_dy,
                },
                Bullet,
            ));
        } 
    }
}

pub fn update_bullets(
    mut bullet_query: Query<(&mut Transform, &Projectile), With<Bullet>>,
    time: Res<Time>,
) {
    for (mut transform, projectile) in &mut bullet_query {
        transform.translation.x += projectile.dx * projectile.speed * time.delta_seconds();
        transform.translation.y += projectile.dy * projectile.speed * time.delta_seconds();
    }
}