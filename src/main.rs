use bevy::math::bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume};
use bevy::prelude::*;

const GUTTER_HEIGHT: f32 = 20.;

#[derive(Component)]
#[require(Position, Shape)]
struct Gutter;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Ai;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Collision {
    Left,
    Right,
    Top,
    Bottom,
}

#[derive(Component, Default)]
struct Velocity(Vec2);

#[derive(Component, Default)]
#[require(Transform)]
struct Position(Vec2);

#[derive(Component, Default)]
struct Shape(Vec2);

#[derive(Component)]
#[require(
    Position,
    Velocity(|| Velocity(Vec2::new(-1., 0.))),
    Shape(|| Shape(Vec2::new(BALL_SIZE, BALL_SIZE))),
)]
struct Ball;

#[derive(Component)]
#[require(
    Position,
    Shape(|| Shape(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT))),
    Velocity
)]
struct Paddle;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(
            Startup,
            (spawn_ball, spawn_camera, spawn_paddles, spawn_gutters),
        )
        .add_systems(
            Update,
            (
                handle_player_input,
                move_ball.after(handle_player_input),
                move_paddles.after(handle_player_input),
                project_positions.after(move_ball).after(move_paddles),
                handle_collisions_gutter
                    .after(move_ball)
                    .after(move_paddles),
                handle_collisions_paddle
                    .after(move_ball)
                    .after(move_paddles),
            ),
        )
        .run();
}

const PADDLE_WIDTH: f32 = 10.;
const PADDLE_HEIGHT: f32 = 50.;

fn spawn_paddles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Query<&Window>,
) {
    println!("Spawning paddles...");

    if let Ok(window) = window.get_single() {
        let window_width = window.resolution.width();
        let padding = 50.;
        let right_paddle_x = window_width / 2. - padding;
        let left_paddle_x = -window_width / 2. + padding;

        let shape = Rectangle::new(PADDLE_WIDTH, PADDLE_HEIGHT);

        let mesh = meshes.add(shape);
        let player_color = materials.add(Color::srgb(0., 1., 0.));
        let ai_color = materials.add(Color::srgb(0., 0., 1.));

        commands.spawn((
            Player,
            Paddle,
            Position(Vec2::new(right_paddle_x, 0.)),
            Mesh2d(mesh.clone()),
            MeshMaterial2d(player_color.clone()),
        ));

        commands.spawn((
            Ai,
            Paddle,
            Position(Vec2::new(left_paddle_x, 0.)),
            Mesh2d(mesh.clone()),
            MeshMaterial2d(ai_color.clone()),
        ));
    }
}

const BALL_SIZE: f32 = 5.;

fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("Spawning ball...");

    let shape = Circle::new(BALL_SIZE);
    let color = Color::srgb(1., 0., 0.);

    // `Assets::add` will load these into memory and return a `Handle` (an ID)
    // to these assets. When all references to this `Handle` are cleaned up
    // the asset is cleaned up.
    let mesh = meshes.add(shape);
    let material = materials.add(color);

    // Here we are using `spawn` instead of `spawn_empty` followed by an
    // `insert`. They mean the same thing, letting us spawn many components on a
    // new entity at once.
    commands.spawn((Ball, Mesh2d(mesh), MeshMaterial2d(material)));
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_empty().insert(Camera2d);
}

fn project_positions(mut positionables: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in &mut positionables {
        transform.translation = position.0.extend(0.);
    }
}

const BALL_SPEED: f32 = 5.;

fn move_ball(mut ball: Query<(&mut Position, &Velocity), With<Ball>>) {
    if let Ok((mut position, velocity)) = ball.get_single_mut() {
        position.0 += velocity.0 * BALL_SPEED
    }
}

// Returns `Some` if `ball` collides with `wall`. The returned `Collision` is the
// side of `wall` that `ball` hit.
fn collide_with_side(ball: BoundingCircle, wall: Aabb2d) -> Option<Collision> {
    if !ball.intersects(&wall) {
        return None;
    }

    let closest = wall.closest_point(ball.center());
    let offset = ball.center() - closest;
    let side = if offset.x.abs() > offset.y.abs() {
        if offset.x < 0. {
            Collision::Left
        } else {
            Collision::Right
        }
    } else if offset.y > 0. {
        Collision::Top
    } else {
        Collision::Bottom
    };

    Some(side)
}

fn collide_with_paddle(ball: BoundingCircle, position: &Position, paddle: &Shape) -> Option<Vec2> {
    let aabb2d = Aabb2d::new(position.0, paddle.0 / 2.);
    if !ball.intersects(&aabb2d) {
        return None;
    }
    //initialize a variable for the x of the vector
    let x;
    //initialize a variable for the y of the vector
    let y;
    //take center of the ball
    let center = ball.center();
    //take center of the paddle
    let paddle_center = aabb2d.center();
    //if the ball is on the right side of the paddle set positive speed
    if center.x > paddle_center.x {
        x = 1.;
    } else {
        //if the ball is on the left side of the paddle set negative speed
        x = -1.;
    }
    //if the ball is on the top side of the paddle set speed proportional to the center distance
    if center.y > paddle_center.y {
        y = (center.y - paddle_center.y) / paddle.0.y / 2.;
    } else {
        //if the ball is on the bottom side of the paddle set speed proportional to the center distance
        y = -(center.y - paddle_center.y) / paddle.0.y / 2.;
    }
    return Some(Vec2::new(x, y));
}

