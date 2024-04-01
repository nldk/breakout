use std::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use std::cmp::*;
use std::thread::Thread;

#[derive(Component)]
pub struct PLAYER;

#[derive(Component)]
pub struct BRICK;


#[derive(Component)]
pub struct BALL {
    dir: Vec3,
}

pub static mut ballSpawn: bool = true;
//tue
pub static mut bricksSpawn: bool = true; //true

pub static mut bricksLeft: u8 = 0; // 0

pub static mut playerspeed: f32 = 600.0;// 600.0

pub const playerSizeX: f32 = 129.0; // 129.30

pub const playerSizeY: f32 = 53.0; // 53.0

pub static mut ballSpeed: f32 = 1000.0; // 450.0

pub const ballSize: f32 = 35.0; // 35.0

// peddelSpace = the room between the bottom and the paddle
pub const peddelSpace: f32 = 2.0; // 2.0

pub const brikSizeX: f32 = 200.0; // 200.0

pub const brikSizeY: f32 = 60.0; // 60.0

pub const kanVerliezen: bool = false;

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawnPeddel, spawnCamera, spawnBall, ))
        .add_systems(Update, (movePlayer, confinePlayerMovement, moveBall, confineBallMovement, ballColitionMetPlayer, briksDisapear, winOrLose, win, spawnBriks, ))
        .run();
}

pub fn spawnPeddel(
    mut commands: Commands,
    assetServer: Res<AssetServer>,
    windowQuery: Query<&Window, With<PrimaryWindow>>,
) {
    let window = windowQuery.get_single().unwrap();

    commands.spawn((SpriteBundle {
        texture: assetServer.load("sprites/button_blue.png"),
        transform: Transform::from_xyz(0.0, window.height() / peddelSpace * -1.0, 0.0),
        ..default()
    }, PLAYER {},
                    AudioBundle {
                        source: assetServer.load("sounds/ethereal-vistas-191254.ogg"),
                        ..default()
                    },
                    MyMusic,
    ));
    /*commands.spawn(TextBundle::from_section(

        TextSection::new(
            "score: ",
            TextStyle{
                font_size: 60.0,
                ..default()
            }

        ),

    )

    );*/
}

pub fn spawnCamera(
    mut comands: Commands
) {
    comands.spawn(Camera2dBundle::default());
}

pub fn spawnBall(
    mut commands: Commands,
    assetServer: Res<AssetServer>,
) {
    commands.spawn((SpriteBundle {
        texture: assetServer.load("sprites/ball_red_large.png"),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        sprite: Sprite {
            custom_size: Some(Vec2::new(ballSize, ballSize)),
            ..default()
        },
        ..default()
    },
                    BALL { dir: Vec3::new(1.0, 1.0, 0.0) }));
}

pub fn spawnBriks(
    mut balltransform: Query<&mut Transform,With<BALL>>,
    mut commands: Commands,
    assetServer: Res<AssetServer>,
) {
    unsafe {
        if bricksSpawn {
            let mut transform = balltransform.single_mut();
            
            let Xreset: f32 = -760.0;
            let mut x: f32 = Xreset;
            let mut y: f32 = 150.0;
            for i in 1..6 {
                for v in 1..9 {
                    commands.spawn((SpriteBundle {
                        texture: assetServer.load("sprites/block_square.png"),
                        transform: Transform::from_xyz(x, y, 0.0),
                        visibility: Visibility::Visible,
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(brikSizeX, brikSizeY)),
                            ..default()
                        },
                        ..default()
                    }, BRICK {}
                    ));
                    x += brikSizeX + 20.0;
                    unsafe { bricksLeft += 1; }
                }
                x = Xreset;
                y += brikSizeY + 20.0;
            }
            bricksSpawn = false
        }
    }
    unsafe { println!("{}", bricksLeft) }
}


fn briksDisapear(
    assetServer: Res<AssetServer>,
    mut commands: Commands,
    mut bricks: Query<(Entity, &Transform), With<BRICK>>,
    ballT: Query<&Transform, With<BALL>>,
    mut ballQ: Query<&mut BALL>,
) {
    for brick in bricks.iter() {
        let balltransform = ballT.single();
        let bricktransform = brick.1.translation;
        let brickP = bricktransform;
        let ballP = balltransform.translation;
        let brickY = brickP.y;
        let brickX = brickP.x;
        let halfBallSize = ballSize / 2.0;
        let halfBrickSizeY = brikSizeY / 2.0;
        let halfBrickSizeX = brikSizeX / 2.0;
        let ballY = ballP.y;
        let ballX = ballP.x;
        let mut ball = ballQ.single_mut();

        let Ycolition = (halfBrickSizeY + halfBallSize) > ((ballY - brickY).abs());
        let Xcolition = (halfBallSize + halfBrickSizeX) > ((ballX - brickX).abs());
        if Ycolition && Xcolition {
            commands.entity(brick.0).despawn();
            ball.dir = Vec3::new(ball.dir.x, -ball.dir.y, ball.dir.z);
            unsafe {
                ballSpeed += 25.0;
                playerspeed += 35.0;
                bricksLeft = bricksLeft - 1;
                println!("{}", bricksLeft)
            }
            commands.spawn((
                AudioBundle {
                    source: assetServer.load("sounds/impactGlass_light_003.ogg"),
                    ..default()
                },
                MyMusic,
            ));
        }
    }
}

fn win() {
    unsafe {
        if bricksLeft == 0 {
            bricksSpawn = true;
        }
    }
}

#[derive(Component)]
struct MyMusic;

