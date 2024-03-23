// Todo: Rewrite this to work with only equip_index.
pub mod equipment_view_model {
    use std::collections::HashMap;

    use crate::{save::common::save_slot::{GaItem, SaveSlot}, util::{params::params::Row, regulation::Regulation}, vm::inventory::InventoryGaitemType};

    #[derive(Default, Clone)]
    pub struct EquipmentItemViewModel {
        pub gaitem_handle: u32,
        pub id: u32,
        pub name: String,
        pub equip_index: u32,
    }

    #[derive(Default, Clone)]
    pub struct EquipmentViewModel  {
        pub left_hand_armaments: [EquipmentItemViewModel; 3],
        pub right_hand_armaments: [EquipmentItemViewModel; 3],
        pub arrows: [EquipmentItemViewModel; 2],
        pub bolts: [EquipmentItemViewModel; 2],
        pub head: EquipmentItemViewModel,
        pub chest: EquipmentItemViewModel,
        pub arms: EquipmentItemViewModel,
        pub legs: EquipmentItemViewModel,
        pub talismans: [EquipmentItemViewModel; 4],
        pub quickitems: [EquipmentItemViewModel; 10],
        pub pouch: [EquipmentItemViewModel; 6],

        pub talisman_count: u32,

        pub current_equipped_items: Vec<u32>,
        pub current_index: usize,

        pub changed: bool,
    }

    impl EquipmentViewModel {
        pub fn from_save(slot:& SaveSlot) -> Self {
            let mut equipment_vm = EquipmentViewModel::default();

            let gaitem_map: HashMap<u32, GaItem> = slot.ga_items.iter()
                .filter(|g| g.gaitem_handle != 0)
                .map(|g| (g.gaitem_handle, *g))
                .collect();

            equipment_vm.talisman_count = 1; 
            for i in 0..slot.equip_inventory_data.key_inventory_items_distinct_count as usize {
                let key_item = slot.equip_inventory_data.key_items[i];
                if (key_item.ga_item_handle ^ InventoryGaitemType::ITEM as u32) == 10040 {
                    equipment_vm.talisman_count = u32::min(1+key_item.quantity as u32, 4);
                } 
            }

            // Weapons Left
            equipment_vm.left_hand_armaments
                .iter_mut()
                .enumerate()
                .for_each(|
                    (index, weapon)| 
                    Self::weapon(slot, weapon, &gaitem_map, &slot.chr_asm2.left_hand_armaments[index])
                );
            
            // Weapons Right
            equipment_vm.right_hand_armaments
            .iter_mut()
            .enumerate()
            .for_each(|
                (index, weapon)| 
                Self::weapon(slot, weapon, &gaitem_map, &slot.chr_asm2.right_hand_armaments[index])
            );

            // Arrows
            equipment_vm.arrows
            .iter_mut()
            .enumerate()
            .for_each(|
                (index, projectile)| 
                Self::projectile(slot, projectile, &gaitem_map, &slot.chr_asm2.arrows[index])
            );
            
            // Bolts
            equipment_vm.bolts
            .iter_mut()
            .enumerate()
            .for_each(|
                (index, projectile)| 
                Self::projectile(slot, projectile, &gaitem_map, &slot.chr_asm2.bolts[index]));
            
            // Armor 
            equipment_vm.head = Self::armor(slot, &gaitem_map, &slot.chr_asm2.head);
            equipment_vm.chest = Self::armor(slot, &gaitem_map, &slot.chr_asm2.chest);
            equipment_vm.arms = Self::armor(slot, &gaitem_map, &slot.chr_asm2.arms);
            equipment_vm.legs = Self::armor(slot, &gaitem_map, &slot.chr_asm2.legs);

            // Talismans
            equipment_vm.talismans
            .iter_mut()
            .enumerate()
            .for_each(|
                (talisman_index, talisman)| 
                Self::talisman(slot, talisman, slot.chr_asm2.talismans[talisman_index])
            );

            // Quickslot
            equipment_vm.quickitems
                .iter_mut()
                .enumerate()
                .for_each(|
                    (qs_index, qs)| 
                    Self::item(slot, qs, slot.equip_item_data.quick_slot_items[qs_index].item_id)
                );
            
            // Pouch
            equipment_vm.pouch
                .iter_mut()
                .enumerate()
                .for_each(|
                    (ps_index, ps)| 
                    Self::item(slot, ps, slot.equip_item_data.pouch_items[ps_index].item_id)
                );

            equipment_vm
        }

