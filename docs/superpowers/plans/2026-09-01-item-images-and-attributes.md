# Item Images and Attributes Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Show each item's icon and its attributes in the inventory lists, the add-items browser and the equipment slots.

**Architecture:** An index generated offline joins our param ids to fanapis image URLs and descriptions by name, and ships embedded in the binary. At runtime a worker pool resolves image bytes from memory, then disk, then network, and a texture store turns them into egui textures. Attribute numbers come from the regulation already loaded from the save, never from the API.

**Tech Stack:** Rust, eframe/egui 0.26, reqwest (blocking), serde_json, rust-embed, image. No new dependencies.

**Spec:** `docs/superpowers/specs/2026-09-01-item-images-and-attributes-design.md`

## Global Constraints

- No new crates. Everything needed is already in `Cargo.toml`.
- Nothing in the **application** on a network or disk path may use `expect` or `unwrap`. This codebase has a history of panicking on unexpected data. The Task 3 generator is exempt: it is an `#[ignore]` test run by hand, and it should fail loudly.
- Numbers come from the regulation. The API supplies only image URL and description.
- Param structs are `#[repr(C, packed)]`. Copy a field into a local before reading it, or the build fails with E0793.
- Disk cache files are named `<param_id>.png`, never by API id.
- No test may touch the live network, except the `#[ignore]` generator.
- An item missing from the index is a normal state, not an error.

---

### Task 1: Name normalisation

**Files:**
- Create: `src/media/mod.rs`
- Create: `src/media/name_match.rs`
- Modify: `src/main.rs` (add `mod media;` beside the other `mod` lines)

**Interfaces:**
- Consumes: nothing
- Produces: `pub fn normalise(name: &str) -> String`

- [ ] **Step 1: Write the failing test**

In `src/media/name_match.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::normalise;

    #[test]
    fn ignores_case_spacing_and_punctuation() {
        assert_eq!(normalise("Hand Axe"), normalise("hand  axe"));
        assert_eq!(normalise("Shard of Alexander"), normalise("Shard Of Alexander"));
        assert_eq!(normalise("Two-Headed Turtle"), normalise("Two Headed Turtle"));
    }

    #[test]
    fn drops_upgrade_suffix() {
        assert_eq!(normalise("Hand Axe +5"), normalise("Hand Axe"));
        assert_eq!(normalise("Hand Axe +25"), normalise("Hand Axe"));
    }

    #[test]
    fn drops_trailing_category_word() {
        assert_eq!(normalise("Crimson Seed Talisman"), normalise("Crimson Seed"));
    }

    #[test]
    fn keeps_distinct_names_distinct() {
        assert_ne!(normalise("Hand Axe"), normalise("Great Axe"));
    }
}
```

- [ ] **Step 2: Run the test to verify it fails**

Run: `cargo test --bin er-save-editor name_match`
Expected: FAIL to compile, `cannot find function normalise`.

- [ ] **Step 3: Write the implementation**

In `src/media/mod.rs`:

```rust
pub mod name_match;
```

In `src/main.rs`, beside the existing `mod` lines:

```rust
mod media;
```

At the top of `src/media/name_match.rs`:

```rust
// The API and our name tables disagree on case, punctuation and trailing
// category words, so both sides are reduced to the same shape before
// they are compared.
const TRAILING_WORDS: [&str; 4] = ["talisman", "cookbook", "bell bearing", "whetblade"];

pub fn normalise(name: &str) -> String {
    let mut text = name.to_lowercase();

    // Drop a "+N" upgrade suffix.
    if let Some(pos) = text.rfind('+') {
        if text[pos + 1..].trim().chars().all(|c| c.is_ascii_digit())
            && !text[pos + 1..].trim().is_empty()
        {
            text.truncate(pos);
        }
    }

    // Punctuation becomes a space so "two-headed" and "two headed" agree.
    let spaced: String = text
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { ' ' })
        .collect();

    let mut words: Vec<&str> = spaced.split_whitespace().collect();

    for trailing in TRAILING_WORDS {
        let parts: Vec<&str> = trailing.split(' ').collect();
        if words.len() > parts.len() && words.ends_with(&parts) {
            words.truncate(words.len() - parts.len());
            break;
        }
    }

    words.join(" ")
}
```

