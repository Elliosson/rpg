//! A simplified implementation of the classic game "Breakout".

use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;
use bevy::time::Stopwatch;
use bevy::window::PrimaryWindow;
use std::time::Instant;
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

#[derive(Component)]
struct IsAttacking {
    start_time: Stopwatch,
}

#[derive(Component)]
struct DeltaAngle {
    delta: f32,
}

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
                attack_animation,
            )
                // `chain`ing systems together runs them in order
                .chain(),
        )
        .add_systems(Update, (bevy::window::close_on_esc, mouse_button_input))
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
        DeltaAngle { delta: 0. },
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
    mut query: Query<(&mut Transform, &DeltaAngle), With<Player>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = q_camera.single();
    let (mut player, delta_angle) = query.single_mut();
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
        player.rotation = Quat::from_rotation_z(angle + delta_angle.delta);
    }
}

fn weapon_movement(
    player: Query<&Transform, (With<Player>, Without<EquipedWeapon>)>,
    mut weapon: Query<&mut Transform, (With<EquipedWeapon>, Without<Player>)>,
) {
    let mut weapon_transform = weapon.single_mut();
    let player_transform = player.single();

    let player_radius: f32 = 60.;
    let (_, angle) = player_transform.rotation.to_axis_angle();
    let rotation = player_transform.rotation;

    let asin = rotation.z;

    let true_angle = if asin < 0. { angle } else { -angle };

    let left_hand_angle = true_angle - 1.;

    let dx = player_radius * left_hand_angle.sin();
    let dy = player_radius * left_hand_angle.cos();

    weapon_transform.translation.x = player_transform.translation.x + dx;
    weapon_transform.translation.y = player_transform.translation.y + dy;
    weapon_transform.rotation = player_transform.rotation;
}

fn mouse_button_input(
    mut commands: Commands,
    buttons: Res<ButtonInput<MouseButton>>,
    mut player: Query<Entity, (With<Player>, Without<IsAttacking>)>,
    time: Res<Time>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        // Left button was pressed
        if let Ok(player_entity) = player.get_single_mut() {
            commands.entity(player_entity).insert(IsAttacking {
                start_time: Stopwatch::new(),
            });
        }
    }
    if buttons.just_released(MouseButton::Left) {
        // Left Button was released
    }
    if buttons.pressed(MouseButton::Right) {
        // Right Button is being held down
    }
    // we can check multiple at once with `.any_*`
    if buttons.any_just_pressed([MouseButton::Left, MouseButton::Right]) {
        // Either the left or the right button was just pressed
    }
}

fn attack_animation(
    mut commands: Commands,
    mut player: Query<
        (Entity, &mut Transform, &mut IsAttacking, &mut DeltaAngle),
        (With<Player>, With<IsAttacking>),
    >,
    mut weapon: Query<&mut Transform, (With<EquipedWeapon>, Without<Player>)>,
    time: Res<Time>,
) {
    if let Ok((entity, player_transform, mut is_attacking, mut delta_angle)) =
        player.get_single_mut()
    {
        is_attacking.start_time.tick(time.delta()).elapsed_secs();
        let delta_time = is_attacking.start_time.elapsed_secs();
        println!("delta time {}", delta_time);
        if delta_time < 0.1 {
            delta_angle.delta = delta_time * 10.;
        } else if delta_time < 0.2 {
            delta_angle.delta = (delta_time - 0.2) * 10.;
        } else {
            delta_angle.delta = 0.;
            commands.entity(entity).remove::<IsAttacking>();
        }
    }
}
