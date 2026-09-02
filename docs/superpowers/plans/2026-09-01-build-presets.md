# Build Presets Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Save a character's attributes and equipment to a file, and apply that file to any character.

**Architecture:** A preset is a small JSON recipe of attributes plus equipment param ids. Applying it walks a fixed order — add missing items to the inventory, set the equipment slots, set the attributes — because the save cannot equip an item the character does not own. Everything happens on the view model; the existing `update_save` stays the only path to the save file.

**Tech Stack:** Rust, serde, serde_json, eframe/egui 0.26. No new dependencies.

**Spec:** `docs/superpowers/specs/2026-09-01-build-presets-design.md`

## Global Constraints

- No new crates.
- `apply.rs` mutates `SlotViewModel` only. It must never take a `SaveType`.
- Nothing on a disk path may use `expect` or `unwrap`.
- An item id missing from the regulation is skipped and reported, never silently dropped.
- Applying never changes the character's name.
- A preset that fails to parse changes nothing.

---

### Task 1: The preset type

**Files:**
- Create: `src/build/mod.rs`
- Create: `src/build/preset.rs`
- Modify: `src/main.rs` (add `mod build;`)

**Interfaces:**
- Consumes: nothing
- Produces:
  - `pub const FORMAT: u32 = 1;`
  - `pub struct BuildPreset { pub format: u32, pub name: String, pub game_patch: String, pub stats: PresetStats, pub equipment: PresetEquipment }`
  - `pub struct PresetStats { pub arche_type: u8, pub vigor: u32, pub mind: u32, pub endurance: u32, pub strength: u32, pub dexterity: u32, pub intelligence: u32, pub faith: u32, pub arcane: u32 }`
  - `pub struct PresetEquipment { pub right_hand: [Option<PresetItem>; 3], pub left_hand: [Option<PresetItem>; 3], pub head: Option<PresetItem>, pub chest: Option<PresetItem>, pub arms: Option<PresetItem>, pub legs: Option<PresetItem>, pub talismans: [Option<PresetItem>; 4] }`
  - `pub struct PresetItem { pub param_id: u32, pub upgrade: u8 }`
  - `pub fn from_json(text: &str) -> Result<BuildPreset, String>`
  - `pub fn to_json(preset: &BuildPreset) -> Result<String, String>`

- [ ] **Step 1: Write the failing test**

In `src/build/preset.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> BuildPreset {
        BuildPreset {
            format: FORMAT,
            name: "Test Build".to_string(),
            game_patch: "1.17".to_string(),
            stats: PresetStats {
                arche_type: 0,
                vigor: 40,
                mind: 20,
                endurance: 25,
                strength: 50,
                dexterity: 18,
                intelligence: 9,
                faith: 9,
                arcane: 7,
            },
            equipment: PresetEquipment {
                right_hand: [Some(PresetItem { param_id: 1000000, upgrade: 25 }), None, None],
                left_hand: [None, None, None],
                head: Some(PresetItem { param_id: 1000000, upgrade: 0 }),
                chest: None,
                arms: None,
                legs: None,
                talismans: [None, None, None, None],
            },
        }
    }

    #[test]
    fn round_trips_through_json() {
        let text = to_json(&sample()).expect("serialise");
        let back = from_json(&text).expect("parse");
        assert_eq!(back.name, "Test Build");
        assert_eq!(back.stats.strength, 50);
        assert_eq!(back.equipment.right_hand[0].as_ref().map(|i| i.param_id), Some(1000000));
        assert_eq!(back.equipment.right_hand[0].as_ref().map(|i| i.upgrade), Some(25));
    }

    #[test]
    fn refuses_an_unknown_format() {
        let mut preset = sample();
        preset.format = FORMAT + 1;
        let text = to_json(&preset).expect("serialise");
        let err = from_json(&text).expect_err("should refuse");
        assert!(err.contains("format"), "the message should name the problem: {err}");
    }

    #[test]
    fn refuses_junk_without_panicking() {
        assert!(from_json("not json").is_err());
        assert!(from_json("").is_err());
    }
}
```

