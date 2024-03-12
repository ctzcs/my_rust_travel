use bevy::prelude::{Camera2dBundle, Commands, Component, default, Transform};

//相机模组
#[derive(Component)]
struct MyCameraMarker;
fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        MyCameraMarker,
    ));
}