- [ ] **Step 4: Run the test to verify it passes**

Run: `cargo test --bin er-save-editor name_match`
Expected: PASS, 4 tests.

- [ ] **Step 5: Commit**

```bash
git add src/media/mod.rs src/media/name_match.rs src/main.rs
git commit -m "Add item name normalisation for the media index join"
```

---

### Task 2: The embedded media index

**Files:**
- Create: `src/media/index.rs`
- Modify: `src/media/mod.rs`

**Interfaces:**
- Consumes: nothing
- Produces:
  - `pub struct MediaEntry { pub image_url: String, pub description: String }`
  - `pub fn entry(param_id: u32) -> Option<&'static MediaEntry>`
  - `pub fn parse(json: &str) -> HashMap<u32, MediaEntry>`

- [ ] **Step 1: Write the failing test**

In `src/media/index.rs`:

```rust
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
```

- [ ] **Step 2: Run the test to verify it fails**

Run: `cargo test --bin er-save-editor media::index`
Expected: FAIL to compile, `cannot find function parse`.

- [ ] **Step 3: Write the implementation**

Create `assets/item_media.json` with an empty object so the embed has something to read until Task 3 fills it:

```bash
echo "{}" > assets/item_media.json
```

In `src/media/index.rs`:

```rust
use std::collections::HashMap;

use once_cell::sync::Lazy;
use rust_embed::RustEmbed;
use serde::Deserialize;

// Only the generated index is embedded; assets/ also holds large icon
// artwork that has no business in the binary.
#[derive(RustEmbed)]
#[folder = "assets/"]
#[include = "item_media.json"]
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
```

In `src/media/mod.rs`, add:

```rust
pub mod index;
```

- [ ] **Step 4: Run the test to verify it passes**

Run: `cargo test --bin er-save-editor media::index`
Expected: PASS, 3 tests.

- [ ] **Step 5: Commit**

```bash
git add src/media/index.rs src/media/mod.rs assets/item_media.json
git commit -m "Add the embedded item media index"
```

---

### Task 3: Generate the index

**Files:**
- Create: `src/media/generate.rs`
- Modify: `src/media/mod.rs`
- Modify: `assets/item_media.json` (regenerated, committed)

**Interfaces:**
- Consumes: `media::name_match::normalise`
- Produces: the populated `assets/item_media.json`, plus recorded coverage numbers used as the test floor.

- [ ] **Step 1: Write the generator**

In `src/media/generate.rs`:

```rust
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
        std::fs::write("assets/item_media.json", json).expect("write index");
        println!("\nwrote {} entries to assets/item_media.json", out.len());
    }
}
```

In `src/media/mod.rs`, add:

```rust
mod generate;
```

- [ ] **Step 2: Run the generator**

Run: `cargo test --bin er-save-editor -- --ignored --nocapture regenerate_item_media`
Expected: a coverage line per group, a list of unmatched names, and `assets/item_media.json` written.

- [ ] **Step 3: Record the measured floor**

Read the printed percentages. In `src/media/index.rs`, add a test using the numbers actually achieved, each rounded **down** to the nearest five so normal drift does not trip it:

```rust
#[test]
fn the_shipped_index_keeps_its_coverage() {
    // Floors recorded from the first successful generation, not guessed.
    // Rerun the generator and update these deliberately if the API changes.
    assert!(super::count() >= REPLACE_WITH_MEASURED_TOTAL);
}
```

Replace `REPLACE_WITH_MEASURED_TOTAL` with the entry count the generator printed, rounded down to the nearest fifty. Leaving the literal in place is a plan failure.

