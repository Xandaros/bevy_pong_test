use bevy::{prelude::*, sprite::collide_aabb::collide, window::WindowMode};

const WINDOW_WIDTH: f32 = 1920.0;
const WINDOW_HEIGHT: f32 = 1080.0;

enum Side {
    Left,
    Right,
}

#[derive(Default)]
struct Scores {
    left: u16,
    right: u16,
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
        width: 1920.0,
        height: 1080.0,
        ..Default::default()
    };

    App::build()
        .insert_resource(window_descriptor)
        .insert_resource(Scores::default())
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(ball_wall_bounce_system.system())
        .add_system(ball_paddle_bounce_system.system())
        .add_system(ball_move_system.system())
        .add_system(paddle_move_system.system())
        .add_system(scoreboard_system.system())
        .add_system(score_system.system())
        .run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut windows: ResMut<Windows>,
    asset_server: Res<AssetServer>,
    ) {
    windows.get_primary_mut().unwrap().update_scale_factor_from_backend(1.0);
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            transform: Transform::from_xyz(-WINDOW_WIDTH / 2.0 + 35.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(30.0, 240.0)),
            ..Default::default()
        })
        .insert(Paddle {side: Side::Left});
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            transform: Transform::from_xyz(WINDOW_WIDTH / 2.0 - 35.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(30.0, 240.0)),
            ..Default::default()
        })
        .insert(Paddle {side: Side::Right});
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(8.0, 8.0)),
            ..Default::default()
        })
        .insert(Ball {velocity: Vec2::new(256.0, 256.0)});
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(0.0),
                    left: Val::Px(0.0),
                    ..Default::default()
                },
                flex_direction: FlexDirection::Row,
                size: Size::new(Val::Percent(100.0), Val::Undefined),
                justify_content: JustifyContent::SpaceAround,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            visible: Visible {
                is_visible: false,
                is_transparent: false,
            },
            ..Default::default()
        }).with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    text: Text {
                        sections: vec![
                            TextSection {
                                value: "0".to_string(),
                                style: TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 40.0,
                                    color: Color::rgb(1.0, 1.0, 1.0),
                                }
                            }
                        ],
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Side::Left);
            parent.spawn_bundle(TextBundle {
                    text: Text {
                        sections: vec![
                            TextSection {
                                value: "0".to_string(),
                                style: TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 40.0,
                                    color: Color::rgb(1.0, 1.0, 1.0),
                                }
                            }
                        ],
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Side::Right);
        });
}

fn score_system(
    mut scores: ResMut<Scores>,
    mut query: Query<(&mut Ball, &mut Transform)>,
) {
    for (mut ball, mut transform) in query.iter_mut() {
        if transform.translation.x > WINDOW_WIDTH / 2.0 {
            // Left scored
            ball.velocity = Vec2::new(256.0, 256.0);
            transform.translation = Vec3::ZERO;
            scores.left += 1;
        } else if transform.translation.x < -WINDOW_WIDTH / 2.0 {
            // Right scored
            ball.velocity = Vec2::new(256.0, 256.0);
            transform.translation = Vec3::ZERO;
            scores.right += 1;
        }
    }
}

fn paddle_move_system(
    time: Res<Time>,
    mut query: Query<(&Paddle, &mut Transform)>,
    keyboard: Res<Input<KeyCode>>,
) {
    for (paddle, mut transform) in query.iter_mut() {
        let mut movement = 0.0;
        match paddle.side {
            Side::Left => {
                if keyboard.pressed(KeyCode::W) {
                    movement += 1.0;
                }
                if keyboard.pressed(KeyCode::S) {
                    movement -= 1.0;
                }
            },
            Side::Right => {
                if keyboard.pressed(KeyCode::Up) {
                    movement += 1.0;
                }
                if keyboard.pressed(KeyCode::Down) {
                    movement -= 1.0;
                }
            }
        }
        transform.translation.y += 256.0 * movement * time.delta_seconds();
        transform.translation.y = transform.translation.y.clamp(-WINDOW_HEIGHT / 2.0, WINDOW_HEIGHT / 2.0);
    }
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
        if transform.translation.y < -WINDOW_HEIGHT / 2.0 {
            transform.translation.y = -WINDOW_HEIGHT / 2.0;
            if ball.velocity.y < 0.0 {
                ball.velocity.y = -ball.velocity.y;
            }
        }
        if transform.translation.y > WINDOW_HEIGHT / 2.0 {
            transform.translation.y = WINDOW_HEIGHT / 2.0;
            if ball.velocity.y > 0.0 {
                ball.velocity.y = -ball.velocity.y;
            }
        }
    }
}

fn ball_paddle_bounce_system(
    mut ball_query: Query<(&mut Ball, &Sprite, &Transform)>,
    paddle_query: Query<(&Paddle, &Sprite, &Transform)>,
) {
    for (mut ball, ball_sprite, ball_transform) in ball_query.iter_mut() {
        for (paddle, paddle_sprite, paddle_transform) in paddle_query.iter() {
            if collide(ball_transform.translation, ball_sprite.size, paddle_transform.translation, paddle_sprite.size).is_some() {
                match paddle.side {
                    Side::Left => {
                        if ball.velocity.x < 0.0 {
                            ball.velocity.x = -ball.velocity.x;
                        }
                    },
                    Side::Right => {
                        if ball.velocity.x > 0.0 {
                            ball.velocity.x = -ball.velocity.x;
                        }
                    }
                }
            }
        }
    }
}

fn scoreboard_system(
    scores: Res<Scores>,
    mut query: Query<(&mut Text, &Side)>,
) {
    for (mut text, side) in query.iter_mut() {
        match side {
            Side::Left => {
                text.sections[0].value = format!("{}", scores.left);
            },
            Side::Right => {
                text.sections[0].value = format!("{}", scores.right);
            }
        }
    }
}
