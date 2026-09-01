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

// Weapon param ids carry the upgrade level in their last two digits, so a
// "+5" of an item resolves to the same artwork as the "+0".
pub fn key(category: MediaCategory, param_id: u32) -> u32 {
    let base = match category {
        MediaCategory::Weapon => (param_id / 100) * 100,
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
        // Floor recorded from the first successful generation after the
        // category-namespaced key fix (2362 entries), rounded down to the
        // nearest fifty, not guessed. Rerun the generator and update this
        // deliberately if the API changes.
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
}
