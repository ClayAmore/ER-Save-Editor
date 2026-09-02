use crate::media::index::{self, MediaCategory};
use crate::util::regulation::Regulation;

#[derive(Default, Clone, Debug)]
pub struct ItemDetails {
    pub description: Option<String>,
    pub attributes: Vec<(String, String)>,
}

// The description lookup goes through the namespaced media key (see
// media::index::key / Ruling A), since the five equipment param tables are
// independent id spaces and a bare param id could resolve to another
// table's item. The regulation lookups below stay keyed by the raw param
// id, which is how the params themselves are indexed.
fn description(category: MediaCategory, param_id: u32) -> Option<String> {
    index::entry(index::key(category, param_id))
        .map(|entry| entry.description.clone())
        .filter(|text| !text.is_empty())
}

pub fn weapon_details(param_id: u32) -> ItemDetails {
    let mut details = ItemDetails {
        description: description(MediaCategory::Weapon, param_id),
        attributes: Vec::new(),
    };

    if let Some(row) = Regulation::equip_weapon_params_map().get(&param_id) {
        // The param structs are packed, so each field is copied out first.
        let (phys, magic, fire, thunder) = (
            row.data.attackBasePhysics,
            row.data.attackBaseMagic,
            row.data.attackBaseFire,
            row.data.attackBaseThunder,
        );
        let (str_req, dex_req, int_req, fai_req, arc_req) = (
            row.data.properStrength,
            row.data.properAgility,
            row.data.properMagic,
            row.data.properFaith,
            row.data.properLuck,
        );
        let weight = row.data.weight;

        details.attributes.push(("Physical".into(), phys.to_string()));
        details.attributes.push(("Magic".into(), magic.to_string()));
        details.attributes.push(("Fire".into(), fire.to_string()));
        details.attributes.push(("Lightning".into(), thunder.to_string()));
        details.attributes.push((
            "Requires".into(),
            format!("Str {str_req} / Dex {dex_req} / Int {int_req} / Fai {fai_req} / Arc {arc_req}"),
        ));
        details.attributes.push(("Weight".into(), format!("{weight:.1}")));
    }

    details
}

pub fn protector_details(param_id: u32) -> ItemDetails {
    let mut details = ItemDetails {
        description: description(MediaCategory::Armor, param_id),
        attributes: Vec::new(),
    };

    if let Some(row) = Regulation::equip_protectors_param_map().get(&param_id) {
        let (phys, magic, fire, thunder) = (
            row.data.defensePhysics,
            row.data.defenseMagic,
            row.data.defenseFire,
            row.data.defenseThunder,
        );
        let weight = row.data.weight;

        details.attributes.push(("Physical".into(), phys.to_string()));
        details.attributes.push(("Magic".into(), magic.to_string()));
        details.attributes.push(("Fire".into(), fire.to_string()));
        details.attributes.push(("Lightning".into(), thunder.to_string()));
        details.attributes.push(("Weight".into(), format!("{weight:.1}")));
    }

    details
}

pub fn ash_details(param_id: u32) -> ItemDetails {
    ItemDetails {
        description: description(MediaCategory::AshOfWar, param_id),
        attributes: Vec::new(),
    }
}

// The category is passed in rather than inferred by probing both tables:
// the goods and accessory param tables are independent id spaces (see
// media::index) and 88 ids collide between them, so guessing from whichever
// table answers first silently shows the wrong item's stats and description.
pub fn simple_details(category: MediaCategory, param_id: u32) -> ItemDetails {
    let mut details = ItemDetails {
        description: None,
        attributes: Vec::new(),
    };

    match category {
        MediaCategory::Accessory => {
            if let Some(row) = Regulation::equip_accessory_param_map().get(&param_id) {
                details.description = description(MediaCategory::Accessory, param_id);
                let weight = row.data.weight;
                details.attributes.push(("Weight".into(), format!("{weight:.1}")));
            }
        }
        MediaCategory::Goods => {
            if let Some(row) = Regulation::equip_goods_param_map().get(&param_id) {
                details.description = description(MediaCategory::Goods, param_id);
                let max = row.data.maxNum;
                details.attributes.push(("Max held".into(), max.to_string()));
            }
        }
        _ => {}
    }

    details
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_unknown_item_yields_no_description_and_no_attributes() {
        let details = weapon_details(1);
        assert!(details.description.is_none());
        assert!(details.attributes.is_empty());
    }

    #[test]
    fn a_missing_description_does_not_hide_attributes() {
        // Attribute numbers come from the regulation, so an item absent from
        // the media index still shows its stats once params are loaded.
        let details = simple_details(MediaCategory::Goods, 1);
        assert!(details.description.is_none());
    }

    #[test]
    fn an_unrecognized_category_yields_the_default_details_without_panicking() {
        let details = simple_details(MediaCategory::Weapon, 1000);
        assert!(details.description.is_none());
        assert!(details.attributes.is_empty());
    }
}