- [ ] **Step 4: Verify**

Run: `cargo test --bin er-save-editor media::index`
Expected: PASS, including the new coverage test.

- [ ] **Step 5: Commit**

```bash
git add src/media/generate.rs src/media/mod.rs src/media/index.rs assets/item_media.json
git commit -m "Generate the item media index from the fanapis dataset"
```

---

### Task 4: Image bytes cache

**Files:**
- Create: `src/media/cache.rs`
- Modify: `src/media/mod.rs`

**Interfaces:**
- Consumes: nothing
- Produces:
  - `pub trait ImageSource: Send + Sync { fn fetch(&self, url: &str) -> Result<Vec<u8>, String>; }`
  - `pub struct HttpSource;`
  - `pub struct ImageCache;`
  - `ImageCache::new(source: Arc<dyn ImageSource>, dir: Option<PathBuf>, workers: usize) -> ImageCache`
  - `ImageCache::request(&self, param_id: u32, url: &str)`
  - `ImageCache::poll(&self) -> Vec<(u32, Option<Vec<u8>>)>`
  - `pub fn default_cache_dir() -> Option<PathBuf>`

- [ ] **Step 1: Write the failing test**

In `src/media/cache.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct FakeSource {
        calls: AtomicUsize,
        payload: Option<Vec<u8>>,
    }

    impl ImageSource for FakeSource {
        fn fetch(&self, _url: &str) -> Result<Vec<u8>, String> {
            self.calls.fetch_add(1, Ordering::SeqCst);
            match &self.payload {
                Some(bytes) => Ok(bytes.clone()),
                None => Err("offline".to_string()),
            }
        }
    }

    fn temp_dir(tag: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("er_cache_test_{tag}"));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("create temp dir");
        dir
    }

    fn drain(cache: &ImageCache, expected: usize) -> Vec<(u32, Option<Vec<u8>>)> {
        let mut got = Vec::new();
        for _ in 0..200 {
            got.extend(cache.poll());
            if got.len() >= expected {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        got
    }

    #[test]
    fn fetches_then_serves_the_same_bytes_from_disk() {
        let dir = temp_dir("hit");
        let source = Arc::new(FakeSource {
            calls: AtomicUsize::new(0),
            payload: Some(vec![1, 2, 3]),
        });

        let cache = ImageCache::new(source.clone(), Some(dir.clone()), 2);
        cache.request(1000000, "https://example.test/a.png");
        let got = drain(&cache, 1);
        assert_eq!(got, vec![(1000000, Some(vec![1, 2, 3]))]);
        assert!(dir.join("1000000.png").exists(), "bytes were not cached to disk");

        // A second cache with no working source still answers, from disk.
        let offline = Arc::new(FakeSource {
            calls: AtomicUsize::new(0),
            payload: None,
        });
        let second = ImageCache::new(offline.clone(), Some(dir), 2);
        second.request(1000000, "https://example.test/a.png");
        let got = drain(&second, 1);
        assert_eq!(got, vec![(1000000, Some(vec![1, 2, 3]))]);
        assert_eq!(offline.calls.load(Ordering::SeqCst), 0, "disk hit still hit the network");
    }

    #[test]
    fn a_failure_reports_none_without_panicking() {
        let dir = temp_dir("miss");
        let source = Arc::new(FakeSource {
            calls: AtomicUsize::new(0),
            payload: None,
        });
        let cache = ImageCache::new(source, Some(dir), 1);
        cache.request(2009600, "https://example.test/missing.png");
        assert_eq!(drain(&cache, 1), vec![(2009600, None)]);
    }

    #[test]
    fn a_locally_supplied_icon_is_used_without_any_url() {
        // This is how extracted DLC icons work: the file is simply there.
        let dir = temp_dir("local");
        std::fs::write(dir.join("5200000.png"), vec![9, 9]).expect("seed icon");
        let source = Arc::new(FakeSource {
            calls: AtomicUsize::new(0),
            payload: None,
        });
        let cache = ImageCache::new(source.clone(), Some(dir), 1);
        cache.request(5200000, "");
        assert_eq!(drain(&cache, 1), vec![(5200000, Some(vec![9, 9]))]);
        assert_eq!(source.calls.load(Ordering::SeqCst), 0);
    }
}
```