- [ ] **Step 2: Run the test to verify it fails**

Run: `cargo test --bin er-save-editor build::preset`
Expected: FAIL to compile, `cannot find type BuildPreset`.

- [ ] **Step 3: Write the implementation**

In `src/build/mod.rs`:

```rust
pub mod preset;
```

In `src/main.rs`, beside the other `mod` lines:

```rust
mod build;
```

At the top of `src/build/preset.rs`:

```rust
use serde::{Deserialize, Serialize};

// Bumped only when the shape changes in a way an older reader cannot
// handle. A reader that meets a newer format refuses the file rather
// than applying half of it.
pub const FORMAT: u32 = 1;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PresetItem {
    pub param_id: u32,
    pub upgrade: u8,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PresetStats {
    pub arche_type: u8,
    pub vigor: u32,
    pub mind: u32,
    pub endurance: u32,
    pub strength: u32,
    pub dexterity: u32,
    pub intelligence: u32,
    pub faith: u32,
    pub arcane: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PresetEquipment {
    pub right_hand: [Option<PresetItem>; 3],
    pub left_hand: [Option<PresetItem>; 3],
    pub head: Option<PresetItem>,
    pub chest: Option<PresetItem>,
    pub arms: Option<PresetItem>,
    pub legs: Option<PresetItem>,
    pub talismans: [Option<PresetItem>; 4],
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct BuildPreset {
    pub format: u32,
    pub name: String,
    pub game_patch: String,
    pub stats: PresetStats,
    pub equipment: PresetEquipment,
}

pub fn to_json(preset: &BuildPreset) -> Result<String, String> {
    serde_json::to_string_pretty(preset).map_err(|e| e.to_string())
}

pub fn from_json(text: &str) -> Result<BuildPreset, String> {
    let preset: BuildPreset = serde_json::from_str(text).map_err(|e| e.to_string())?;
    if preset.format != FORMAT {
        return Err(format!(
            "unsupported preset format {}, this build understands {}",
            preset.format, FORMAT
        ));
    }
    Ok(preset)
}
```

- [ ] **Step 4: Run the test to verify it passes**

Run: `cargo test --bin er-save-editor build::preset`
Expected: PASS, 3 tests.

- [ ] **Step 5: Commit**

```bash
git add src/build/mod.rs src/build/preset.rs src/main.rs
git commit -m "Add the build preset format"
```

---

### Task 2: Preset storage on disk

**Files:**
- Create: `src/build/store.rs`
- Modify: `src/build/mod.rs`

**Interfaces:**
- Consumes: `build::preset::{BuildPreset, from_json, to_json}`
- Produces:
  - `pub fn builds_dir() -> Option<PathBuf>`
  - `pub fn save_to(dir: &Path, preset: &BuildPreset) -> Result<PathBuf, String>`
  - `pub fn load_from(path: &Path) -> Result<BuildPreset, String>`
  - `pub fn list_in(dir: &Path) -> Vec<(PathBuf, String)>`

- [ ] **Step 1: Write the failing test**

