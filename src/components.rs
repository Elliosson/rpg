use bevy::prelude::*;
use bevy::time::Stopwatch;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Tree;

#[derive(Component)]
pub enum Sepax2dShape {
    Circle(f32),
    Rectangle(f32, f32),
}

#[derive(Component)]
pub struct Collision;

#[derive(Component)]
pub struct Slim;

#[derive(Component)]
pub struct Rock;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct EquipedWeapon;

#[derive(Component)]
pub struct IsAttacking {
    pub start_time: Stopwatch,
}

#[derive(Component)]
pub struct DeltaAngle {
    pub delta: f32,
}

#[derive(Component)]
pub struct MapBackground;

#[derive(Component)]
pub struct IsHitAnimation {
    pub dx: f32,
    pub dy: f32,
    pub start_time: Stopwatch,
    pub already_moved: bool,
}

#[derive(Component)]
pub struct Lifepoint {
    pub life: f32,
}

#[derive(Component)]
pub struct LifeBar {
    pub linked_entity: Entity,
}

#[derive(Component)]
pub struct IsHit;

#[derive(Component)]

pub struct Imobile;

#[derive(Component)]

pub struct Mobile;

#[derive(Component)]

pub struct Weight {
    pub weight: i32,
}

#[derive(Component)]

pub struct Target {
    pub entity: Entity,
    pub direction: f32,
}

#[derive(Component)]

pub struct ContactAttack;

#[derive(Component)]

pub struct Monster;
