use bevy::prelude::*;

mod components;
pub use components::*;
mod systems;
pub use systems::*;

const PLAYER_SPEED: f32 = 200.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
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
                weapon_hit,
            )
                // `chain`ing systems together runs them in order
                .chain(),
        )
        .add_systems(Update, (bevy::window::close_on_esc, mouse_button_input))
        .run();
}

// Add the game's entities to our world
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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
        Sepax2dShape::Rectangle(80., 4.),
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

    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0., 0., -1.0),
                scale: Vec3::splat(4.0),
                ..default()
            },
            texture: asset_server.load("map.png"),

            ..default()
        },
        MapBackground,
    ));

    let character_circle: sepax2d::prelude::Circle = sepax2d::prelude::Circle::new((0., 0.), 1.);
    let tree_circle: sepax2d::prelude::Circle = sepax2d::prelude::Circle::new((0., 0.), 2.);

    sepax2d::prelude::sat_collision(&character_circle, &tree_circle);
}