In `src/build/store.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::build::preset::{BuildPreset, PresetEquipment, PresetStats, FORMAT};

    fn temp_dir(tag: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("er_builds_test_{tag}"));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("create temp dir");
        dir
    }

    fn preset(name: &str) -> BuildPreset {
        BuildPreset {
            format: FORMAT,
            name: name.to_string(),
            game_patch: "1.17".to_string(),
            stats: PresetStats::default(),
            equipment: PresetEquipment::default(),
        }
    }

    #[test]
    fn saves_then_lists_then_loads() {
        let dir = temp_dir("roundtrip");
        let path = save_to(&dir, &preset("Strength Build")).expect("save");
        assert!(path.exists());

        let listed = list_in(&dir);
        assert_eq!(listed.len(), 1);
        assert_eq!(listed[0].1, "Strength Build");

        let back = load_from(&listed[0].0).expect("load");
        assert_eq!(back.name, "Strength Build");
    }

    #[test]
    fn a_name_with_path_characters_cannot_escape_the_directory() {
        let dir = temp_dir("escape");
        let path = save_to(&dir, &preset("../../evil")).expect("save");
        assert!(path.starts_with(&dir), "preset escaped its directory: {path:?}");
    }

    #[test]
    fn listing_an_absent_directory_is_empty_not_an_error() {
        assert!(list_in(Path::new("no/such/directory")).is_empty());
    }

    #[test]
    fn loading_junk_reports_an_error() {
        let dir = temp_dir("junk");
        let path = dir.join("broken.json");
        std::fs::write(&path, "not json").expect("write");
        assert!(load_from(&path).is_err());
    }
}
```

- [ ] **Step 2: Run the test to verify it fails**

Run: `cargo test --bin er-save-editor build::store`
Expected: FAIL to compile, `cannot find function save_to`.

- [ ] **Step 3: Write the implementation**

At the top of `src/build/store.rs`:

```rust
use std::path::{Path, PathBuf};

use super::preset::{from_json, to_json, BuildPreset};

// Same home as the image cache, so everything the app keeps for a user
// sits in one place.
pub fn builds_dir() -> Option<PathBuf> {
    if let Ok(local) = std::env::var("LOCALAPPDATA") {
        let dir = PathBuf::from(local).join("er-save-editor").join("builds");
        if std::fs::create_dir_all(&dir).is_ok() {
            return Some(dir);
        }
    }
    let beside = std::env::current_exe().ok()?.parent()?.join("builds");
    std::fs::create_dir_all(&beside).ok()?;
    Some(beside)
}

// A preset name is user text and ends up in a path, so it is reduced to
// characters that cannot walk out of the directory.
fn file_stem(name: &str) -> String {
    let cleaned: String = name
        .chars()
        .map(|c| if c.is_alphanumeric() || c == ' ' || c == '-' || c == '_' { c } else { '_' })
        .collect();
    let trimmed = cleaned.trim().to_string();
    if trimmed.is_empty() {
        "build".to_string()
    } else {
        trimmed
    }
}

pub fn save_to(dir: &Path, preset: &BuildPreset) -> Result<PathBuf, String> {
    let text = to_json(preset)?;
    let path = dir.join(format!("{}.json", file_stem(&preset.name)));
    std::fs::write(&path, text).map_err(|e| e.to_string())?;
    Ok(path)
}

pub fn load_from(path: &Path) -> Result<BuildPreset, String> {
    let text = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    from_json(&text)
}

pub fn list_in(dir: &Path) -> Vec<(PathBuf, String)> {
    let entries = match std::fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return Vec::new(),
    };
    let mut out: Vec<(PathBuf, String)> = entries
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .filter(|path| path.extension().and_then(|e| e.to_str()) == Some("json"))
        .filter_map(|path| load_from(&path).ok().map(|preset| (path, preset.name)))
        .collect();
    out.sort_by(|a, b| a.1.to_lowercase().cmp(&b.1.to_lowercase()));
    out
}
```

In `src/build/mod.rs`, add:

```rust
pub mod store;
```

- [ ] **Step 4: Run the test to verify it passes**

Run: `cargo test --bin er-save-editor build::store`
Expected: PASS, 4 tests.

- [ ] **Step 5: Commit**

```bash
git add src/build/store.rs src/build/mod.rs
git commit -m "Add build preset storage"
```

---

### Task 3: Capture a build from a character

**Files:**
- Create: `src/build/apply.rs`
- Modify: `src/build/mod.rs`

**Interfaces:**
- Consumes: `build::preset::*`, `vm::slot::slot_view_model::SlotViewModel`
- Produces: `pub fn capture(slot: &SlotViewModel, name: &str, game_patch: &str) -> BuildPreset`

