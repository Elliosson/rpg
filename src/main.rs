//! A simplified implementation of the classic game "Breakout".

use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use sepax2d::prelude::*;

const PLAYER_SPEED: f32 = 500.0;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Tree;

#[derive(Component)]
enum Sepax2dShape {
    Circle(f32),
}

#[derive(Component)]
struct Collision;

#[derive(Component)]
struct Slim;

#[derive(Component)]
struct Rock;

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct EquipedWeapon;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        // Add our gameplay simulation systems to the fixed timestep schedule
        // which runs at 64 Hz by default
        .add_systems(
            FixedUpdate,
            (
                move_player,
                camera_on_player,
                collison,
                rotate_player,
                weapon_movement,
            )
                // `chain`ing systems together runs them in order
                .chain(),
        )
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

// Add the game's entities to our world
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Camera
    commands.spawn((Camera2dBundle::default(), MainCamera));

    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                ..default()
            },
            texture: asset_server.load("character.png"),
            ..default()
        },
        Player,
        Collision,
        Sepax2dShape::Circle(26.),
    ));

    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(200.0, 200.0, 0.0),
                ..default()
            },
            texture: asset_server.load("tree.png"),
            ..default()
        },
        Tree,
        Collision,
        Sepax2dShape::Circle(56.),
    ));

    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(-200.0, -200.0, 0.0),
                ..default()
            },
            texture: asset_server.load("slim.png"),
            ..default()
        },
        Slim,
        Collision,
        Sepax2dShape::Circle(52.),
    ));

    commands.spawn((
        SpriteBundle {
            transform: Transform { ..default() },
            texture: asset_server.load("hammer.png"),
            ..default()
        },
        EquipedWeapon,
    ));

    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(-200.0, 200.0, 0.0),
                ..default()
            },
            texture: asset_server.load("rock.png"),
            ..default()
        },
        Rock,
        Collision,
        Sepax2dShape::Circle(162.),
    ));

    let character_circle: sepax2d::prelude::Circle = sepax2d::prelude::Circle::new((0., 0.), 1.);
    let tree_circle: sepax2d::prelude::Circle = sepax2d::prelude::Circle::new((0., 0.), 2.);

    sepax2d::prelude::sat_collision(&character_circle, &tree_circle);
}

fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let mut player_transform = query.single_mut();
    let mut direction_x = 0.0;
    let mut direction_y = 0.0;

    if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
        direction_x -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
        direction_x += 1.0;
    }

    if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
        direction_y -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
        direction_y += 1.0;
    }

    player_transform.translation.x =
        player_transform.translation.x + direction_x * PLAYER_SPEED * time.delta_seconds();
    player_transform.translation.y =
        player_transform.translation.y + direction_y * PLAYER_SPEED * time.delta_seconds();
}

fn collison(
    mut player: Query<(&mut Transform, &Sepax2dShape), (With<Player>, With<Sepax2dShape>)>,
    collidables: Query<
        (&Transform, &Sepax2dShape),
        (With<Collision>, With<Sepax2dShape>, Without<Player>),
    >,
) {
    let (mut player_transform, shape) = player.single_mut();
    let radius = if let Sepax2dShape::Circle(radius) = shape {
        *radius
    } else {
        0.
    };

    let character_circle = sepax2d::prelude::Circle::new(
        (
            player_transform.translation.x,
            player_transform.translation.y,
        ),
        radius,
    );

    for (transform, shape) in collidables.iter() {
        let (x_delta, y_delta): (f32, f32) = match shape {
            Sepax2dShape::Circle(radius) => {
                let object = sepax2d::prelude::Circle::new(
                    (transform.translation.x, transform.translation.y),
                    *radius,
                );
                sepax2d::prelude::sat_collision(&object, &character_circle)
            }
        };
        //super messy resolve. at the end. juste forbit to move if colliding
        player_transform.translation.x += x_delta;
        player_transform.translation.y += y_delta;
    }
}

fn camera_on_player(
    query_player: Query<&Transform, (With<Player>, Without<Camera2d>)>,
    mut query_camera: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
) {
    let player_transform = query_player.single();
    for mut camera in query_camera.iter_mut() {
        camera.translation = player_transform.translation;
    }
}

fn rotate_player(
    mut query: Query<&mut Transform, With<Player>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = q_camera.single();
    let mut player = query.single_mut();
    let window = q_windows.single();
    let player_v2 = player.translation.truncate();

    if let Some(cursor_world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        eprintln!(
            "World coords: {}/{}",
            cursor_world_position.x, cursor_world_position.y
        );
        let diff = cursor_world_position - player_v2;
        let angle = diff.y.atan2(diff.x) - FRAC_PI_2;
        player.rotation = Quat::from_rotation_z(angle);
    }
}

fn weapon_movement(
    player: Query<&Transform, (With<Player>, Without<EquipedWeapon>)>,
    mut weapon: Query<&mut Transform, (With<EquipedWeapon>, Without<Player>)>,
) {
    let mut weapon_transform = weapon.single_mut();
    let player_transform = player.single();

    let player_radius: f32 = 30.;
    let (_, angle) = player_transform.rotation.to_axis_angle();
    let rotation = player_transform.rotation;
    let magic = rotation.w * rotation.z;

    let dx = -player_radius * magic.sin() * 2.;
    let dy = player_radius * angle.cos();

    weapon_transform.translation.x = player_transform.translation.x + dx;
    weapon_transform.translation.y = player_transform.translation.y + dy;
    weapon_transform.rotation = player_transform.rotation;
}
