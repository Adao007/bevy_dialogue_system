use bevy::prelude::*; 

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, init_player); 
    }
}

#[derive(Component)]
pub struct Player; 

fn init_player(
    mut commands: Commands
) {
    commands.spawn((
        Player,
        Camera3d::default(),
        Transform::from_xyz(0.0, 10.0, 0.0),
    ));
}