- [ ] **Step 1: Write the failing test**

In `src/build/apply.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::save::save::save::Save;
    use crate::vm::vm::vm::ViewModel;
    use std::path::PathBuf;

    fn first_save() -> Option<Save> {
        let dir = std::fs::read_dir("saves").ok()?;
        let mut paths: Vec<PathBuf> = dir
            .filter_map(|e| e.ok().map(|e| e.path()))
            .filter(|p| matches!(p.extension().and_then(|e| e.to_str()), Some("sl2") | Some("co2")))
            .collect();
        paths.sort();
        Save::from_path(paths.first()?).ok()
    }

    #[test]
    fn capture_records_the_characters_attributes() {
        let save = match first_save() {
            Some(save) => save,
            None => {
                eprintln!("skipping: no saves in saves/");
                return;
            }
        };
        let vm = ViewModel::from_save(&save);
        let slot = &vm.slots[0];

        let preset = capture(slot, "Captured", "1.17");
        assert_eq!(preset.name, "Captured");
        assert_eq!(preset.format, crate::build::preset::FORMAT);
        assert_eq!(preset.stats.vigor, slot.stats_vm.vigor);
        assert_eq!(preset.stats.arcane, slot.stats_vm.arcane);
    }
}
```

- [ ] **Step 2: Run the test to verify it fails**

Run: `cargo test --bin er-save-editor build::apply`
Expected: FAIL to compile, `cannot find function capture`.

- [ ] **Step 3: Write the implementation**

At the top of `src/build/apply.rs`:

```rust
use crate::vm::equipment::equipment_view_model::EquipmentItemViewModel;
use crate::vm::slot::slot_view_model::SlotViewModel;

use super::preset::{BuildPreset, PresetEquipment, PresetItem, PresetStats, FORMAT};

// An empty equipment slot is u32::MAX in the save, and 0 in some paths.
// Both mean "nothing here".
fn slot_item(item: &EquipmentItemViewModel) -> Option<PresetItem> {
    if item.item_id == 0 || item.item_id == u32::MAX {
        return None;
    }
    Some(PresetItem {
        param_id: item.item_id,
        upgrade: (item.item_id % 100) as u8,
    })
}

pub fn capture(slot: &SlotViewModel, name: &str, game_patch: &str) -> BuildPreset {
    let stats = PresetStats {
        arche_type: slot.stats_vm.arche_type as u8,
        vigor: slot.stats_vm.vigor,
        mind: slot.stats_vm.mind,
        endurance: slot.stats_vm.endurance,
        strength: slot.stats_vm.strength,
        dexterity: slot.stats_vm.dexterity,
        intelligence: slot.stats_vm.intelligence,
        faith: slot.stats_vm.faith,
        arcane: slot.stats_vm.arcane,
    };

    let equip = &slot.equipment_vm;
    let equipment = PresetEquipment {
        right_hand: [
            slot_item(&equip.right_hand_armaments[0]),
            slot_item(&equip.right_hand_armaments[1]),
            slot_item(&equip.right_hand_armaments[2]),
        ],
        left_hand: [
            slot_item(&equip.left_hand_armaments[0]),
            slot_item(&equip.left_hand_armaments[1]),
            slot_item(&equip.left_hand_armaments[2]),
        ],
        head: slot_item(&equip.head),
        chest: slot_item(&equip.chest),
        arms: slot_item(&equip.arms),
        legs: slot_item(&equip.legs),
        talismans: [
            slot_item(&equip.talismans[0]),
            slot_item(&equip.talismans[1]),
            slot_item(&equip.talismans[2]),
            slot_item(&equip.talismans[3]),
        ],
    };

    BuildPreset {
        format: FORMAT,
        name: name.to_string(),
        game_patch: game_patch.to_string(),
        stats,
        equipment,
    }
}
```

In `src/build/mod.rs`, add:

```rust
pub mod apply;
```

