use core::f32::consts::PI;

use bevy::{
    prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}, ui::update, window::PrimaryWindow
};

use libm::{acosf, atan2f, cosf, pow, powf, sinf, sqrtf};

mod utils;

const PLAYER_SPEED: f32 = 100.0;
const REGULAR_BULLET_SPEED: f32 = 150.0;

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

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, init)
    .add_systems(Update, (move_player, shoot, update_bullets))
    .run();
}

fn init(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window_width = window_query.single().width();
    let window_height = window_query.single().height();

    // Camera
    commands.spawn(
        Camera2dBundle {
            transform: Transform::from_xyz(window_width / 2.0, -window_height / 2.0, 0.0),
            ..default()
        });
    
    //commands.spawn(Camera2dBundle::default());

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

fn shoot(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<&Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
) {
    let player_transform = query.single();
    
    if mouse_button_input.pressed(MouseButton::Left) {
        if let Some(position) = window_query.single().cursor_position() {
            // Get player position.
            let player_x = player_transform.translation.x;
            let player_y = player_transform.translation.y;
            println!("{:?}", player_transform.translation);

            // Calculate distance between cursor and player.
            //let cursor_distance: f32 = sqrtf(powf(player_x - position.x, 2.0) + powf(player_y - position.y, 2.0));
            
            // Calculate difference between player position and mouse position.
            let diff_x = position.x - player_x;
            let diff_y = position.y - (player_y).abs();

            // Calculate angle between mouse and player.
            //let angle = diff_x / sqrtf(powf(diff_x, 2.0) + powf(diff_y, 2.0));
            let angle = atan2f(diff_x, diff_y);

            // Calculate delta x and delta y for bullet.
            let bullet_dx = sinf(angle);
            let bullet_dy = -cosf(angle);

            // Spawn bullet.
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: Mesh2dHandle(meshes.add(Rectangle::new(5.0, 5.0))),
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

fn update_bullets(
    mut bullet_query: Query<(&mut Transform, &Projectile), With<Bullet>>,
    time: Res<Time>,
) {
    for (mut transform, projectile) in &mut bullet_query {
        transform.translation.x += projectile.dx * projectile.speed * time.delta_seconds();
        transform.translation.y += projectile.dy * projectile.speed * time.delta_seconds();
    }
}