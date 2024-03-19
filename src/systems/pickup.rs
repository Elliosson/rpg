use crate::{components::*, utils::distance};
use bevy::prelude::*;

const MAX_PICKUP_DIST: f32 = 50.;
const INVENTORY_SIZE: i32 = 20;

pub fn add_to_invotory(inventory: &mut Inventory, name: &String) -> bool {
    //for now just stack
    for (_, case) in inventory.slots.iter_mut() {
        match case {
            InventoryCase::Stack(item_name, amount) => {
                if name == item_name {
                    *amount += 1;
                    return true;
                }
            }
            _ => {}
        }
    }

    //create new Stack
    for i in 1..(INVENTORY_SIZE + 1) {
        if inventory.slots.get(&i).is_none() {
            inventory
                .slots
                .insert(i, InventoryCase::Stack(name.clone(), 1));
            return true;
        }
    }

    //failed to insert the item in the inventory
    return false;
}

pub fn pickup(
    mut commands: Commands,
    mut pickables: Query<(Entity, &mut Transform, &Pickable, &PropName), Without<Player>>,
    player: Query<(Entity, &Transform), (With<WantToPickup>, With<Player>)>,
    mut inventory: ResMut<Inventory>,
) {
    if player.is_empty() {
        return;
    }
    let (player_entity, player_trans) = player.single();

    let mut closest: Option<(f32, Entity, String)> = None;

    for (entity, transform, _, name) in pickables.iter_mut() {
        let dist = distance(&player_trans, &transform);

        if dist < MAX_PICKUP_DIST {
            if let Some((min_dist, _, _)) = closest {
                if min_dist > dist {
                    closest = Some((dist, entity, name.name.clone()));
                }
            } else {
                closest = Some((dist, entity, name.name.clone()));
            }
        }
    }

    if let Some((_, entity, name)) = closest {
        //add in invotory
        if add_to_invotory(&mut inventory, &name) {
            //despawn if success
            commands.entity(entity).despawn();
        }
    }
    commands.entity(player_entity).remove::<WantToPickup>();
}
