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
