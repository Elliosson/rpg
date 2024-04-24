use crate::components::*;
use bevy::prelude::*;

pub fn update_player_stats(
    inventory: Res<Inventory>,
    mut player_stats: Query<&mut CreatureStats, With<Player>>,
    gears: Query<(Entity, &Gear), Without<Player>>,
) {
    let mut armor = 0.;
    let mut player_stat = player_stats.single_mut();

    if let Some(slot) = inventory.slots.get(&101) {
        match slot {
            InventoryCase::Unique(_, entity) => {
                let (_, gear) = gears.get(*entity).unwrap();
                armor += gear.armor;
            }
            _ => {}
        }
    }

    if let Some(slot) = inventory.slots.get(&102) {
        match slot {
            InventoryCase::Unique(_, entity) => {
                let (_, gear) = gears.get(*entity).unwrap();
                armor += gear.armor;
            }
            _ => {}
        }
    }

    if let Some(slot) = inventory.slots.get(&103) {
        match slot {
            InventoryCase::Unique(_, entity) => {
                let (_, gear) = gears.get(*entity).unwrap();
                armor += gear.armor;
            }
            _ => {}
        }
    }

    player_stat.armor = armor;
}
