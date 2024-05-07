use std::collections::HashMap;

use bevy::prelude::*;
use bevy::time::Stopwatch;
use serde::Deserialize;

//todo use sparse set for the component often add/remove

#[derive(Component, Deserialize, Debug, Clone)]
pub struct Player {}

#[derive(Component, Deserialize, Debug, Clone)]
pub struct Tree {}

#[derive(Component, Deserialize, Debug, Clone)]
pub enum Sepax2dShape {
    Circle(f32),
    Rectangle(f32, f32),
}

#[derive(Component, Deserialize, Debug, Clone)]
pub struct Collision {}

#[derive(Component, Deserialize, Debug, Clone)]
pub struct Slim {}

#[derive(Component, Deserialize, Debug, Clone)]
pub struct Rock {}

#[derive(Component, Deserialize, Debug, Clone)]
pub struct MainCamera {}

#[derive(Component, Deserialize, Debug, Clone)]
pub struct EquipedWeapon {}

#[derive(Component, Debug)]
pub struct IsAttacking {
    pub start_time: Stopwatch,
}

#[derive(Component, Deserialize, Debug, Clone)]
pub struct DeltaAngle {
    pub delta: f32,
}

#[derive(Component, Deserialize, Debug, Clone)]
pub struct MapBackground {}

#[derive(Component, Debug)]
pub struct IsHitAnimation {
    pub dx: f32,
    pub dy: f32,
    pub start_time: Stopwatch,
    pub already_moved: bool,
}

#[derive(Component, Deserialize, Debug, Clone)]
pub struct Lifepoint {
    pub life: f32,
    pub max: f32,
}

#[derive(Component, Deserialize, Debug, Clone)]
pub struct LifeBar {
    pub linked_entity: Entity,
}

#[derive(Component, Deserialize, Debug, Clone)]
pub struct IsHit {}

#[derive(Component, Deserialize, Debug, Clone)]

pub struct Imobile {}

#[derive(Component, Deserialize, Debug, Clone)]

pub struct Mobile {}

#[derive(Component, Deserialize, Debug, Clone)]

pub struct Weight {
    pub weight: i32,
}

#[derive(Component, Deserialize, Debug, Clone)]

pub struct Target {
    pub entity: Entity,
    pub direction: f32,
}

#[derive(Component, Deserialize, Debug, Clone)]

pub struct ContactAttack {}

#[derive(Component, Deserialize, Debug, Clone)]

pub struct Monster {}

#[derive(Component, Deserialize, Debug, Clone)]

pub struct Buildable {}

#[derive(Component, Deserialize, Debug, Clone)]
pub struct PropName {
    pub name: String,
}

#[derive(Component, Deserialize, Debug, Clone)]
pub struct Level {
    pub xp: f32,
    pub level: i32,
}

#[derive(Component, Deserialize, Debug, Clone)]
pub struct LastHitBy {
    pub entity: Entity,
}

#[derive(Component, Deserialize, Debug, Clone)]
pub struct LevelText {}

#[derive(Component, Deserialize, Debug, Clone)]
pub struct UniqueItem {}

#[derive(Component, Deserialize, Debug, Clone)]
pub struct SlotButton {
    pub id: i32,
}

#[derive(Component, Debug, Clone)]
pub struct InventorySlot {
    pub id: i32,
    pub previous_interaction: Interaction,
}

#[derive(Component, Deserialize, Debug, Clone)]
pub struct InventoryScreen;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum InventoryUiState {
    #[default]
    Closed,
    Open,
}

#[derive(Resource, Deserialize, Debug, Clone)]

pub struct ButtonPressed {
    pub entity: Option<Entity>,
}

#[derive(Resource, Deserialize, Debug, Clone)]

pub struct ButtonHovered {
    pub entity: Option<Entity>,
}

#[derive(Resource, Deserialize, Debug, Clone)]
pub struct ButtonJustReleased {
    pub entity: Option<Entity>,
}

#[derive(Clone)]
pub enum InventoryCase {
    Stack(String, i32),
    Unique(String, Entity),
}

#[derive(Resource, Clone)]
pub struct Inventory {
    pub slots: HashMap<i32, InventoryCase>,
}

//ensure the images stay loaded.
#[derive(Resource, Clone)]
pub struct StoreImageHandle {
    pub images: HashMap<String, Handle<Image>>,
}

//ensure the images stay loaded.
#[derive(Resource, Clone)]
pub struct ActionBarUsed {
    pub id: Option<i32>,
}

#[derive(Component, Deserialize, Debug, Clone)]
pub struct HealthPotion {
    pub heal: f32,
}

#[derive(Component, Deserialize, Debug, Clone)]
pub struct ActivatedBy {
    pub entity: Entity,
}

#[derive(Component, Deserialize, Debug, Clone)]
pub struct Pickable {}

#[derive(Component, Deserialize, Debug, Clone)]
pub struct GearSlot {
    pub name: String,
}

#[derive(Component, Deserialize, Debug, Clone)]
pub struct WantToPickup {}

#[derive(Component, Deserialize, Debug, Clone)]
pub struct Drop {
    pub name: String,
}

#[derive(Component, Deserialize, Debug, Clone)]
pub struct Gear {
    pub armor: f32,
}

#[derive(Component, Deserialize, Debug, Clone)]
pub struct CreatureStats {
    pub armor: f32,
}

#[derive(Resource, Deserialize, Debug, Clone)]
pub struct ToSpawn {
    pub items: Vec<(String, f32, f32)>,
}

#[derive(Component, Deserialize, Debug, Clone)]
pub enum ZLayer {
    Background,
    Object,
    Item,
    Creature,
    Player,
    Weapon,
}

impl ZLayer {
    pub fn value(&self) -> f32 {
        match *self {
            ZLayer::Background => 0.,
            ZLayer::Object => 1.,
            ZLayer::Item => 2.,
            ZLayer::Creature => 3.,
            ZLayer::Player => 4.,
            ZLayer::Weapon => 5.,
        }
    }
}
