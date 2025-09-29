use bevy::prelude::*; 
use avian3d::prelude::*; 

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            PhysicsPlugins::default(),
        ))
        .add_systems(Startup, setup_level); 
    }
}

fn setup_level(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawns the level's floor
    commands.spawn((
        RigidBody::Static,
        Collider::cuboid(1000.0, 0.0, 1000.0),
        Mesh3d(meshes.add(Cuboid::new(1000.0, 0.0, 1000.0))), 
        MeshMaterial3d(materials.add(Color::WHITE)), 
        Transform::IDENTITY,
    )); 

    // Spawn a Wall
    commands.spawn((
        RigidBody::Static,
        Collider::cuboid(30.0, 30.0, 30.0), 
        Mesh3d(meshes.add(Cuboid::new(30.0, 30.0, 30.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_xyz(0.0, 0.0, -100.0),
    ));

    // Spawn light 
    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true, 
            ..default()
        }, 
        Transform::from_xyz(100.0, 200.0, 100.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}