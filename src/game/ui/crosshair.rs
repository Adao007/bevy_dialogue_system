use bevy::{
    prelude::*,
    window::PrimaryWindow,
};

// This node bundle is like a canvas that we can print on!
pub fn spawn_crosshair(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
) {
    let crosshair_size = 2.0;
    if let Ok(window) = window_query.single() {
        commands.spawn(
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        ImageNode::solid_color(Color::srgb(1.0, 1.0, 1.0)),
                        Node {
                            width: Val::Px(crosshair_size),
                            height: Val::Px(crosshair_size),
                            left: Val::Px(window.width() / 2. - (crosshair_size / 2.)),
                            top: Val::Px(window.height() / 2. - (crosshair_size / 2.)),
                            ..default()
                        }
                    ));
                });
    }
}