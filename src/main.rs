use bevy::prelude::*; 

pub mod game; 
pub mod player; 

fn main() {
    App::new()
        .add_plugins((DefaultPlugins,
            game::game::GamePlugin,
            player::player::PlayerPlugin,
        ))
        .run(); 
}
