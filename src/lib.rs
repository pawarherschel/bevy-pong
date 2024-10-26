mod ball;
mod enemy;
mod last_mouse_position;
mod player;
mod resolution;
mod score;
mod velocity;
mod zaphkiel;

use crate::resolution::RESOLUTION;
use bevy::prelude::{
    App, Camera2dBundle, Commands, DefaultPlugins, PluginGroup, Startup, Vec2, Window, WindowPlugin,
};

pub struct Pong;

impl bevy::prelude::Plugin for Pong {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: String::from("Pong!"),
                    resolution: Vec2 {
                        x: RESOLUTION.x,
                        y: RESOLUTION.y,
                    }
                    .into(),
                    ..Default::default()
                }),
                ..Default::default()
            }),
        )
        .add_systems(Startup, setup_camera)
        .add_plugins(velocity::Plugin)
        .add_plugins(player::Plugin)
        .add_plugins(resolution::Plugin)
        .add_plugins(last_mouse_position::Plugin)
        .add_plugins(ball::Plugin)
        .add_plugins(score::Plugin)
        .add_plugins(enemy::Plugin);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