        fn weapon(slot:& SaveSlot, weapon: &mut EquipmentItemViewModel, gaitem_map: &HashMap<u32, GaItem> ,gaitem_handle: &u32) {
            weapon.gaitem_handle = *gaitem_handle;
            weapon.id = gaitem_map[gaitem_handle].item_id;
            weapon.equip_index = Self::equip_index(slot, *gaitem_handle);
            weapon.name = Self::name_or_empty(Regulation::equip_weapon_params_map(), (weapon.id/100)*100);
        }

        fn projectile(slot:& SaveSlot, projectile: &mut EquipmentItemViewModel, gaitem_map: &HashMap<u32, GaItem>, gaitem_handle: &u32) {
            projectile.gaitem_handle = *gaitem_handle;
            projectile.id = if gaitem_map.get(gaitem_handle).is_some() {gaitem_map[gaitem_handle].item_id} else {0};
            projectile.equip_index = Self::equip_index(slot, *gaitem_handle);
            projectile.name = Self::name_or_empty(Regulation::equip_weapon_params_map(), projectile.id);
        }

        fn armor(slot:& SaveSlot, gaitem_map: &HashMap<u32, GaItem>, gaitem_handle: &u32) -> EquipmentItemViewModel {
            let mut armor = EquipmentItemViewModel::default();
            armor.gaitem_handle = *gaitem_handle;
            armor.id = if gaitem_map[gaitem_handle].item_id  != 0 {gaitem_map[gaitem_handle].item_id  ^ 0x10000000} else {0};
            armor.equip_index=  Self::equip_index(slot, *gaitem_handle);
            armor.name = Self::name_or_empty(Regulation::equip_protectors_param_map(), armor.id);
            armor
        }

        fn talisman(slot:& SaveSlot, talisman: &mut EquipmentItemViewModel, gaitem_handle: u32) {
            talisman.gaitem_handle = gaitem_handle;
            talisman.id = if gaitem_handle != 0 {gaitem_handle ^ 0xA0000000} else {0};
            talisman.equip_index = Self::equip_index(slot, gaitem_handle);
            talisman.name = Self::name_or_empty(Regulation::equip_accessory_param_map(), talisman.id);
        }

        fn item(slot:& SaveSlot, item: &mut EquipmentItemViewModel, gaitem_handle: u32) {
            item.gaitem_handle = gaitem_handle;
            item.id = if gaitem_handle != 0 {gaitem_handle ^ InventoryGaitemType::ITEM as u32} else {0};
            item.equip_index = Self::equip_index(slot, gaitem_handle);
            item.name = Self::name_or_empty(Regulation::equip_goods_param_map(), item.id);
        }

        pub fn equip_index(slot:& SaveSlot, gaitem_handle: u32) -> u32 {
            if gaitem_handle == 0 {return u32::MAX;}
            match slot.equip_inventory_data.common_items.iter().position(|i| i.ga_item_handle == gaitem_handle) {
                Some(index) => index as u32 + 0x180,
                None => 0,
            }
        }

        fn name_or_empty<T>(map: &HashMap<u32, Row<T>>, id: u32) -> String where T: Default + Clone{
            let res = map.get(&id);
            match res {
                Some(row) => row.name.to_string(),
                None => "Empty".to_string(),
            }
        }
    }
}