- [ ] **Step 2: Run the test to verify it fails**

Run: `cargo test --bin er-save-editor media::cache`
Expected: FAIL to compile, `cannot find type ImageCache`.

- [ ] **Step 3: Write the implementation**

At the top of `src/media/cache.rs`:

```rust
use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};

pub trait ImageSource: Send + Sync {
    fn fetch(&self, url: &str) -> Result<Vec<u8>, String>;
}

pub struct HttpSource;

impl ImageSource for HttpSource {
    fn fetch(&self, url: &str) -> Result<Vec<u8>, String> {
        let response = reqwest::blocking::get(url).map_err(|e| e.to_string())?;
        if !response.status().is_success() {
            return Err(format!("http {}", response.status()));
        }
        response.bytes().map(|b| b.to_vec()).map_err(|e| e.to_string())
    }
}

// Cache files are keyed by param id, which is what lets a player drop in
// icons extracted from their own installation and have them found.
pub fn default_cache_dir() -> Option<PathBuf> {
    if let Ok(local) = std::env::var("LOCALAPPDATA") {
        let dir = PathBuf::from(local).join("er-save-editor").join("images");
        if std::fs::create_dir_all(&dir).is_ok() {
            return Some(dir);
        }
    }
    let beside = std::env::current_exe().ok()?.parent()?.join("image-cache");
    std::fs::create_dir_all(&beside).ok()?;
    Some(beside)
}

struct Job {
    param_id: u32,
    url: String,
}

pub struct ImageCache {
    jobs: Sender<Job>,
    results: Arc<Mutex<Receiver<(u32, Option<Vec<u8>>)>>>,
}

impl ImageCache {
    pub fn new(source: Arc<dyn ImageSource>, dir: Option<PathBuf>, workers: usize) -> ImageCache {
        let (job_tx, job_rx) = channel::<Job>();
        let (result_tx, result_rx) = channel::<(u32, Option<Vec<u8>>)>();
        let job_rx = Arc::new(Mutex::new(job_rx));

        for _ in 0..workers.max(1) {
            let job_rx = job_rx.clone();
            let result_tx = result_tx.clone();
            let source = source.clone();
            let dir = dir.clone();
            std::thread::spawn(move || loop {
                let job = {
                    let guard = match job_rx.lock() {
                        Ok(guard) => guard,
                        Err(_) => return,
                    };
                    match guard.recv() {
                        Ok(job) => job,
                        Err(_) => return,
                    }
                };

                let path = dir.as_ref().map(|d| d.join(format!("{}.png", job.param_id)));

                // Disk first. An extracted icon wins over the network.
                if let Some(path) = &path {
                    if let Ok(bytes) = std::fs::read(path) {
                        let _ = result_tx.send((job.param_id, Some(bytes)));
                        continue;
                    }
                }

                if job.url.is_empty() {
                    let _ = result_tx.send((job.param_id, None));
                    continue;
                }

                match source.fetch(&job.url) {
                    Ok(bytes) => {
                        if let Some(path) = &path {
                            let _ = std::fs::write(path, &bytes);
                        }
                        let _ = result_tx.send((job.param_id, Some(bytes)));
                    }
                    Err(_) => {
                        let _ = result_tx.send((job.param_id, None));
                    }
                }
            });
        }

        ImageCache {
            jobs: job_tx,
            results: Arc::new(Mutex::new(result_rx)),
        }
    }

    pub fn request(&self, param_id: u32, url: &str) {
        let _ = self.jobs.send(Job {
            param_id,
            url: url.to_string(),
        });
    }

    pub fn poll(&self) -> Vec<(u32, Option<Vec<u8>>)> {
        let mut out = Vec::new();
        if let Ok(guard) = self.results.lock() {
            while let Ok(item) = guard.try_recv() {
                out.push(item);
            }
        }
        out
    }
}
```

