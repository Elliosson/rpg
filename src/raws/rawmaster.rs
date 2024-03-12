use std::collections::{HashMap, HashSet};

use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use serde::Deserialize;

use super::Raws;
use crate::components::*;

#[derive(Deserialize, Debug)]
pub struct Template {
    pub name: String,
    pub image: String,
    pub player: Option<Player>,
    pub mobile: Option<Mobile>,
    pub slim: Option<Slim>,
    pub collision: Option<Collision>,
    pub shape: Option<Sepax2dShape>,
    pub life: Option<Lifepoint>,
    pub weight: Option<Weight>,
    pub monster: Option<Monster>,
    pub contact_attack: Option<ContactAttack>,
    pub imobile: Option<Imobile>,
    pub rock: Option<Rock>,
    pub equiped_weapon: Option<EquipedWeapon>,
    pub tree: Option<Tree>,
    pub unique_item: Option<UniqueItem>,
}

pub struct RawMaster {
    pub raws: Raws,
    pub prop_index: HashMap<String, usize>,
}

impl RawMaster {
    pub fn empty() -> RawMaster {
        RawMaster {
            raws: Raws { props: Vec::new() },

            prop_index: HashMap::new(),
        }
    }

    pub fn load(&mut self, raws: Raws) {
        self.raws = raws;
        self.prop_index = HashMap::new();
        let mut used_names: HashSet<String> = HashSet::new();

        for (i, prop) in self.raws.props.iter().enumerate() {
            if used_names.contains(&prop.name) {
                println!("WARNING -  duplicate prop name in raws [{}]", prop.name);
            }
            self.prop_index.insert(prop.name.clone(), i);

            used_names.insert(prop.name.clone());
        }
    }
}

pub fn spawn_props(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    pos: (f32, f32),
    template: &Template,
) -> Entity {
    let entity = commands
        .spawn((SpriteBundle {
            transform: Transform {
                translation: Vec3::new(pos.0, pos.1, 0.0),
                ..default()
            },
            texture: asset_server.load(&template.image),
            ..default()
        },))
        .id();

    commands.entity(entity).insert(PropName {
        name: template.name.clone(),
    });

    if let Some(comp) = &template.player {
        commands.entity(entity).insert(comp.clone());
    }
    if let Some(comp) = &template.mobile {
        commands.entity(entity).insert(comp.clone());
    }
    if let Some(comp) = &template.slim {
        commands.entity(entity).insert(comp.clone());
    }
    if let Some(comp) = &template.collision {
        commands.entity(entity).insert(comp.clone());
    }
    if let Some(comp) = &template.shape {
        commands.entity(entity).insert(comp.clone());
    }
    if let Some(comp) = &template.life {
        commands.entity(entity).insert(comp.clone());

        //life bar
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Rectangle::new(10.0, 1.0))),
                material: materials.add(ColorMaterial::default()),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            },
            LifeBar {
                linked_entity: entity,
            },
        ));
    }
    if let Some(comp) = &template.weight {
        commands.entity(entity).insert(comp.clone());
    }
    if let Some(comp) = &template.monster {
        commands.entity(entity).insert(comp.clone());
    }
    if let Some(comp) = &template.contact_attack {
        commands.entity(entity).insert(comp.clone());
    }
    if let Some(comp) = &template.imobile {
        commands.entity(entity).insert(comp.clone());
    }
    if let Some(comp) = &template.rock {
        commands.entity(entity).insert(comp.clone());
    }
    if let Some(comp) = &template.equiped_weapon {
        commands.entity(entity).insert(comp.clone());
    }
    if let Some(comp) = &template.tree {
        commands.entity(entity).insert(comp.clone());
    }
    if let Some(comp) = &template.unique_item {
        commands.entity(entity).insert(comp.clone());
    }

    return entity;
}

pub fn spawn_named_entity(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    pos: (f32, f32),
    name: String,
    raws: &RawMaster,
) -> Entity {
    return spawn_props(
        commands,
        asset_server,
        meshes,
        materials,
        pos,
        &raws.raws.props[raws.prop_index[&name]],
    );
}
