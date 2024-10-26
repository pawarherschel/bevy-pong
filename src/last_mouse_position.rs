use bevy::prelude::{App, Deref, DerefMut, PreUpdate, Query, ResMut, Resource, Vec2, Window, With};
use bevy::window::PrimaryWindow;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LastMousePosition::default())
            .add_systems(PreUpdate, pre_update);
    }
}

#[derive(Default, Debug, Resource, Clone, Copy, Deref, DerefMut)]
pub struct LastMousePosition(Vec2);

#[allow(clippy::needless_pass_by_value)]
fn pre_update(
    window: Query<&Window, With<PrimaryWindow>>,
    mut last_mouse_position: ResMut<LastMousePosition>,
) {
    let mouse = window
        .get_single()
        .map(Window::cursor_position)
        .unwrap_or(Some(**last_mouse_position))
        .unwrap_or(**last_mouse_position);

    **last_mouse_position = mouse;
}
