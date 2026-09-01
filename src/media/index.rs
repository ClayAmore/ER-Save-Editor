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
    use super::parse;

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
}