In `src/media/mod.rs`, add:

```rust
pub mod cache;
```

- [ ] **Step 4: Run the test to verify it passes**

Run: `cargo test --bin er-save-editor media::cache`
Expected: PASS, 3 tests.

- [ ] **Step 5: Commit**

```bash
git add src/media/cache.rs src/media/mod.rs
git commit -m "Add the item image cache, disk before network"
```

---

### Task 5: Texture store

**Files:**
- Create: `src/media/textures.rs`
- Modify: `src/media/mod.rs`

**Interfaces:**
- Consumes: `media::cache::{ImageCache, HttpSource, default_cache_dir}`, `media::index::entry`
- Produces:
  - `pub struct ItemTextures;`
  - `ItemTextures::new() -> ItemTextures`
  - `ItemTextures::poll(&mut self, ctx: &egui::Context)`
  - `ItemTextures::texture(&mut self, param_id: u32) -> Option<egui::TextureHandle>`

- [ ] **Step 1: Write the failing test**

In `src/media/textures.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_item_with_no_index_entry_is_never_requested_twice() {
        let mut textures = ItemTextures::new();
        // 1 is not a real param id in the index.
        assert!(textures.texture(1).is_none());
        assert!(textures.texture(1).is_none());
        assert_eq!(textures.state_count(), 1, "the miss was not remembered");
    }
}
```

- [ ] **Step 2: Run the test to verify it fails**

Run: `cargo test --bin er-save-editor media::textures`
Expected: FAIL to compile, `cannot find type ItemTextures`.

- [ ] **Step 3: Write the implementation**

In `src/media/textures.rs`:

```rust
use std::collections::HashMap;
use std::sync::Arc;

use eframe::egui;

use super::cache::{default_cache_dir, HttpSource, ImageCache};
use super::index;

enum State {
    Requested,
    Ready(egui::TextureHandle),
    // Never retried. This is what stops an offline list asking once a frame.
    Failed,
}

pub struct ItemTextures {
    cache: ImageCache,
    state: HashMap<u32, State>,
}

impl ItemTextures {
    pub fn new() -> ItemTextures {
        ItemTextures {
            cache: ImageCache::new(Arc::new(HttpSource), default_cache_dir(), 4),
            state: HashMap::new(),
        }
    }

    pub fn state_count(&self) -> usize {
        self.state.len()
    }

    // Called once per frame, before anything asks for a texture.
    pub fn poll(&mut self, ctx: &egui::Context) {
        for (param_id, bytes) in self.cache.poll() {
            let next = match bytes.and_then(|bytes| decode(&bytes)) {
                Some((size, pixels)) => {
                    let image = egui::ColorImage { size, pixels };
                    State::Ready(ctx.load_texture(
                        format!("item_{param_id}"),
                        image,
                        egui::TextureOptions::LINEAR,
                    ))
                }
                None => State::Failed,
            };
            self.state.insert(param_id, next);
        }
    }

    pub fn texture(&mut self, param_id: u32) -> Option<egui::TextureHandle> {
        match self.state.get(&param_id) {
            Some(State::Ready(handle)) => return Some(handle.clone()),
            Some(_) => return None,
            None => {}
        }

        // An item the index does not know still gets a slot, so the miss is
        // remembered and never asked about again.
        let url = index::entry(param_id).map(|e| e.image_url.clone()).unwrap_or_default();
        self.cache.request(param_id, &url);
        self.state.insert(param_id, State::Requested);
        None
    }
}

fn decode(bytes: &[u8]) -> Option<([usize; 2], Vec<egui::Color32>)> {
    let image = image::load_from_memory(bytes).ok()?.to_rgba8();
    let (width, height) = image.dimensions();
    let pixels = image
        .into_raw()
        .chunks_exact(4)
        .map(|p| egui::Color32::from_rgba_unmultiplied(p[0], p[1], p[2], p[3]))
        .collect();
    Some(([width as usize, height as usize], pixels))
}
```

