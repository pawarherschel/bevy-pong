use crate::ball::Ball;
use crate::resolution::HALF_RESOLUTION;
use crate::zaphkiel::{exponential_decay, DECAY};
use bevy::prelude::{
    App, Assets, Capsule2d, Color, ColorMaterial, ColorMesh2dBundle, Commands, Component, Deref,
    DerefMut, Mesh, PostUpdate, Query, Res, ResMut, Resource, Startup, Time, Transform, Update,
    Vec3, With, Without,
};
use bevy::sprite::Mesh2dHandle;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LastDirection::default())
            .add_systems(Startup, startup)
            .add_systems(Update, update)
            .add_systems(PostUpdate, restrict);
    }
}

#[derive(Resource, Deref, DerefMut, Default)]
struct LastDirection(Option<f32>);

#[derive(Component)]
pub struct Enemy;

impl Enemy {
    pub const RADIUS: f32 = 5.0;
    pub const LENGTH: f32 = 45.0;
    pub const PADDING: f32 = 3.0;
    pub const MAX_SPEED: f32 = 40.0;
    pub const MIN_SPEED: f32 = 2.0;
    const COLOR: Color = Color::WHITE;
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut color_material: ResMut<Assets<ColorMaterial>>,
) {
    let capsule = Capsule2d::new(Enemy::RADIUS, Enemy::LENGTH);
    let handle = meshes.add(capsule);
    let mesh = Mesh2dHandle(handle);

    let material = color_material.add(Enemy::COLOR);

    let transform = Transform::from_translation(Vec3::new(
        HALF_RESOLUTION.x - (Enemy::RADIUS + Enemy::PADDING),
        0.0,
        0.0,
    ));

    commands.spawn((
        Enemy,
        ColorMesh2dBundle {
            mesh,
            material,
            transform,
            ..Default::default()
        },
    ));
}

#[allow(clippy::needless_pass_by_value)]
fn update(
    ball: Query<&Transform, (With<Ball>, Without<Enemy>)>,
    mut query: Query<&mut Transform, (With<Enemy>, Without<Ball>)>,
    mut last_direction: ResMut<LastDirection>,
    time: Res<Time>,
) {
    let ball = ball
        .get_single()
        .expect("only one ball should exist")
        .translation;

    let mut tf = query
        .get_single_mut()
        .expect("only one enemy should exist in the game");

    let ball_x = ball.x;
    let ball_y = ball.y;

    let target = if ball_x >= 0.0 {
        **last_direction = None;
        ball_y
    } else {
        last_direction.unwrap_or_else(|| {
            let dir = tf.translation.y.signum() * -1.0;
            **last_direction = Some(dir);
            dir
        }) * HALF_RESOLUTION.y
    };

    let speed = tf.translation.y
        - exponential_decay(tf.translation.y, target, DECAY * 0.5, time.delta_seconds());

    let sign = speed.signum();

    let speed = speed.abs();

    let speed = speed.clamp(Enemy::MIN_SPEED, Enemy::MAX_SPEED);

    tf.translation.y -= speed * sign;
}

const BOUNDS: (Vec3, Vec3) = (
    Vec3::new(
        HALF_RESOLUTION.x - (Enemy::RADIUS + Enemy::PADDING),
        (Enemy::LENGTH / 2.0 + Enemy::RADIUS + Enemy::PADDING) - HALF_RESOLUTION.y,
        0.0,
    ),
    Vec3::new(
        HALF_RESOLUTION.x - (Enemy::RADIUS + Enemy::PADDING),
        HALF_RESOLUTION.y - (Enemy::LENGTH / 2.0 + Enemy::RADIUS + Enemy::PADDING),
        0.0,
    ),
);

fn restrict(mut query: Query<&mut Transform, With<Enemy>>) {
    let mut tf = query
        .get_single_mut()
        .expect("only one enemy should exist in the game");

    let (min, max) = BOUNDS;

    tf.translation = tf.translation.clamp(min, max);
}
