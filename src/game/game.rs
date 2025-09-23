use bevy::prelude::*; 
use avian3d::prelude::*; 

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            PhysicsPlugins::default(),
        )); 
    }
}