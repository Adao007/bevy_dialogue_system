use super::{cursor::cursor, level::level, player::player, ui::ui, window::window};
use bevy::prelude::*;
use bevy_rapier3d::plugin::{NoUserData, RapierPhysicsPlugin};

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            RapierPhysicsPlugin::<NoUserData>::default(),
            level::LevelPlugin,
            player::PlayerPlugin,
            window::WindowSettingsPlugin,
            cursor::CursorPlugin,
            ui::UiPlugin,
        ));
    }
}
