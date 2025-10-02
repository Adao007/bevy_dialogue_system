use bevy::{
    prelude::*,
    window::PrimaryWindow,
};
use super::camera_controller;
use avian3d::prelude::*;
use crate::game::shooting;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(shooting::tracer::TracerPlugin)
            .add_systems(Startup, init_player)
            .add_systems(Update, (update_player, camera_controller::update_camera_controller));
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

fn update_player(
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut commands: Commands,
    spatial_query: SpatialQuery,
    mut player_query: Query<(&mut Player, &mut Transform, &mut GlobalTransform, &mut Camera)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let window = window_query.single().unwrap();

    if let Ok((_player, transform, global_transform, camera)) = player_query.single_mut() {
        if mouse_input.just_pressed(MouseButton::Left) {
            if let Ok(ray) = camera.viewport_to_world(
                &global_transform,
                Vec2::new(window.width() / 2.0, window.height() / 2.0),
            ) {
                let hit = spatial_query.cast_ray(
                    ray.origin,
                    ray.direction,
                    f32::MAX,
                    true,
                    &SpatialQueryFilter::default(),
                );

                if let Some(hit) = hit {
                    let tracer = StandardMaterial {
                        base_color: Color::srgb(1.0, 1.0, 0.0),
                        unlit: true,
                        ..default()
                    };

                    commands.spawn((
                        Transform::from_translation(Vec3::splat(f32::MAX)),
                        Mesh3d(meshes.add(Cuboid::from_size(Vec3::new(0.1, 0.1, 1.0)))),
                        MeshMaterial3d(materials.add(tracer)),
                        shooting::tracer::BulletTracer::new(
                            transform.translation,
                            hit.normal,
                            100.0,
                        )
                        ));
                }

            }
        }
        else {
            return;
        }
    }
}
