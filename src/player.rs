use crate::last_mouse_position::LastMousePosition;
use crate::remap;
use crate::resolution::{HALF_RESOLUTION, RESOLUTION};
use crate::zaphkiel::{exponential_decay, DECAY};
use bevy::prelude::{
    App, Assets, Capsule2d, Color, ColorMaterial, ColorMesh2dBundle, Commands, Component,
    IntoSystemConfigs, Mesh, Query, Res, ResMut, Startup, Time, Transform, Update, Vec3, With,
};
use bevy::sprite::Mesh2dHandle;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, (handle_input, restrict).chain());
    }
}

#[derive(Component)]
pub struct Player;

impl Player {
    pub const RADIUS: f32 = 5.0;
    pub const LENGTH: f32 = 45.0;
    pub const PADDING: f32 = 3.0;
    const COLOR: Color = Color::WHITE;
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let capsule = Capsule2d::new(Player::RADIUS, Player::LENGTH);
    let handle = meshes.add(capsule);
    let mesh = Mesh2dHandle(handle);

    let material = ColorMaterial::from_color(Player::COLOR);
    let material = materials.add(material);

    let transform = Transform::from_translation(Vec3::new(
        -HALF_RESOLUTION.x + Player::RADIUS + Player::PADDING,
        0.0,
        0.0,
    ));

    commands.spawn((
        Player,
        ColorMesh2dBundle {
            mesh,
            material,
            transform,
            ..Default::default()
        },
    ));
}

#[allow(clippy::needless_pass_by_value)]
fn handle_input(
    last_mouse_position: Res<LastMousePosition>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let mouse_y = last_mouse_position.y;

    let mouse_y = remap! {
        value: mouse_y,
        from: 0.0, RESOLUTION.y,
        to: HALF_RESOLUTION.y, -HALF_RESOLUTION.y
    };

    let mut tf = query
        .get_single_mut()
        .expect("only one player should exist in the game");

    tf.translation.y = exponential_decay(tf.translation.y, mouse_y, DECAY, time.delta_seconds());
}

const BOUNDS: (Vec3, Vec3) = (
    Vec3::new(
        (Player::RADIUS + Player::PADDING) - HALF_RESOLUTION.x,
        (Player::LENGTH / 2.0 + Player::RADIUS + Player::PADDING) - HALF_RESOLUTION.y,
        0.0,
    ),
    Vec3::new(
        (Player::RADIUS + Player::PADDING) - HALF_RESOLUTION.x,
        HALF_RESOLUTION.y - (Player::LENGTH / 2.0 + Player::RADIUS + Player::PADDING),
        0.0,
    ),
);

fn restrict(mut query: Query<&mut Transform, With<Player>>) {
    let mut tf = query
        .get_single_mut()
        .expect("only one player should exist in the game");

    let (min, max) = BOUNDS;

    tf.translation = tf.translation.clamp(min, max);
}