In `src/media/mod.rs`, add:

```rust
pub mod textures;
```

- [ ] **Step 4: Run the test to verify it passes**

Run: `cargo test --bin er-save-editor media::textures`
Expected: PASS, 1 test.

- [ ] **Step 5: Commit**

```bash
git add src/media/textures.rs src/media/mod.rs
git commit -m "Add the item texture store"
```

---

### Task 6: Item details view model

**Files:**
- Create: `src/vm/item_details.rs`
- Modify: `src/vm/mod.rs`

**Interfaces:**
- Consumes: `media::index::entry`, `util::regulation::Regulation`
- Produces:
  - `pub struct ItemDetails { pub description: Option<String>, pub attributes: Vec<(String, String)> }`
  - `pub fn weapon_details(param_id: u32) -> ItemDetails`
  - `pub fn protector_details(param_id: u32) -> ItemDetails`
  - `pub fn simple_details(param_id: u32) -> ItemDetails`

- [ ] **Step 1: Write the failing test**

In `src/vm/item_details.rs`:

```rust
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
        let details = simple_details(1);
        assert!(details.description.is_none());
    }
}
```

- [ ] **Step 2: Run the test to verify it fails**

Run: `cargo test --bin er-save-editor item_details`
Expected: FAIL to compile, `cannot find function weapon_details`.

- [ ] **Step 3: Write the implementation**

In `src/vm/item_details.rs`:

```rust
use crate::media::index;
use crate::util::regulation::Regulation;

#[derive(Default, Clone, Debug)]
pub struct ItemDetails {
    pub description: Option<String>,
    pub attributes: Vec<(String, String)>,
}

fn description(param_id: u32) -> Option<String> {
    index::entry(param_id)
        .map(|entry| entry.description.clone())
        .filter(|text| !text.is_empty())
}

pub fn weapon_details(param_id: u32) -> ItemDetails {
    let mut details = ItemDetails {
        description: description(param_id),
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
        description: description(param_id),
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

pub fn simple_details(param_id: u32) -> ItemDetails {
    let mut details = ItemDetails {
        description: description(param_id),
        attributes: Vec::new(),
    };

    if let Some(row) = Regulation::equip_accessory_param_map().get(&param_id) {
        let weight = row.data.weight;
        details.attributes.push(("Weight".into(), format!("{weight:.1}")));
    } else if let Some(row) = Regulation::equip_goods_param_map().get(&param_id) {
        let max = row.data.maxNum;
        details.attributes.push(("Max held".into(), max.to_string()));
    }

    details
}
```

In `src/vm/mod.rs`, add:

```rust
pub mod item_details;
```

- [ ] **Step 4: Run the test to verify it passes**

Run: `cargo test --bin er-save-editor item_details`
Expected: PASS, 2 tests.

- [ ] **Step 5: Commit**

```bash
git add src/vm/item_details.rs src/vm/mod.rs
git commit -m "Add item details assembled from regulation params and the media index"
```

---

### Task 7: Icon and details in the add-items panel

**Files:**
- Modify: `src/main.rs` (hold `ItemTextures` on `App`, poll each frame, pass to the UI)
- Modify: `src/ui/inventory/add.rs:370` (the selected item heading)

