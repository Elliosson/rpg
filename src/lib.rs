pub mod components;
pub mod init;
pub mod map_json;
pub mod raws;
pub mod systems;
pub mod ui;
pub mod utils;
use bevy::prelude::*;
use init::*;
use map_json::*;
use raws::*;
use systems::*;
use ui::*;

#[macro_use]
extern crate lazy_static;

pub const PLAYER_SPEED: f32 = 200.0;

// Add the game's entities to our world
pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    load_raws();

    // Camera
    commands.spawn((Camera2dBundle::default(), MainCamera {}));

    cache_image_handles(&mut commands, &asset_server);
    init_player(&mut commands, &asset_server, &mut meshes, &mut materials);

    let raws: &RawMaster = &RAWS.lock().unwrap();
    load_map(
        &mut commands,
        &asset_server,
        &mut meshes,
        &mut materials,
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
}

pub fn init_world() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        // Add our gameplay simulation systems to the fixed timestep schedule
        // which runs at 64 Hz by default
        .add_systems(
            FixedUpdate,
            (
                move_player,
                use_action_bar,
                equipped_item,
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
                update_inventoty_ui,
                update_action_bar_ui,
            )
                // `chain`ing systems together runs them in order
                .chain(),
        )
        .add_systems(FixedUpdate, (health_potion, level_up).chain())
        .add_systems(
            Update,
            (
                bevy::window::close_on_esc,
                mouse_button_input,
                item_slot_button,
                inventory_button,
                pickup,
                spawner,
                update_player_stats,
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
        .insert_resource(ActionBarUsed { id: None })
        .insert_resource(ToSpawn { items: Vec::new() })
        .run();
}
