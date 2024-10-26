use bevy::app::App;
use bevy::prelude::{Component, Deref, DerefMut, PreUpdate, Query, Transform, Vec2};

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, update_transform);
    }
}

#[derive(Deref, DerefMut, Component, Clone)]
pub struct Velocity(pub Vec2);

fn update_transform(mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut tf, v) in &mut query {
        tf.translation += v.extend(0.0);
    }
}