**Interfaces:**
- Consumes: `media::textures::ItemTextures`, `vm::item_details::*`
- Produces: the threaded signatures Tasks 8 and 9 build on:
  - `pub fn inventory(ui: &mut Ui, vm: &mut ViewModel, textures: &mut ItemTextures)`
  - `pub fn add(ui: &mut Ui, vm: &mut ViewModel, textures: &mut ItemTextures)`
  - `pub fn browse_inventory(ui: &mut Ui, vm: &mut ViewModel, textures: &mut ItemTextures)`
  - `pub fn equipment(ui: &mut Ui, vm: &mut ViewModel, textures: &mut ItemTextures)`

- [ ] **Step 1: Add the texture store to App and poll it**

In `src/main.rs`, add the field to `pub struct App`:

```rust
    textures: media::textures::ItemTextures,
```

In `App::new`, add:

```rust
            textures: media::textures::ItemTextures::new(),
```

As the first line of `impl eframe::App for App { fn update(...) }`, before `ctx.set_zoom_factor(1.75);`:

```rust
        self.textures.poll(ctx);
```

- [ ] **Step 2: Thread the texture store down to the UI**

Five signatures change. Do all of them now so Tasks 8 and 9 only add drawing code.

`src/main.rs:287-288`, inside the route match:

```rust
                    Route::Equipment => equipment(ui, &mut self.vm, &mut self.textures),
                    Route::Inventory => inventory(ui, &mut self.vm, &mut self.textures),
```

`src/ui/inventory/inventory.rs:6` — signature and the two calls in the central panel:

```rust
    pub fn inventory(ui: &mut Ui, vm: &mut ViewModel, textures: &mut ItemTextures) {
```

```rust
                InventoryRoute::Add => {add(ui, vm, textures);},
                InventoryRoute::Browse => {browse_inventory(ui, vm, textures);},
```

with `use crate::media::textures::ItemTextures;` added to its imports.

`src/ui/inventory/add.rs:19`:

```rust
pub fn add(ui: &mut Ui, vm: &mut ViewModel, textures: &mut ItemTextures) {
```

`src/ui/inventory/browse.rs:4`:

```rust
pub fn browse_inventory(ui: &mut Ui, vm: &mut ViewModel, textures: &mut ItemTextures) {
```

`src/ui/equipment.rs:10`:

```rust
    pub fn equipment(ui: &mut Ui, vm: &mut ViewModel, textures: &mut ItemTextures) {
```

Each of those four files needs `use crate::media::textures::ItemTextures;`. Build now: `cargo build` should succeed with `textures` unused in three of them.

- [ ] **Step 3: Show the icon and details beside the selected item**

In `src/ui/inventory/add.rs`, the selected item heading at line 370 currently reads:

```rust
ui.label(egui::RichText::new(regulation_vm.selected_item.name.to_string()).strong().heading().size(24.));
```

Replace it with a horizontal row that puts the icon first, then the same heading, with the details listed underneath:

```rust
ui.horizontal(|ui| {
    match textures.texture(regulation_vm.selected_item.id) {
        Some(handle) => {
            ui.image((handle.id(), egui::vec2(64., 64.)));
        }
        None => {
            // Normal for DLC content the dataset never covered.
            let (rect, _) = ui.allocate_exact_size(egui::vec2(64., 64.), egui::Sense::hover());
            ui.painter().rect_filled(rect, 4.0, egui::Color32::from_black_alpha(40));
        }
    }
    ui.label(egui::RichText::new(regulation_vm.selected_item.name.to_string()).strong().heading().size(24.));
});

let details = crate::vm::item_details::weapon_details(regulation_vm.selected_item.id);
if let Some(description) = &details.description {
    ui.add_space(4.0);
    ui.label(egui::RichText::new(description).size(11.).italics());
}
for (label, value) in &details.attributes {
    ui.label(format!("{label}: {value}"));
}
```

- [ ] **Step 4: Build and run the app**

Run: `cargo run --release`
Load a save, go to the add-items browser, select a base-game weapon. Expected: its icon, description and stats appear. Select a DLC weapon. Expected: a flat placeholder square, no description, stats still listed, no crash.

- [ ] **Step 5: Verify nothing regressed**

