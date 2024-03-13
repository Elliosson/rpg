use std::collections::HashMap;

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
                drag_and_drop,
                update_level_text,
                equipped_item,
                update_inventoty_ui,
                update_action_bar_ui,
            )
                // `chain`ing systems together runs them in order
                .chain(),
        )
        .add_systems(
            Update,
            (
                bevy::window::close_on_esc,
                mouse_button_input,
                item_slot_button,
                inventory_button,
            ),
        )
        .init_state::<InventoryUiState>()
        .add_systems(OnEnter(InventoryUiState::Open), inventory_ui)
        .add_systems(
            OnExit(InventoryUiState::Open),
            despawn_screen::<InventoryScreen>,
        )
        .insert_resource(ButtonHovered { entity: None })
        .insert_resource(ButtonPressed { entity: None })
        .insert_resource(ButtonJustReleased { entity: None })
        .run();
}

// Add the game's entities to our world
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    load_raws();
    let raws: &RawMaster = &RAWS.lock().unwrap();

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

    commands.insert_resource(StoreImageHandle {
        images: [
            ("lance".to_string(), asset_server.load("lance_icon.png")),
            (
                "sword_icon".to_string(),
                asset_server.load("sword_icon.png"),
            ),
            (
                "hammer_icon".to_string(),
                asset_server.load("hammer_icon.png"),
            ),
        ]
        .iter()
        .cloned()
        .collect(),
    });

    spawn_named_entity(
        &mut commands,
        &asset_server,
        &mut meshes,
        &mut materials,
        (200., 200.),
        "tree".to_string(),
        raws,
    );

    let hammer = spawn_named_entity(
        &mut commands,
        &asset_server,
        &mut meshes,
        &mut materials,
        (0., 0.),
        "hammer".to_string(),
        raws,
    );

    let sword = spawn_named_entity(
        &mut commands,
        &asset_server,
        &mut meshes,
        &mut materials,
        (0., 0.),
        "sword".to_string(),
        raws,
    );

    let lance = spawn_named_entity(
        &mut commands,
        &asset_server,
        &mut meshes,
        &mut materials,
        (0., 0.),
        "lance".to_string(),
        raws,
    );

    commands.insert_resource(Inventory {
        slots: [
            (
                1,
                InventoryCase::Unique("hammer".to_string(), hammer.clone()),
            ),
            (2, InventoryCase::Unique("sword".to_string(), sword.clone())),
            (3, InventoryCase::Unique("lance".to_string(), lance.clone())),
        ]
        .iter()
        .cloned()
        .collect(),
    });

    spawn_named_entity(
        &mut commands,
        &asset_server,
        &mut meshes,
        &mut materials,
        (-200., 200.),
        "rock".to_string(),
        raws,
    );

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
        (-300., -300.),
        "slim".to_string(),
        raws,
    );
}
