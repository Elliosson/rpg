use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

mod components;
pub use components::*;
mod systems;
use raws::{load_raws, spawn_named_entity, RawMaster, RAWS};
pub use systems::*;
use ui::spawn_ui;
mod raws;
mod ui;
mod utils;

#[macro_use]
extern crate lazy_static;

const PLAYER_SPEED: f32 = 200.0;

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

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
                monster_find_target,
                move_slim,
                camera_on_player,
                contact_attack,
                collison,
                rotate_player,
                weapon_movement,
                attack_animation,
                weapon_hit,
                is_hit_animation,
                remove_life,
                draw_life,
                death,
                update_level_text,
            )
                // `chain`ing systems together runs them in order
                .chain(),
        )
        .add_systems(Update, (bevy::window::close_on_esc, mouse_button_input))
        .insert_resource(InventoryUi { open: false })
        .init_state::<InventoryUiState>()
        .add_systems(OnEnter(InventoryUiState::Open), inventory_ui)
        .add_systems(
            OnExit(InventoryUiState::Open),
            despawn_screen::<InventoryScreen>,
        )
        .run();
}

// Add the game's entities to our world
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Camera
    commands.spawn((Camera2dBundle::default(), MainCamera {}));

    let player_entity = commands
        .spawn((
            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 0.0),
                    ..default()
                },
                texture: asset_server.load("character.png"),
                ..default()
            },
            Player {},
            Collision {},
            Sepax2dShape::Circle(26.),
            DeltaAngle { delta: 0. },
            Mobile {},
            Weight { weight: 1000 },
            Lifepoint { life: 100. },
            Level { level: 1, xp: 0. },
        ))
        .id();

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(10.0, 1.0))),
            material: materials.add(ColorMaterial::default()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        LifeBar {
            linked_entity: player_entity,
        },
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
        Tree {},
        Collision {},
        Sepax2dShape::Circle(56.),
        Imobile {},
    ));

    commands.spawn((
        SpriteBundle {
            transform: Transform { ..default() },
            texture: asset_server.load("hammer.png"),
            ..default()
        },
        EquipedWeapon {},
        Sepax2dShape::Rectangle(70., 4.),
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
        Rock {},
        Collision {},
        Sepax2dShape::Circle(162.),
        Imobile {},
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
        MapBackground {},
    ));

    spawn_ui(&mut commands, &asset_server);

    load_raws();
    let raws: &RawMaster = &RAWS.lock().unwrap();
    spawn_named_entity(
        &mut commands,
        &asset_server,
        &mut meshes,
        &mut materials,
        (500., 500.),
        "slim".to_string(),
        raws,
    );

    spawn_named_entity(
        &mut commands,
        &asset_server,
        &mut meshes,
        &mut materials,
        (600., 500.),
        "slim".to_string(),
        raws,
    );

    spawn_named_entity(
        &mut commands,
        &asset_server,
        &mut meshes,
        &mut materials,
        (500., 600.),
        "slim".to_string(),
        raws,
    );

    spawn_named_entity(
        &mut commands,
        &asset_server,
        &mut meshes,
        &mut materials,
        (-200., -200.),
        "slim".to_string(),
        raws,
    );
}