pub fn movePlayer(
    input: Res<ButtonInput<KeyCode>>,
    mut player: Query<&mut Transform, With<PLAYER>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player.get_single_mut() {
        //dir stands for direction
        let mut dir = Vec3::ZERO;
        //checking for input
        if input.pressed(KeyCode::ArrowLeft) {
            dir += Vec3::new(-1.0, 0.0, 0.0);
        } else if input.pressed(KeyCode::ArrowRight) {
            dir += Vec3::new(1.0, 0.0, 0.0);
        }

        if dir.length() > 0.0 {
            dir = dir.normalize();
        }
        unsafe { transform.translation += dir * playerspeed * time.delta_seconds(); }
    }
}

pub fn confinePlayerMovement(
    mut playerTransform: Query<&mut Transform, With<PLAYER>>,
    windowQuery: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut transform) = playerTransform.get_single_mut() {
        let window = windowQuery.get_single().unwrap();
        let halfWindowSizeX = window.width() / 2.0;
        let halfPlayerSizeX = playerSizeX / 2.0;
        let minX = 0.0 - halfWindowSizeX + halfPlayerSizeX;
        let maxX = 0.0 + halfWindowSizeX - halfPlayerSizeX;
        let mut translation = transform.translation;
        if translation.x < minX {
            translation.x = minX;
        }
        if translation.x > maxX {
            translation.x = maxX
        }
        transform.translation = translation;
    }
}

pub fn moveBall(
    mut balltransform: Query<&mut Transform, With<BALL>>,
    time: Res<Time>,
    ballQuery: Query<&BALL>,
) {
    if let Ok(mut transform) = balltransform.get_single_mut() {
        let ball = ballQuery.single();
        let mut dir = ball.dir;
        if dir.length() > 0.0 {
            dir = dir.normalize();
        }
        unsafe { transform.translation += dir * ballSpeed * time.delta_seconds(); }
    }
}

pub fn confineBallMovement(
    mut ballTransform: Query<&mut Transform, With<BALL>>,
    windowQuery: Query<&Window, With<PrimaryWindow>>,
    mut ballQuery: Query<&mut BALL>,
) {
    let mut ball = ballQuery.single_mut();

    if let Ok(mut transform) = ballTransform.get_single_mut() {
        let window = windowQuery.get_single().unwrap();
        let halfWindowSizeY = window.height() / 2.0;
        let halfWindowSizeX = window.width() / 2.0;
        let halfBallSize = ballSize / 2.0;
        let minX = 0.0 - halfWindowSizeX + halfBallSize;
        let maxX = 0.0 + halfWindowSizeX - halfBallSize;
        let minY = 0.0 - halfWindowSizeY + halfBallSize;
        let maxY = 0.0 + halfWindowSizeY - halfBallSize;
        let mut translation = transform.translation;

        if translation.x < minX {
            translation.x = minX;
            ball.dir = Vec3::new(-ball.dir.x, ball.dir.y, ball.dir.z);
        }
        if translation.x > maxX {
            translation.x = maxX;
            ball.dir = Vec3::new(-ball.dir.x, ball.dir.y, ball.dir.z);
        }
        if translation.y > maxY {
            translation.y = maxY;
            ball.dir = Vec3::new(ball.dir.x, -ball.dir.y, ball.dir.z);
        }
        if translation.y < minY {
            translation.y = minY;
            ball.dir = Vec3::new(ball.dir.x, -ball.dir.y, ball.dir.z);
        }
        transform.translation = translation;
    }
}

fn ballColitionMetPlayer(
    mut commands: Commands,
    assetServer: Res<AssetServer>,
    mut ballTransformQ: Query<&mut Transform, (With<BALL>, Without<PLAYER>)>,
    mut ballQuery: Query<&mut BALL>,
    playerTransformQ: Query<&mut Transform, (With<PLAYER>, Without<BALL>)>,
) {
    if let Ok(mut balltransform) = ballTransformQ.get_single_mut() {
        if let Ok(mut playertransform) = playerTransformQ.get_single() {
            let mut ball = ballQuery.single_mut();
            let halfPlayerSizeX = playerSizeX / 2.0;
            let halfPlayerSizeY = playerSizeY / 2.0;
            let halfBallSize = ballSize / 2.0;
            let playerX = playertransform.translation.x;
            let playerY = playertransform.translation.y;
            let ballX = balltransform.translation.x;
            let ballY = balltransform.translation.y;
            let Ycolition = (halfPlayerSizeY + halfBallSize) > ((ballY - playerY).abs());
            let Xcolition = (halfBallSize + halfPlayerSizeX) > ((ballX - playerX).abs());
            let mut dir = ball.dir;
            if Xcolition && Ycolition {
                ball.dir = Vec3::new(ball.dir.x, -ball.dir.y, ball.dir.z);
                if dir.length() > 0.0 {
                    dir = dir.normalize();
                }


                commands.spawn((
                    AudioBundle {
                        source: assetServer.load("sounds/impactGlass_light_003.ogg"),
                        ..default()
                    },
                    MyMusic,
                ));
            }
        }
    }
}

fn winOrLose(
    windowQuery: Query<&Window, With<PrimaryWindow>>,
    balltransform: Query<&Transform, With<BALL>>,
) {
    if kanVerliezen {
        if let Ok(mut transform) = balltransform.get_single() {
            let transformY = transform.translation.y;
            let window = windowQuery.single();
            let windowHeight = window.height() / 2.0;
            let loseY = 0.0 - windowHeight + ballSize;
            if transformY < loseY {
                panic!("You lose");
            }
        }
    }
}