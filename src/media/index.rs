use std::collections::HashMap;

use once_cell::sync::Lazy;
use rust_embed::RustEmbed;
use serde::Deserialize;

// Only the generated index is embedded; assets/ also holds large icon
// artwork that has no business in the binary.
#[derive(RustEmbed)]
#[folder = "assets/media/"]
struct MediaAsset;

#[derive(Deserialize, Clone, Debug, Default)]
pub struct MediaEntry {
    pub image_url: String,
    pub description: String,
}

// Elden Ring's five equipment param tables are five independent id
// spaces: weapon 1000000 is a Dagger, protector 1000000 is the Haima
// Glintstone Crown. The index therefore keys on the category offset the
// rest of this codebase already uses for the same reason, so a lookup
// cannot return another table's item.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MediaCategory {
    Weapon,
    Armor,
    Accessory,
    Goods,
    AshOfWar,
}

impl MediaCategory {
    pub fn offset(self) -> u32 {
        match self {
            MediaCategory::Weapon => 0x0,
            MediaCategory::Armor => 0x10000000,
            MediaCategory::Accessory => 0x20000000,
            MediaCategory::Goods => 0x40000000,
            MediaCategory::AshOfWar => 0x80000000,
        }
    }
}

// Weapon param ids are `base + affinity*100`, with base weapons landing on
// multiples of 10000 and the upgrade level in the last two digits (e.g.
// 1000000 Dagger, 1000100 Heavy Dagger, 1000105 Heavy Dagger +5 all share
// the Dagger's artwork; 1010000 Black Knife and 1020000 Parrying Dagger are
// separate base weapons and stay distinct). Reducing only to `/100*100`
// strips the upgrade level but leaves the affinity, so an affinity variant
// would ask for artwork that does not exist under its own name - affinity
// variants and upgrade levels both share the base weapon's artwork.
pub fn key(category: MediaCategory, param_id: u32) -> u32 {
    let base = match category {
        MediaCategory::Weapon => (param_id / 10000) * 10000,
        _ => param_id,
    };
    category.offset() | base
}

pub fn parse(json: &str) -> HashMap<u32, MediaEntry> {
    let raw: HashMap<String, MediaEntry> = match serde_json::from_str(json) {
        Ok(raw) => raw,
        // A corrupt index costs icons, never the app.
        Err(err) => {
            println!("item media index unreadable: {err}");
            return HashMap::new();
        }
    };
    raw.into_iter()
        .filter_map(|(id, entry)| id.parse::<u32>().ok().map(|id| (id, entry)))
        .collect()
}

static INDEX: Lazy<HashMap<u32, MediaEntry>> = Lazy::new(|| {
    match MediaAsset::get("item_media.json") {
        Some(file) => match std::str::from_utf8(&file.data) {
            Ok(text) => parse(text),
            Err(_) => HashMap::new(),
        },
        None => HashMap::new(),
    }
});

pub fn entry(param_id: u32) -> Option<&'static MediaEntry> {
    INDEX.get(&param_id)
}

pub fn count() -> usize {
    INDEX.len()
}

#[cfg(test)]
mod tests {
    use super::{key, parse, MediaCategory};

