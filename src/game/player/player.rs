use bevy::prelude::*; 
use super::camera_controller;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, init_player)
            .add_systems(Update, camera_controller::update_camera_controller);
    }
}

#[derive(Component)]
pub struct Player; 

fn init_player(
    mut commands: Commands
) {
    let fov = 103.0_f32.to_radians();

    commands.spawn((
        Player,
        Camera3d::default(),
        camera_controller::CameraController {
            sensitivity: 0.035,
            rotation: Vec2::ZERO,
            rotation_lock: 88.0,
        },
        Projection::Perspective(PerspectiveProjection { fov: fov, ..default()}),
        Transform::from_xyz(0.0, 10.0, 0.0),
    ));
}