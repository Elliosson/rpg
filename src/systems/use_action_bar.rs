use crate::{
    components::*,
    raws::{spawn_named_item, RawMaster, RAWS},
};
use bevy::prelude::*;

pub fn use_action_bar(
    mut commands: Commands,
    mut equiped_weapons: Query<
        (Entity, &mut Handle<Image>),
        (With<EquipedWeapon>, Without<SlotButton>),
    >,
    mut inventory: ResMut<Inventory>,
    mut action_bar_used: ResMut<ActionBarUsed>,
    player: Query<Entity, With<Player>>,
) {
    if let Some(id) = action_bar_used.id {
        if let Some(mut inv_case) = inventory.slots.get_mut(&id) {
            match inv_case {
                InventoryCase::Unique(_, to_equip_entity) => {
                    let (equipped_entity, _) = equiped_weapons.single_mut();
                    commands.entity(equipped_entity).remove::<EquipedWeapon>();
                    commands.entity(*to_equip_entity).insert(EquipedWeapon {});
                }
                InventoryCase::Stack(name, amount) => {
                    //reduce the amount by 1.
                    //clean if 0, here? an other system?
                    //ok here.
                    if *amount > 0 {
                        *amount -= 1;

                        //spawn an entity of the name.
                        //attack origin to the player.
                        //then a system will handle it.
                        let raws: &RawMaster = &RAWS.lock().unwrap();

                        let item_entity = spawn_named_item(&mut commands, name.clone(), raws);
                        let player_entity = player.single();
                        commands.entity(item_entity).insert(ActivatedBy {
                            entity: player_entity,
                        });
                    }
                }
                _ => {}
            }
        }
    }

    action_bar_used.id = None;
}