**Note for the implementer:** `EquipmentItemViewModel`'s field for the
param id may not be called `item_id`. Open
`src/vm/equipment.rs`, read the struct, and use the real field name. Do
not invent one.

- [ ] **Step 4: Run the test to verify it passes**

Run: `cargo test --bin er-save-editor build::apply`
Expected: PASS, 1 test (or a skip message if `saves/` is empty).

- [ ] **Step 5: Commit**

```bash
git add src/build/apply.rs src/build/mod.rs
git commit -m "Capture a build preset from a character"
```

---

### Task 4: Apply a build to a character

**Files:**
- Modify: `src/build/apply.rs`

**Interfaces:**
- Consumes: `capture`, `vm::inventory::InventoryViewModel::add_to_inventory`, `util::regulation::Regulation`
- Produces:
  - `pub struct ApplyReport { pub added: Vec<u32>, pub equipped: Vec<u32>, pub skipped: Vec<u32> }`
  - `pub fn apply(slot: &mut SlotViewModel, preset: &BuildPreset) -> ApplyReport`

- [ ] **Step 1: Write the failing test**

Add to the `tests` module in `src/build/apply.rs`:

```rust
    #[test]
    fn apply_sets_the_attributes_from_the_preset() {
        let save = match first_save() {
            Some(save) => save,
            None => {
                eprintln!("skipping: no saves in saves/");
                return;
            }
        };
        let mut vm = ViewModel::from_save(&save);
        let mut preset = capture(&vm.slots[0], "Target", "1.17");
        preset.stats.vigor = 60;
        preset.stats.strength = 66;

        let report = apply(&mut vm.slots[0], &preset);
        assert_eq!(vm.slots[0].stats_vm.vigor, 60);
        assert_eq!(vm.slots[0].stats_vm.strength, 66);
        assert!(report.skipped.is_empty(), "a captured build should not skip its own items");
    }

    #[test]
    fn apply_skips_and_reports_an_id_the_regulation_does_not_have() {
        let save = match first_save() {
            Some(save) => save,
            None => {
                eprintln!("skipping: no saves in saves/");
                return;
            }
        };
        let mut vm = ViewModel::from_save(&save);
        let mut preset = capture(&vm.slots[0], "Bogus", "1.17");
        preset.equipment.head = Some(crate::build::preset::PresetItem {
            param_id: 4242424,
            upgrade: 0,
        });

        let report = apply(&mut vm.slots[0], &preset);
        assert!(report.skipped.contains(&4242424));
        // The rest still applied.
        assert_eq!(vm.slots[0].stats_vm.vigor, preset.stats.vigor);
    }
```

- [ ] **Step 2: Run the test to verify it fails**

Run: `cargo test --bin er-save-editor build::apply`
Expected: FAIL to compile, `cannot find function apply`.

- [ ] **Step 3: Write the implementation**

Add to `src/build/apply.rs`:

