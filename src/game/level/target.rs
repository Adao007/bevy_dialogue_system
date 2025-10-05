use bevy::prelude::*;

pub struct TargetPlugin;
impl Plugin for TargetPlugin {
    fn build(&self, app: &mut App) {}
}

#[derive(Component)]
pub struct Target;

#[derive(Component)]
pub struct DeadTarget;

#[derive(Resource, Clone, Copy)]
pub struct GridShot {
    pub grid_size: i32,
    pub cell_size: f32,
    pub max_targets: i32,
}

impl GridShot {
    pub fn generate_new_position(&self, rand: &mut ThreadRng) -> Vec2 {
        return (Vec2::new(
            rand.gen_range(0..self.grid_size) as f32,
            rand.gen_range(0..self.grid_size) as f32,
        ) - Vec2::new(self.grid_size as f32 / 2.0, 0.0)
            + (Vec2::Y * 0.5))
            * self.cell_size;
    }
}

fn init_grid_shot(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>, 
) {
    let grid_shot = GridShot{
        grid_size: 5,
        cell_size: 5.0,
        max_targets: 5, 
    };

    let target_material = materials.add(StandardMaterial) {
        base_color: Color::srpg(1.0, 0.0, 0.0),
        ..default()
    });

    commands.insert_resource(grid_shot);

    let target_radius = grid_shot.cell_size / 8.0;

    let collider_radius = target_radius * f32::sin(PI / 4.0);

    
}
