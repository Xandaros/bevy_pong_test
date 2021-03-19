use bevy::{window::WindowMode, prelude::*};

enum Side {
    Left,
    Right,
}

struct Paddle {
    side: Side
}

struct Ball {
    velocity: Vec2,
}

fn main() {
    let window_descriptor = WindowDescriptor {
        title: "Pong".to_string(),
        mode: WindowMode::Fullscreen {use_size: false},
        ..Default::default()
    };

    App::build()
        .insert_resource(window_descriptor)
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(ball_wall_bounce_system.system())
        .add_system(ball_move_system.system())
        .run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
    commands
        .spawn({
            let mut bundle = OrthographicCameraBundle::new_2d();
            bundle.transform.scale = Vec3::new(1.1, 1.1, 1.0);
            bundle
        })
        .spawn(UiCameraBundle::default());
    commands
        .spawn(SpriteBundle {
            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            transform: Transform::from_xyz(-512.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(30.0, 120.0)),
            ..Default::default()
        })
        .with(Paddle {side: Side::Left});
    commands
        .spawn(SpriteBundle {
            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            transform: Transform::from_xyz(512.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(30.0, 120.0)),
            ..Default::default()
        })
        .with(Paddle {side: Side::Right});
    commands
        .spawn(SpriteBundle {
            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(8.0, 8.0)),
            ..Default::default()
        })
        .with(Ball {velocity: Vec2::new(128.0, 128.0)});
}

fn ball_move_system(
    time: Res<Time>,
    mut query: Query<(&Ball, &mut Transform)>,
) {
    let elapsed = f32::min(0.2, time.delta_seconds());
    for (ball, mut transform) in query.iter_mut() {
        transform.translation.x += ball.velocity.x * elapsed;
        transform.translation.y += ball.velocity.y * elapsed;
    }
}

fn ball_wall_bounce_system(
    mut query: Query<(&mut Ball, &mut Transform)>,
) {
    for (mut ball, mut transform) in query.iter_mut() {
        if transform.translation.y < -360.0 {
            transform.translation.y = -360.0;
            if ball.velocity.y < 0.0 {
                ball.velocity.y = -ball.velocity.y;
            }
        }
        if transform.translation.y > 360.0 {
            transform.translation.y = 360.0;
            if ball.velocity.y > 0.0 {
                ball.velocity.y = -ball.velocity.y;
            }
        }
    }
}