```rust
#[derive(Default, Clone, Debug)]
pub struct ApplyReport {
    pub added: Vec<u32>,
    pub equipped: Vec<u32>,
    pub skipped: Vec<u32>,
}

impl ApplyReport {
    pub fn summary(&self) -> String {
        format!(
            "{} item(s) added, {} equipped, {} skipped",
            self.added.len(),
            self.equipped.len(),
            self.skipped.len()
        )
    }
}

// True when the loaded save's regulation knows this id at all. A preset
// made on another patch can name something this save cannot hold.
fn known_to_regulation(param_id: u32) -> bool {
    use crate::util::regulation::Regulation;
    Regulation::equip_weapon_params_map().contains_key(&param_id)
        || Regulation::equip_protectors_param_map().contains_key(&param_id)
        || Regulation::equip_accessory_param_map().contains_key(&param_id)
        || Regulation::equip_goods_param_map().contains_key(&param_id)
}

pub fn apply(slot: &mut SlotViewModel, preset: &BuildPreset) -> ApplyReport {
    let mut report = ApplyReport::default();

    // Attributes first; they cannot fail and never depend on inventory.
    slot.stats_vm.vigor = preset.stats.vigor;
    slot.stats_vm.mind = preset.stats.mind;
    slot.stats_vm.endurance = preset.stats.endurance;
    slot.stats_vm.strength = preset.stats.strength;
    slot.stats_vm.dexterity = preset.stats.dexterity;
    slot.stats_vm.intelligence = preset.stats.intelligence;
    slot.stats_vm.faith = preset.stats.faith;
    slot.stats_vm.arcane = preset.stats.arcane;

    let wanted: Vec<&PresetItem> = preset
        .equipment
        .right_hand
        .iter()
        .chain(preset.equipment.left_hand.iter())
        .chain(std::iter::once(&preset.equipment.head))
        .chain(std::iter::once(&preset.equipment.chest))
        .chain(std::iter::once(&preset.equipment.arms))
        .chain(std::iter::once(&preset.equipment.legs))
        .chain(preset.equipment.talismans.iter())
        .filter_map(|item| item.as_ref())
        .collect();

    for item in wanted {
        if !known_to_regulation(item.param_id) {
            report.skipped.push(item.param_id);
            continue;
        }
        report.equipped.push(item.param_id);
    }

    report
}
```

**Note for the implementer:** this leaves two things for you to finish
against the real view model, and neither may be guessed:

1. Adding a missing item to the inventory. `InventoryViewModel::add_to_inventory`
   takes a `&RegulationItemViewModel`, not a param id. Read
   `src/vm/inventory/add_single.rs` and `src/vm/regulation.rs` to find how
   the app builds one from a param id, and use that same path. Record each
   id you add in `report.added`.
2. Writing the equipment slots. Read `EquipmentViewModel` and set the
   same fields `update_equipment` reads, and set `equipment_vm.changed = true`
   so `update_save` picks the change up.

Both must run **after** the attributes and **in the order** add-then-equip,
because the save cannot reference an item the character does not own.

- [ ] **Step 4: Run the test to verify it passes**

Run: `cargo test --bin er-save-editor build::apply`
Expected: PASS.

- [ ] **Step 5: Verify a real round trip**

Run: `cargo test -- --test-threads=1`
Expected: PASS, including the existing round-trip tests. Then in the app: load a save, capture a build, apply it to a different character, save to a new file, reload that file, and confirm the equipment and attributes match.

- [ ] **Step 6: Commit**

```bash
git add src/build/apply.rs
git commit -m "Apply a build preset to a character"
```

---

### Task 5: The builds screen

**Files:**
- Create: `src/ui/builds.rs`
- Modify: `src/ui/mod.rs`
- Modify: `src/ui/menu.rs` (add a `Builds` route)
- Modify: `src/main.rs` (route to the new screen)

**Interfaces:**
- Consumes: `build::store::*`, `build::apply::*`
- Produces: `pub fn builds(ui: &mut Ui, vm: &mut ViewModel)`

- [ ] **Step 1: Add the route**

In `src/ui/menu.rs`, add `Builds` to the `Route` enum and a button for it beside the existing ones, following the pattern already used for `Regions`.

In `src/main.rs`, add to the route match:

```rust
                    Route::Builds => builds(ui, &mut self.vm),
```

- [ ] **Step 2: Write the screen**

In `src/ui/builds.rs`:

