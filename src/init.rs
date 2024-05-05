use crate::components;
use crate::raws::{spawn_named_entity, RawMaster, RAWS};
use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
pub use components::*;

pub fn init_player(
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

pub fn cache_image_handles(commands: &mut Commands, asset_server: &Res<AssetServer>) {
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
            (
                "health_potion".to_string(),
                asset_server.load("health_potion_icon.png"),
            ),
            (
                "armor_transparent".to_string(),
                asset_server.load("armor_transparent.png"),
            ),
            (
                "boots_transparent".to_string(),
                asset_server.load("boots_transparent.png"),
            ),
            (
                "helmet_transparent".to_string(),
                asset_server.load("helmet_transparent.png"),
            ),
            (
                "armor_icon".to_string(),
                asset_server.load("armor_icon.png"),
            ),
            (
                "boots_icon".to_string(),
                asset_server.load("boots_icon.png"),
            ),
            (
                "helmet_icon".to_string(),
                asset_server.load("helmet_icon.png"),
            ),
        ]
        .iter()
        .cloned()
        .collect(),
    });
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
