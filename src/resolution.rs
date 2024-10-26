use bevy::prelude::{App, Resource};

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(RESOLUTION);
    }
}

#[derive(Default, Debug, Resource, Copy, Clone)]
pub struct Resolution {
    pub x: f32,
    pub y: f32,
}

pub const RESOLUTION: Resolution = Resolution {
    x: 1024.0,
    y: 480.0,
};

pub const HALF_RESOLUTION: Resolution = {
    const HALF_X: f32 = RESOLUTION.x / 2.0;
    const HALF_Y: f32 = RESOLUTION.y / 2.0;

    Resolution {
        x: HALF_X,
        y: HALF_Y,
    }
};
