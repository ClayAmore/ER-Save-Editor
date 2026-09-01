// Run by hand, never in CI:
//   cargo test --bin er-save-editor -- --ignored regenerate_item_media
//
// The name join is fragile, so it happens here, once, offline, with a
// report a person reads, rather than on a user's machine.
#[cfg(test)]
mod generator {
    use std::collections::HashMap;

    use crate::db::{
        accessory_name::accessory_name::ACCESSORY_NAME, aow_name::aow_name::AOW_NAME,
        armor_name::armor_name::ARMOR_NAME, item_name::item_name::ITEM_NAME,
        weapon_name::weapon_name::WEAPON_NAME,
    };
    use crate::media::name_match::normalise;

    const BASE: &str = "https://eldenring.fanapis.com/api";

    fn fetch_endpoint(endpoint: &str) -> Vec<(String, String, String)> {
        let mut out = Vec::new();
        let client = reqwest::blocking::Client::new();
        for page in 0..40 {
            let url = format!("{BASE}/{endpoint}?limit=100&page={page}");
            let body = match client.get(&url).send().and_then(|r| r.text()) {
                Ok(body) => body,
                Err(err) => {
                    println!("  {endpoint} page {page} failed: {err}");
                    break;
                }
            };
            let value: serde_json::Value = match serde_json::from_str(&body) {
                Ok(value) => value,
                Err(_) => break,
            };
            let rows = match value.get("data").and_then(|d| d.as_array()) {
                Some(rows) => rows.clone(),
                None => break,
            };
            let got = rows.len();
            for row in rows {
                let name = row.get("name").and_then(|v| v.as_str()).unwrap_or("");
                let image = row.get("image").and_then(|v| v.as_str()).unwrap_or("");
                let desc = row.get("description").and_then(|v| v.as_str()).unwrap_or("");
                if !name.is_empty() && !image.is_empty() {
                    out.push((name.to_string(), image.to_string(), desc.to_string()));
                }
            }
            if got < 100 {
                break;
            }
        }
        out
    }

    #[test]
    #[ignore]
    fn regenerate_item_media() {
        // Every endpoint is pulled, then matched against every param id we
        // have a name for. Nothing is skipped on the assumption that it will
        // not be found, so new API content is picked up by rerunning this.
        let groups: [(&str, Vec<&str>); 5] = [
            ("weapons", vec!["weapons", "shields", "ammos"]),
            ("goods", vec!["items", "sorceries", "incantations", "spirits"]),
            ("armor", vec!["armors"]),
            ("talismans", vec!["talismans"]),
            ("ashes", vec!["ashes"]),
        ];

        let mut api: HashMap<&str, HashMap<String, (String, String)>> = HashMap::new();
        for (group, endpoints) in &groups {
            let mut by_name = HashMap::new();
            for endpoint in endpoints {
                let rows = fetch_endpoint(endpoint);
                println!("fetched {} rows from {endpoint}", rows.len());
                for (name, image, desc) in rows {
                    by_name.insert(normalise(&name), (image, desc));
                }
            }
            api.insert(group, by_name);
        }

        let tables: [(&str, Vec<(u32, String)>); 5] = [
            ("weapons", WEAPON_NAME.lock().unwrap().iter().map(|(k, v)| (*k, v.to_string())).collect()),
            ("goods", ITEM_NAME.lock().unwrap().iter().map(|(k, v)| (*k, v.to_string())).collect()),
            ("armor", ARMOR_NAME.lock().unwrap().iter().map(|(k, v)| (*k, v.to_string())).collect()),
            ("talismans", ACCESSORY_NAME.lock().unwrap().iter().map(|(k, v)| (*k, v.to_string())).collect()),
            ("ashes", AOW_NAME.lock().unwrap().iter().map(|(k, v)| (*k, v.to_string())).collect()),
        ];

        let mut out: HashMap<String, serde_json::Value> = HashMap::new();
        println!("\ncoverage");
        for (group, rows) in &tables {
            let lookup = &api[group];
            let mut matched = 0usize;
            let mut named = 0usize;
            let mut misses: Vec<String> = Vec::new();
            for (id, name) in rows {
                if name.is_empty() {
                    continue;
                }
                named += 1;
                match lookup.get(&normalise(name)) {
                    Some((image, desc)) => {
                        matched += 1;
                        out.insert(
                            id.to_string(),
                            serde_json::json!({"image_url": image, "description": desc}),
                        );
                    }
                    None => misses.push(name.clone()),
                }
            }
            let pct = if named == 0 { 0 } else { matched * 100 / named };
            println!("  {group}: {matched} of {named} named rows matched ({pct}%)");
            misses.sort();
            misses.dedup();
            for miss in misses.iter().take(40) {
                println!("      unmatched: {miss}");
            }
            if misses.len() > 40 {
                println!("      ... and {} more", misses.len() - 40);
            }
        }

        let json = serde_json::to_string_pretty(&out).expect("serialise index");
        std::fs::write("assets/media/item_media.json", json).expect("write index");
        println!("\nwrote {} entries to assets/media/item_media.json", out.len());
    }
}
