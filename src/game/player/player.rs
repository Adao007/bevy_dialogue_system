use super::camera_controller;
use crate::game::{
    level::target::{DeadTarget, Target},
    shooting,
};
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier3d::{plugin::ReadRapierContext, prelude::QueryFilter};

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(shooting::tracer::TracerPlugin)
            .add_systems(Startup, init_player)
            .add_systems(
                Update,
                (update_player, camera_controller::update_camera_controller),
            );
    }
}

#[derive(Component)]
pub struct Player;

fn init_player(mut commands: Commands) {
    let fov = 103.0_f32.to_radians();

    commands.spawn((
        Player,
        Camera3d::default(),
        camera_controller::CameraController {
            sensitivity: 0.035,
            rotation: Vec2::ZERO,
            rotation_lock: 88.0,
        },
        Projection::Perspective(PerspectiveProjection {
            fov: fov,
            ..default()
        }),
        Transform::from_xyz(0.0, 10.0, 0.0),
    ));
}

fn update_player(
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut commands: Commands,
    rapier_context: ReadRapierContext,
    mut player_query: Query<(
        &mut Player,
        &mut Transform,
        &mut GlobalTransform,
        &mut Camera,
    )>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    target_query: Query<Entity, With<Target>>,
) {
    let window = window_query.single().unwrap();
    let context = rapier_context.single().unwrap();

    if let Ok((_player, transform, global_transform, camera)) = player_query.single_mut() {
        if mouse_input.just_pressed(MouseButton::Left) {
            if let Ok(ray) = camera.viewport_to_world(
                &global_transform,
                Vec2::new(window.width() / 2.0, window.height() / 2.0),
            ) {
                let hit = context.cast_ray_and_get_normal(
                    ray.origin,
                    ray.direction.into(),
                    f32::MAX,
                    true,
                    QueryFilter::default(),
                );

                if let Some((entity, ray_intersection)) = hit {
                    if let Ok(_entity) = target_query.get(entity) {
                        commands.entity(entity).insert(DeadTarget);
                    }
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
                            ray_intersection.point,
                            100.0,
                        ),
                    ));
                }
            }
        } else {
            return;
        }
    }
}
