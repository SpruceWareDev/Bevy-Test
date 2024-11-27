use bevy::{
    prelude::*, window::PrimaryWindow
};

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, (init, bevy_test::game::init).chain())
    .add_systems(Update, 
        (
            bevy_test::game::move_player, 
            bevy_test::game::shoot, 
            bevy_test::game::update_bullets
        )
    )
    .run();
}

fn init(
    mut commands: Commands,
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
}