```rust
pub mod builds {
    use eframe::egui::{self, Ui};

    use crate::build::{apply, store};
    use crate::vm::vm::vm::ViewModel;

    #[derive(Default)]
    pub struct BuildsState {
        pub new_name: String,
        pub status: String,
    }

    pub fn builds(ui: &mut Ui, vm: &mut ViewModel, state: &mut BuildsState) {
        let dir = match store::builds_dir() {
            Some(dir) => dir,
            None => {
                ui.label("Could not find a writable folder for builds.");
                return;
            }
        };

        ui.heading("Builds");
        ui.add_space(8.0);

        ui.horizontal(|ui| {
            ui.label("Name:");
            ui.add(egui::TextEdit::singleline(&mut state.new_name).desired_width(200.));
            if ui.button("Save current character as a build").clicked() {
                let preset = apply::capture(&vm.slots[vm.index], &state.new_name, "1.17");
                state.status = match store::save_to(&dir, &preset) {
                    Ok(path) => format!("Saved to {}", path.display()),
                    Err(err) => format!("Could not save: {err}"),
                };
            }
        });

        ui.add_space(12.0);
        ui.separator();

        for (path, name) in store::list_in(&dir) {
            ui.horizontal(|ui| {
                ui.label(&name);
                if ui.button("Apply to this character").clicked() {
                    state.status = match store::load_from(&path) {
                        Ok(preset) => {
                            let report = apply::apply(&mut vm.slots[vm.index], &preset);
                            // Never silent after a destructive edit.
                            report.summary()
                        }
                        Err(err) => format!("Could not read that build: {err}"),
                    };
                }
            });
        }

        if !state.status.is_empty() {
            ui.add_space(12.0);
            ui.label(&state.status);
        }
    }
}
```

`BuildsState` lives on `App` beside `textures`, and is passed in from the route match.

- [ ] **Step 3: Build and exercise it**

Run: `cargo run --release`
Save a build, apply it to a second character, read the summary line, then save the file and reload it. Expected: the summary names what happened, and the applied character keeps its own name.

- [ ] **Step 4: Verify nothing regressed**

Run: `cargo test -- --test-threads=1`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add src/ui/builds.rs src/ui/mod.rs src/ui/menu.rs src/main.rs
git commit -m "Add the builds screen"
```

---

### Task 6: Export and import a build as text

**Files:**
- Modify: `src/ui/builds.rs`

**Interfaces:**
- Consumes: `build::preset::{from_json, to_json}`
- Produces: nothing

- [ ] **Step 1: Add copy and paste**

Sharing is a string on the clipboard, not a service. Add to the screen:

```rust
        ui.add_space(12.0);
        ui.horizontal(|ui| {
            if ui.button("Copy current character as text").clicked() {
                let preset = apply::capture(&vm.slots[vm.index], &state.new_name, "1.17");
                state.status = match crate::build::preset::to_json(&preset) {
                    Ok(text) => {
                        ui.output_mut(|o| o.copied_text = text);
                        "Build copied to the clipboard".to_string()
                    }
                    Err(err) => format!("Could not copy: {err}"),
                };
            }
        });

        ui.horizontal(|ui| {
            ui.label("Paste a build:");
            ui.add(egui::TextEdit::multiline(&mut state.import_text).desired_rows(2));
            if ui.button("Apply pasted build").clicked() {
                state.status = match crate::build::preset::from_json(&state.import_text) {
                    Ok(preset) => apply::apply(&mut vm.slots[vm.index], &preset).summary(),
                    Err(err) => format!("That is not a build this version reads: {err}"),
                };
            }
        });
```

Add `pub import_text: String` to `BuildsState`.

- [ ] **Step 2: Build and exercise it**

Run: `cargo run --release`
Copy a build, clear the field, paste it back, apply. Expected: the same summary as applying from a file. Paste nonsense. Expected: a refusal message and no change.

- [ ] **Step 3: Verify nothing regressed**

Run: `cargo test -- --test-threads=1`
Expected: PASS.

- [ ] **Step 4: Commit**

```bash
git add src/ui/builds.rs
git commit -m "Share builds as text through the clipboard"
```

---

## Done when

- A character's attributes and equipment save to a file and apply to another character.
- Applying reports what was added, equipped and skipped, and never renames the character.
- A preset naming an item the regulation lacks skips it and applies the rest.
- A preset from an unknown format is refused with a message and changes nothing.
- Applying then saving produces a file the game loads.
- `cargo test -- --test-threads=1` passes.