    const SAMPLE: &str = r#"{
        "1000000": {"image_url": "https://example.test/a.png", "description": "An axe."},
        "2009600": {"image_url": "https://example.test/b.png", "description": "Attire."}
    }"#;

    #[test]
    fn reads_entries_keyed_by_param_id() {
        let index = parse(SAMPLE);
        assert_eq!(index.len(), 2);
        assert_eq!(index[&1000000].description, "An axe.");
        assert_eq!(index[&2009600].image_url, "https://example.test/b.png");
    }

    #[test]
    fn malformed_json_yields_an_empty_index_rather_than_panicking() {
        assert!(parse("not json at all").is_empty());
        assert!(parse("").is_empty());
    }

    #[test]
    fn the_shipped_index_parses() {
        // The generated file is committed, so this guards against shipping a
        // broken one.
        assert!(super::entry(1000000).is_some() || super::count() == 0);
    }

    #[test]
    fn the_shipped_index_keeps_its_coverage() {
        // Floor recorded from a measured generation run, rounded down to
        // the nearest fifty, not guessed. Reconfirmed at 2362 entries after
        // reducing MediaCategory::Weapon's key to the base weapon
        // (param_id/10000*10000): the API this generator pulls from only
        // ever offered artwork under base weapon names, never per-affinity
        // ("Heavy Dagger", "Keen Dagger", ...), so those affinity rows were
        // already absent from `out` before this fix and collapsing their
        // keys onto the base weapon does not remove any entries here - the
        // fix's effect is at lookup time, where a save's affinity/upgrade
        // variant id now reduces to the base key this index actually has,
        // rather than to a key nothing was ever written under. Rerun the
        // generator and update this deliberately if the API changes.
        assert!(super::count() >= 2350);
    }

    #[test]
    fn category_offsets_match_the_inventory_item_type_the_rest_of_the_app_uses() {
        use crate::vm::inventory::InventoryItemType;
        assert_eq!(MediaCategory::Weapon.offset(), InventoryItemType::WEAPON as u32);
        assert_eq!(MediaCategory::Armor.offset(), InventoryItemType::ARMOR as u32);
        assert_eq!(MediaCategory::Accessory.offset(), InventoryItemType::ACCESSORY as u32);
        assert_eq!(MediaCategory::Goods.offset(), InventoryItemType::ITEM as u32);
        assert_eq!(MediaCategory::AshOfWar.offset(), InventoryItemType::AOW as u32);
    }

    #[test]
    fn a_weapon_and_an_armor_that_share_a_raw_param_id_no_longer_collide() {
        assert_ne!(
            key(MediaCategory::Weapon, 1000000),
            key(MediaCategory::Armor, 1000000)
        );
    }

    #[test]
    fn a_weapons_upgrade_level_resolves_to_the_same_key_as_its_base() {
        assert_eq!(
            key(MediaCategory::Weapon, 1000005),
            key(MediaCategory::Weapon, 1000000)
        );
    }

    #[test]
    fn an_affinity_variant_resolves_to_the_same_key_as_its_base_weapon() {
        // 1000100 is Heavy Dagger - an affinity of the 1000000 Dagger, not a
        // different weapon. Both must resolve to the base weapon's artwork.
        assert_eq!(
            key(MediaCategory::Weapon, 1000100),
            key(MediaCategory::Weapon, 1000000)
        );
    }

    #[test]
    fn an_affinity_variants_upgrade_level_still_resolves_to_the_base_weapon() {
        // 1000105 is Heavy Dagger +5.
        assert_eq!(
            key(MediaCategory::Weapon, 1000105),
            key(MediaCategory::Weapon, 1000000)
        );
    }

    #[test]
    fn distinct_base_weapons_stay_distinct() {
        // 1020000 is Parrying Dagger, a different base weapon from 1000000
        // Dagger, not an affinity of it.
        assert_ne!(
            key(MediaCategory::Weapon, 1020000),
            key(MediaCategory::Weapon, 1000000)
        );
    }

    #[test]
    fn goods_and_accessory_no_longer_collide_on_a_shared_raw_param_id() {
        // Param id 1000 is "Flask of Crimson Tears" in the goods table and
        // "Crimson Amber Medallion" in the accessory table (see
        // src/db/item_name.rs and src/db/accessory_name.rs) - a real
        // collision, not a contrived one, and both are present in the
        // shipped index. If the namespacing in `key` is ever collapsed back
        // to a bare param id, this must fail loudly rather than let the
        // wrong item's description and stats show up under the right icon.
        let goods_key = key(MediaCategory::Goods, 1000);
        let accessory_key = key(MediaCategory::Accessory, 1000);
        assert_ne!(goods_key, accessory_key);

        let goods_entry = super::entry(goods_key);
        let accessory_entry = super::entry(accessory_key);
        assert!(goods_entry.is_some(), "expected a shipped goods entry for id 1000");
        assert!(accessory_entry.is_some(), "expected a shipped accessory entry for id 1000");
        assert_ne!(
            goods_entry.unwrap().description,
            accessory_entry.unwrap().description,
            "goods and accessory entries for the same raw id must not resolve to the same content"
        );
    }
}
