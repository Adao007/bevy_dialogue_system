use bevy::{
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};

pub struct CursorPlugin;
impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Cursor>()
            .add_systems(Startup, init_cursor_properties)
            .add_systems(Update, update_cursor_locking);
    }
}

#[derive(Resource, Default)]
pub struct Cursor {
    locked: bool,
}

impl Cursor {
    pub fn invert_lock(
        &mut self,
        windows: &mut Query<&mut Window, With<PrimaryWindow>>
    ) {
        self.locked = !self.locked;
        if let Ok(mut window) = windows.single_mut() {
            window.cursor_options.visible = !self.locked;
            if self.locked {
                let window_height = window.height();
                let window_width = window.width();
                window.cursor_options.grab_mode = CursorGrabMode::Locked; // Works for Windows, use CursorGrabMode::Locked for else
                window.set_cursor_position(Some(Vec2::new(window_width / 2.0, window_height / 2.0)));
            }
            else {
                window.cursor_options.grab_mode = CursorGrabMode::None;
            }
        }
    }
}

fn init_cursor_properties(
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
    mut cursor: ResMut<Cursor>,
) {
    cursor.invert_lock(&mut window_query);
}

fn update_cursor_locking(
    keys: Res<ButtonInput<KeyCode>>,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
    mut cursor: ResMut<Cursor>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        cursor.invert_lock(&mut window_query);
    }
}