fn handle_collisions_paddle(
    mut ball: Query<(&mut Velocity, &Position, &Shape), With<Ball>>,
    paddles: Query<(&Position, &Shape), With<Paddle>>,
) {
    if let Ok((mut ball_velocity, ball_position, ball_shape)) = ball.get_single_mut() {
        for (position, shape) in &paddles {
            if let Some(collision) = collide_with_paddle(
                BoundingCircle::new(ball_position.0, ball_shape.0.x),
                position,
                shape

            ) {
                ball_velocity.0 = collision;
            }
        }
    }
}

fn handle_collisions_gutter(
    mut ball: Query<(&mut Velocity, &Position, &Shape), With<Ball>>,
    gutters: Query<(&Position, &Shape), With<Gutter>>,
) {
    if let Ok((mut ball_velocity, ball_position, ball_shape)) = ball.get_single_mut() {
        for (position, shape) in &gutters {
            if let Some(collision) = collide_with_side(
                BoundingCircle::new(ball_position.0, ball_shape.0.x),
                Aabb2d::new(position.0, shape.0 / 2.),
            ) {
                match collision {
                    Collision::Left => {
                        ball_velocity.0.x *= -1.;
                    }
                    Collision::Right => {
                        ball_velocity.0.x *= -1.;
                    }
                    Collision::Top => {
                        ball_velocity.0.y *= -1.;
                    }
                    Collision::Bottom => {
                        ball_velocity.0.y *= -1.;
                    }
                }
            }
        }
    }
}

// The system that spawns the gutters
fn spawn_gutters(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Query<&Window>,
) {
    println!("Spawning gutters...");

    if let Ok(window) = window.get_single() {
        let window_height = window.resolution.height();
        let window_width = window.resolution.width();
        let gutter_color = Color::srgb(0., 0., 0.);

        let gutter_shape = Rectangle::new(window_width, GUTTER_HEIGHT);
        let mesh = meshes.add(gutter_shape);
        let material = materials.add(gutter_color);

        let gutter_y = window_height / 2. - GUTTER_HEIGHT / 2.;
        commands.spawn((
            Gutter,
            Position(Vec2::new(0., gutter_y)),
            Shape(Vec2::new(window_width, GUTTER_HEIGHT)),
            Mesh2d(mesh.clone()),
            MeshMaterial2d(material.clone()),
        ));

        let gutter_y = -window_height / 2. + GUTTER_HEIGHT / 2.;
        commands.spawn((
            Gutter,
            Position(Vec2::new(0., gutter_y)),
            Shape(Vec2::new(window_width, GUTTER_HEIGHT)),
            Mesh2d(mesh.clone()),
            MeshMaterial2d(material.clone()),
        ));

        let gutter_shape = Rectangle::new(GUTTER_HEIGHT, window_height);
        let mesh = meshes.add(gutter_shape);
        let material = materials.add(gutter_color);

        let gutter_x = window_width / 2. - GUTTER_HEIGHT / 2.;
        commands.spawn((
            Gutter,
            Position(Vec2::new(gutter_x, 0.)),
            Shape(Vec2::new(GUTTER_HEIGHT, window_height)),
            Mesh2d(mesh.clone()),
            MeshMaterial2d(material.clone()),
        ));

        let gutter_x = -window_width / 2. + GUTTER_HEIGHT / 2.;
        commands.spawn((
            Gutter,
            Position(Vec2::new(gutter_x, 0.)),
            Shape(Vec2::new(GUTTER_HEIGHT, window_height)),
            Mesh2d(mesh.clone()),
            MeshMaterial2d(material.clone()),
        ));
    }
}

const PADDLE_SPEED: f32 = 5.;

fn handle_player_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut paddle: Query<&mut Velocity, With<Player>>,
) {
    if let Ok(mut velocity) = paddle.get_single_mut() {
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            velocity.0.y = 1.;
        } else if keyboard_input.pressed(KeyCode::ArrowDown) {
            velocity.0.y = -1.;
        } else {
            velocity.0.y = 0.;
        }
    }
}

fn move_paddles(
    mut paddle: Query<(&mut Position, &Velocity), With<Paddle>>,
    window: Query<&Window>,
) {
    if let Ok(window) = window.get_single() {
        let window_height = window.resolution.height();
        let max_y = window_height / 2. - GUTTER_HEIGHT - PADDLE_HEIGHT / 2.;

        for (mut position, velocity) in &mut paddle {
            let new_position = position.0 + velocity.0 * PADDLE_SPEED;
            if new_position.y.abs() < max_y {
                position.0 = new_position;
            }
        }
    }
}