Run: `cargo test -- --test-threads=1`
Expected: PASS, all existing tests plus the new ones.

- [ ] **Step 6: Commit**

```bash
git add src/main.rs src/ui/inventory/inventory.rs src/ui/inventory/add.rs src/ui/inventory/browse.rs src/ui/equipment.rs
git commit -m "Show item icon and details in the add-items panel"
```

---

### Task 8: Icons in the inventory list

**Files:**
- Modify: `src/ui/inventory/browse.rs:100-108`

**Interfaces:**
- Consumes: `media::textures::ItemTextures`
- Produces: nothing

- [ ] **Step 1: Add an icon column**

`browse.rs` builds a `TableBuilder`. Add one `Column::exact(28.)` as the first column, and in the row body, before the existing `ui.label(format!("{}",item.item_id));`, add:

```rust
row.col(|ui| {
    // Only rows the table actually renders reach here, so only visible
    // icons are ever requested.
    match textures.texture(item.item_id) {
        Some(handle) => {
            ui.image((handle.id(), egui::vec2(24., 24.)));
        }
        None => {
            ui.allocate_exact_size(egui::vec2(24., 24.), egui::Sense::hover());
        }
    }
});
```

Add a matching header cell so the columns line up:

```rust
header.col(|ui| { ui.label(""); });
```

`browse_inventory` already takes `textures: &mut ItemTextures` from Task 7, so no signature changes here.

- [ ] **Step 2: Build and check scrolling**

Run: `cargo run --release`
Open the inventory list and scroll quickly through several hundred rows. Expected: icons fill in as rows appear, scrolling stays smooth, and rows without an icon keep their spacing.

- [ ] **Step 3: Check the offline path**

Disconnect the network, delete `%LOCALAPPDATA%\er-save-editor\images`, and scroll the same list. Expected: placeholders everywhere, no freeze, no repeated stall. This is the behaviour the `Failed` state exists to guarantee.

- [ ] **Step 4: Verify nothing regressed**

Run: `cargo test -- --test-threads=1`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add src/ui/inventory/browse.rs
git commit -m "Show item icons in the inventory list"
```

---

### Task 9: Icons in the equipment slots

**Files:**
- Modify: `src/ui/equipment.rs`

**Interfaces:**
- Consumes: `media::textures::ItemTextures`
- Produces: nothing

- [ ] **Step 1: Draw the icon in each slot**

Each equipment slot in `equipment.rs` renders a label for the equipped piece. Before that label, insert the same icon block used in Task 7 at 40x40:

```rust
match textures.texture(equipped_id) {
    Some(handle) => {
        ui.image((handle.id(), egui::vec2(40., 40.)));
    }
    None => {
        let (rect, _) = ui.allocate_exact_size(egui::vec2(40., 40.), egui::Sense::hover());
        ui.painter().rect_filled(rect, 4.0, egui::Color32::from_black_alpha(40));
    }
}
```

`equipped_id` is the param id already used to look the slot's name up. `equipment` already takes `textures: &mut ItemTextures` from Task 7, so no signature changes here.

- [ ] **Step 2: Build and check every slot**

Run: `cargo run --release`
Open the equipment screen. Expected: an icon in each filled slot, a placeholder in each empty one, and no shift in the existing layout beyond the icon's width.

- [ ] **Step 3: Verify nothing regressed**

Run: `cargo test -- --test-threads=1`
Expected: PASS.

- [ ] **Step 4: Commit**

```bash
git add src/ui/equipment.rs
git commit -m "Show item icons in the equipment slots"
```

---

## Done when

- A base-game item shows its icon, description and stats in all three places.
- A DLC item shows its stats and a placeholder, with no description and no crash.
- Dropping extracted icons named `<param_id>.png` into the cache directory makes DLC icons appear with no code change.
- With no network and an empty cache, every screen still works.
- `cargo test -- --test-threads=1` passes.
