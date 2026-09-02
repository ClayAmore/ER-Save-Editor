# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

A Windows desktop GUI (eframe/egui) for editing Elden Ring save files — PC `.sl2`/`.co2` and PlayStation Save Wizard exported `.txt`. Single binary crate, `er-save-editor`. `build.rs` embeds the app icon via `winres` and only runs on Windows.

From the README: the project exists for offline build-making. Do not add anything that enables online behaviour the game itself does not allow.

## Commands

```
cargo check                 # fast compile check
cargo run                   # launch the GUI (debug keeps the console window)
cargo build --release       # release build hides the console (windows_subsystem = "windows")
cargo test                  # all tests; they live in the binary target, not tests/
cargo test steed_attire     # one module
cargo test steed_attire::reads_the_applied_attire   # one test
cargo test -- --nocapture   # save-shape tests print useful diagnostics
```

Regenerate the embedded item media index (network, offline, by hand — never in CI). It rewrites `assets/media/item_media.json` and prints a per-category match-rate report you are expected to read:

```
cargo test --bin er-save-editor -- --ignored regenerate_item_media
```

### Tests need real saves

`saves/` is gitignored and empty on a fresh clone. Every save-shape test (`save_round_trip`, `stat_edit`, `modded_saves`, `new_classes`, `dlc_names`, `steed_attire`) prints `skipping: no saves in saves/` and **passes vacuously** without them. Before trusting a green run on anything touching parsing or writing, drop an `ER0000.sl2` (and/or `ER0000.co2`) into `saves/`. The `steed_attire` tests additionally need the two reference saves `saves/attire_silver.sl2` and `saves/attire_funereal.sl2`.

## Architecture

Bytes flow one way down and back up:

```
file → Read trait → save::  (on-disk structs) → vm::  (view models) → ui::  (egui)
file ← Write trait ← save::                   ← vm::update_save     ← edits
```

- **`read::Read` / `write::Write`** (`src/read/read.rs`, `src/write/write.rs`) — two one-method traits every save structure implements. `read` takes a `BinaryReader`, `write` returns `Vec<u8>`. Symmetry between them is what makes round-tripping work.
- **`save::`** — structs that mirror the file layout exactly. `save/common/` is shared, `save/pc/` and `save/playstation/` hold the platform-specific headers and `UserData10`/`UserData11` wrappers.
- **`vm::`** — editable view models. `ViewModel::from_save` builds the whole tree; `ViewModel::update_save` writes it back. `vm/slot.rs` fans out per character into `general`/`stats`/`equipment`/`inventory`/`events`/`regions`.
- **`ui::`** — one module per route, each a plain `fn(ui, &mut ViewModel)`. `Route` in `ui/menu.rs` drives the match in `App::update`.
- **`db::`** — hand-maintained static tables (item/weapon/armor names, event-flag byte+bit offsets, grace and boss ids), mostly `Lazy<Mutex<HashMap>>`. These are the source of names the regulation does not provide.
- **`util::`** — `bnd4` (FromSoftware container), `params`/`param_structs` (the ~18k-line generated param row layouts), `regulation` (decrypt + parse), `validator` (irregular-save detection).
- **`media::`** — item icons and descriptions; see below.

### Invariants that matter more than they look

**Byte-identical round trip.** Reading a save and writing it back unedited must reproduce the file exactly (`save_round_trip`). Any new field must be written back in the same position and width it was read from. A size change is an immediate failure.

**`changed` flags gate writes.** `update_save` skips inventory and equipment entirely unless `inventory_vm.changed` / `equipment_vm.changed` is set. This is deliberate: rebuilding the gaitem map and inventory lists from the view model is lossy compared to leaving the original bytes alone. If an edit silently doesn't persist, an unset `changed` flag is the first thing to check.

**`SaveType` is the platform abstraction.** `save/save.rs` is a long list of accessors that each `match` on `SaveType::{Unknown, PC, PlayStation}` with near-duplicated arms. Adding an editable field means adding a getter/setter pair there, both arms, and `Unknown` panics by design. This duplication is the established shape — follow it rather than refactoring around it.

**Params are per-save global state.** Every save embeds its own encrypted `regulation.bin` (AES-CBC → BND4 → zstd). `Regulation::init_params(save)` must run before any param lookup, and the derived maps (`equip_weapon_params_map()` etc.) are process-wide statics, `Box::leak`ed and rebuilt when the regulation's md5 changes. The reason is a real bug class: opening save B must never answer with save A's params. When a regulation fails to parse, the maps are cleared rather than left stale — an empty lookup fails loudly, stale params do not.

**Five id spaces, not one.** Weapon 1000000 and protector 1000000 are different items. Categories are disambiguated by a shared set of offsets used consistently across the codebase — `0x0` weapon, `0x10000000` armor, `0x20000000` accessory, `0x40000000` goods, `0x80000000` ash of war — in `vm::inventory::{InventoryItemType, InventoryGaitemType}` and `media::index::MediaCategory::offset`. Never key a cross-table lookup or a cache file on a bare param id.

**Weapon ids encode affinity and upgrade.** `base + affinity*100 + level`, base weapons on multiples of 10000. `/100*100` strips the level but keeps the affinity; artwork and details belong to the base, so `media::index::key` reduces weapons with `(id / 10000) * 10000`.

**Panics are the parser's error channel.** The `save::` parsers `assert!` their way through the layout, so an unrecognised file surfaces as a panic, not an `Err`. `App::guarded` wraps every load in `catch_unwind` so a bad file reports a message instead of taking the window down. Keep new load paths going through it.

### media/

Item icons and descriptions come from a generated join, not from the game files. `assets/media/item_media.json` maps namespaced media keys to `{image_url, description}` and is the only thing in `assets/` embedded in the binary (via `rust-embed`). Icons themselves are fetched at runtime over HTTP into a disk cache (`%LOCALAPPDATA%\er-save-editor\images`, keyed by media key so users can drop in their own extracted icons), decoded on a worker thread, and turned into egui textures by `ItemTextures::poll`, called once per frame before anything asks for one. A failed fetch is recorded as `Failed` and never retried, which is what stops an offline session hammering the network once a frame; bytes that arrive but don't decode are discarded from disk so a later run can re-fetch.

The name-based join in `media/generate.rs` is fragile by nature, which is why it runs offline behind `#[ignore]` with a coverage report rather than on a user's machine.

## Conventions

- Every file wraps its contents in an inner `pub mod <same_name_as_file>`, so paths read `crate::vm::vm::vm::ViewModel`, `crate::ui::stats::stats::stats`. Odd, but universal — match it in new files.
- Comments explain *why* a non-obvious rule exists (id collisions, stale params, `changed` gating), not what the line does. Keep that ratio.
- Test names are full sentences describing the behaviour (`a_broken_regulation_never_reuses_the_previous_saves_params`).
- Tests that depend on files self-skip with an `eprintln!("skipping: ...")` rather than failing.

## Planning docs

`docs/superpowers/{specs,plans}/` holds design and implementation-plan markdown for larger features; `.superpowers/sdd/` holds the per-task briefs, reports and review diffs from executing them. Useful for the reasoning behind a feature, not as a spec of current behaviour.
