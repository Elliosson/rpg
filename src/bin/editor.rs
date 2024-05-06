use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::sprite::Mesh2dHandle;
use rpg::init::*;
use rpg::map_json::*;
use rpg::raws::*;
use rpg::systems::*;
use rpg::ui::*;

pub fn init_editor_player(
    mut commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    mut materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let raws: &RawMaster = &RAWS.lock().unwrap();
    let player_entity = commands
        .spawn((
            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 4.0),
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
            CreatureStats { armor: 0. },
            Lifepoint {
                life: 100.,
                max: 100.,
            },
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

    let armor = spawn_named_entity(
        &mut commands,
        &asset_server,
        &mut meshes,
        &mut materials,
        (0., 0.),
        "armor".to_string(),
        raws,
    );
    commands.entity(armor).remove::<Transform>();
    commands.entity(armor).insert(Visibility::Hidden);

    let health_potion = spawn_named_entity(
        &mut commands,
        &asset_server,
        &mut meshes,
        &mut materials,
        (300., 300.),
        "health_potion".to_string(),
        raws,
    );

    let helmet = spawn_named_entity(
        &mut commands,
        &asset_server,
        &mut meshes,
        &mut materials,
        (0., 0.),
        "helmet".to_string(),
        raws,
    );
    commands.entity(helmet).remove::<Transform>();
    commands.entity(helmet).insert(Visibility::Hidden);

    let boots = spawn_named_entity(
        &mut commands,
        &asset_server,
        &mut meshes,
        &mut materials,
        (0., 0.),
        "boots".to_string(),
        raws,
    );
    commands.entity(boots).remove::<Transform>();
    commands.entity(boots).insert(Visibility::Hidden);

    commands.entity(health_potion).insert(Pickable {});

    commands.insert_resource(Inventory {
        slots: [
            (
                1,
                InventoryCase::Unique("hammer".to_string(), hammer.clone()),
            ),
            (2, InventoryCase::Unique("sword".to_string(), sword.clone())),
            (3, InventoryCase::Unique("lance".to_string(), lance.clone())),
            (4, InventoryCase::Stack("health_potion".to_string(), 10)),
            (5, InventoryCase::Stack("tree".to_string(), 10)),
            (6, InventoryCase::Stack("rock".to_string(), 10)),
            (
                10,
                InventoryCase::Unique("helmet".to_string(), helmet.clone()),
            ),
            (
                11,
                InventoryCase::Unique("armor".to_string(), armor.clone()),
            ),
            (
                12,
                InventoryCase::Unique("boots".to_string(), boots.clone()),
            ),
        ]
        .iter()
        .cloned()
        .collect(),
    });
}

// Add the game's entities to our world
pub fn setup_editor(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    load_raws();

    // Camera
    commands.spawn((Camera2dBundle::default(), MainCamera {}));

    cache_image_handles(&mut commands, &asset_server);
    init_editor_player(&mut commands, &asset_server, &mut meshes, &mut materials);

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

pub fn init_editor() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup_editor)
        // Add our gameplay simulation systems to the fixed timestep schedule
        // which runs at 64 Hz by default
        .add_systems(
            FixedUpdate,
            (
                move_player,
                use_action_bar,
                equipped_item,
                monster_find_target,
                camera_on_player,
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

fn main() {
    init_editor();
}
