use crate::enemy::Enemy;
use crate::player::Player;
use crate::remap;
use crate::resolution::HALF_RESOLUTION;
use crate::score::CreateScore;
use crate::score::Scorer;
use crate::velocity::Velocity;
use bevy::math::bounding::{Aabb2d, IntersectsVolume};
use bevy::prelude::{
    info, App, Assets, Circle, Color, ColorMaterial, ColorMesh2dBundle, Commands, Component,
    Entity, Event, EventReader, EventWriter, IntoSystemConfigs, Mesh, Or, PostUpdate, Query,
    ResMut, Startup, Transform, Update, Vec2, Vec3, Vec3Swizzles, With, Without,
};
use bevy::sprite::Mesh2dHandle;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Flip>()
            .add_event::<Respawn>()
            .add_systems(Startup, startup)
            .add_systems(Update, update)
            .add_systems(
                PostUpdate,
                ((paddle_collision, flip_handler).chain(), respawn_handler),
            );
    }
}

#[derive(Component, Default)]
pub struct Ball;

impl Ball {
    pub const RADIUS: f32 = 5.0;
    const COLOR: Color = Color::WHITE;
    const SPEED: f32 = 15.0;
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let circle = Circle::new(Ball::RADIUS);
    let handle = meshes.add(circle);
    let mesh = Mesh2dHandle(handle);

    let material = materials.add(Ball::COLOR);

    let velocity = crand();

    commands.spawn((
        Ball,
        ColorMesh2dBundle {
            mesh,
            material,
            ..Default::default()
        },
        Velocity(velocity),
    ));
}

fn crand() -> Vec2 {
    let time = std::time::UNIX_EPOCH
        .elapsed()
        .expect("time should not go backwards from UNIX_EPOCH");

    let angle = time.as_secs_f32();

    let (y, x) = angle.sin_cos();

    let ys = y.signum();
    let xs = x.signum();

    let Vec2 { x, y } = Vec2::new(x, y).normalize();

    let ny = remap! {
        value: y,
        from: -1.0, 1.0,
        to: 0.10, 0.35
    };
    let nx = remap! {
        value: x,
        from: -1.0, 1.0,
        to: 0.10, 0.35
    };

    let x = xs * nx;
    let y = ys * ny;

    Vec2::new(x, y).normalize() * Ball::SPEED
}

const PADDING: f32 = 3.0;

const BOUNDS: (Vec3, Vec3) = (
    Vec3::new(
        (Ball::RADIUS + PADDING + Ball::RADIUS) - HALF_RESOLUTION.x,
        (Ball::RADIUS + PADDING) - HALF_RESOLUTION.y,
        0.0,
    ),
    Vec3::new(
        HALF_RESOLUTION.x - (Ball::RADIUS + PADDING + Ball::RADIUS),
        HALF_RESOLUTION.y - (Ball::RADIUS + PADDING),
        0.0,
    ),
);

fn update(
    mut query: Query<(Entity, &mut Transform), With<Ball>>,
    mut flip: EventWriter<Flip>,
    mut create_score: EventWriter<CreateScore>,
) {
    let (min, max) = BOUNDS;

    let (entity, mut tf) = query.get_single_mut().expect("only one ball should exist");

    let Transform {
        translation: Vec3 { x, y, .. },
        ..
    } = *tf;

    tf.translation.y = y.clamp(min.y, max.y);

    let y_flag = y < min.y || y > max.y;

    flip.send(Flip {
        entity,
        x: false,
        y: y_flag,
    });

    let enemy_score = x <= min.x;
    let player_score = x >= max.x;

    if enemy_score {
        create_score.send(CreateScore {
            scorer: Scorer::Enemy,
        });
    }

    if player_score {
        create_score.send(CreateScore {
            scorer: Scorer::Player,
        });
    }
}

#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::type_complexity)]
fn paddle_collision(
    ball: Query<(Entity, &Transform), (With<Ball>, (Without<Player>, Without<Enemy>))>,
    paddles: Query<&Transform, (Or<(With<Player>, With<Enemy>)>, Without<Ball>)>,
    mut flip: EventWriter<Flip>,
) {
    let (entity, ball) = ball.get_single().expect("only one ball should exist");

    if paddles.iter().any(|paddle| {
        check_collision(
            ball.translation.xy(),
            Vec2::splat(Ball::RADIUS),
            paddle.translation.xy(),
            Vec2 {
                x: Player::RADIUS,
                y: Player::LENGTH / 2.0,
            },
        )
    }) {
        info!("bonk!");

        flip.send(Flip {
            entity,
            x: true,
            y: false,
        });
    }
}

fn check_collision(
    a_position: Vec2,
    a_half_extends: Vec2,
    b_position: Vec2,
    b_half_extends: Vec2,
) -> bool {
    let a = Aabb2d::new(a_position, a_half_extends);
    let b = Aabb2d::new(b_position, b_half_extends);

    a.intersects(&b)
}

#[derive(Event, Debug)]
struct Flip {
    entity: Entity,
    x: bool,
    y: bool,
}

fn flip_handler(mut flips: EventReader<Flip>, mut query: Query<&mut Velocity, With<Ball>>) {
    if flips.is_empty() {
        return;
    }

    for &Flip { entity, x, y } in flips.read() {
        let mut velocity = query.get_mut(entity).expect("entity should exist");
        if x {
            velocity.x *= -1.0;
        }
        if y {
            velocity.y *= -1.0;
        }
    }
}

#[derive(Event, Debug)]
pub struct Respawn;

fn respawn_handler(
    mut respawn: EventReader<Respawn>,
    mut query: Query<(&mut Transform, &mut Velocity), With<Ball>>,
) {
    if respawn.is_empty() {
        return;
    }

    respawn.clear();

    let (mut tf, mut v) = query
        .get_single_mut()
        .expect("only one ball should be present");

    tf.translation = Vec3::default();

    **v = crand();
}
