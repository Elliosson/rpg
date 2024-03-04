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
