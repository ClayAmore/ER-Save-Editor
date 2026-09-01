# Item Images and Attributes

**Status:** design approved, implementation starting on branch
`feature/item-images-and-attributes`.

First of three sub-projects split out of a larger request. The other two
are build presets and a visual redesign; each gets its own design, plan
and implementation.

## Goal

Show an image and the item's attributes for weapons, armour, talismans,
ashes of war and goods, in the inventory lists, the add-items browser
and the equipment slots.

## Decisions

| Question | Answer |
|---|---|
| Which sub-project first | Images and attributes |
| Where images appear | Inventory lists, add-items browser, equipment slots |
| Scope | Attributes as well as images |
| Attribute source | Numbers from the regulation, image and description from the API |
| Image delivery | Fetched when first shown, cached on disk |
| Index construction | Generated once offline, embedded in the binary |
| DLC images | Disk cache keyed by param id, so locally extracted icons are picked up with no code change |

## What the API gives, and what it does not

`eldenring.fanapis.com` returns an image URL, a description and scraped
stats per item, keyed by its own opaque id such as
`17f69c35d2cl0i1oh7zuqfb3mdvsj`. There is no FromSoftware param id, so
the only join back to our data is the item name.

Endpoint totals, measured: weapons 307, shields 69, ashes 90, talismans
87, armors 568, items 462, sorceries 71, incantations 98, ammos 53,
spirits 64 — 1869 entries, roughly 18 MB of 200x200 PNGs at about 10 KB
each.

Measured coverage gaps, by fetching every page of an endpoint and
searching the names:

- No Shadow of the Erdtree content. Milady, Backhand Blade, Bloodfiend,
  Death Knight, Smithscript and Dryleaf are all absent from `weapons`;
  Death Knight, Oathseeker, Verdigris, Leda, Ansbach and Rellana are all
  absent from `armors`.
- Names do not match ours exactly even in the base game. The API writes
  `Shard Of Alexander` where we write `Shard of Alexander`, and
  `Crimson Seed Talisman` where we write `Crimson Seed`. `Two-Headed
  Turtle` finds nothing at all.

These are measurements, not assumptions, but the code must not encode
them. Generation attempts a match for every param id we know and falls
back only on a real miss, so the day the API adds DLC content a
regeneration picks it up with no code change.

## The regulation stays the source of numbers

Each save carries its own regulation, holding attack values,
requirements, scaling and weight for the player's exact game version,
DLC included. The API's equivalents are scraped from wikis and frozen at
the 2022 release. Numbers come from the regulation; the API supplies
only what the regulation lacks, which is the image and the description.

## Generating the index

A test marked `#[ignore]`, run by hand:

```
cargo test -- --ignored regenerate_item_media
```

It needs no save file: the name tables in `src/db/*_name.rs` are static
and already map param ids to names.

Steps:

1. Fetch all pages of the ten endpoints.
2. Normalise both sides of the join: lowercase, strip punctuation,
   collapse whitespace, drop a trailing `+N` upgrade suffix, and drop a
   trailing category word such as `talisman` when it is absent on our
   side.
3. For every param id in the five name tables, look for a match.
4. Write `assets/item_media.json` mapping param id to image URL and
   description.
5. Print a coverage report per category and list every unmatched name.

Unmatched entries are corrected by hand in the generated file. Placing
the fragile name join in an offline step, run once, with a report I can
read, is the reason this approach was chosen over matching at runtime.

Endpoints map onto our params as:

| Param | Endpoints |
|---|---|
| `EquipParamWeapon` | `weapons`, `shields`, `ammos` |
| `EquipParamGoods` | `items`, `sorceries`, `incantations`, `spirits` |
| `EquipParamProtector` | `armors` |
| `EquipParamAccessory` | `talismans` |
| `EquipParamGem` | `ashes` |

## Modules

| Module | Responsibility | Depends on |
|---|---|---|
| `media/index.rs` | Embedded index; `lookup(param_id) -> Option<&MediaEntry>` | serde_json |
| `media/cache.rs` | Image bytes: memory, then disk, then network | reqwest |
| `vm/item_details.rs` | Joins regulation attributes with the index entry | both |
| `ui/*` | Draws only; asks for a texture and renders whatever exists | vm |

`cache.rs` returns bytes rather than textures, so it knows nothing about
egui and can be tested without a window.

Every dependency this needs is already in `Cargo.toml`: `reqwest` with
`blocking` and `json`, `serde_json`, `image` and `rust_embed`. No new
crates.

## Getting an image on screen

The UI redraws continuously, so nothing may block it. Four worker
threads take requests over a channel and return bytes the same way. The
UI polls the channel each frame and uploads textures. Four is chosen to
keep a scrolling list filling quickly without opening more sockets than
a community API should be asked to serve.

Each item holds one of four states: `NotRequested`, `Requested`, `Ready`
or `Failed`. A `Failed` is never retried, which is what stops an offline
list from hammering the network once per frame. Only rows currently
visible are requested.

The disk cache lives in `%LOCALAPPDATA%\er-save-editor\images\`, falling
back to a folder beside the executable when that variable is missing, so
the app still works from a directory it cannot write to.

Cache files are named by param id, `1000000.png`, not by API id. That
one choice is what opens the door to DLC images: a player who extracts
icons from their own installation with UXM and
[erdb](https://github.com/EldenRingDatabase/erdb) (MIT) can drop the
folder into the cache and every item gains an image, including content
the API will never carry. No second code path, no local mode, no `if` —
the cache is consulted before the network and simply finds them.

Those icons are FromSoftware's, so they stay in a player's local cache
and are never committed to this repository or shipped with a build.

Scraping a wiki was considered and rejected. Fextralife's `robots.txt`
prohibits automated retrieval of its content and specifically forbids
"creating data sets containing our content or sharing it with others",
with `Disallow: /wiki/*` covering every content page. Extracting from
files the player already owns has no such problem.

## What a DLC item looks like

Its name and all of its numbers, from the regulation. No image and no
description, with a quiet placeholder where the icon goes. This is the
normal state for a large share of a current save's inventory, not an
error, and the screen presents it as such.

## Error handling

No network, a 404, a full disk or malformed JSON all resolve to the same
thing: the item renders without an image and the app carries on. Given
this codebase's history of panics on unexpected data, the rule for new
code is that nothing on a network or disk path uses `expect` or
`unwrap`.

## Testing

- Generation reports coverage per category. The first run establishes
  what coverage is actually achievable; those numbers are then written
  into the test as a floor, so a later regeneration that silently
  matches fewer items fails instead of quietly shipping blanks. The
  floor is recorded from a measurement, never guessed up front.
- Name normalisation: `Hand Axe +5`, `hand axe` and `Hand  Axe` agree.
- The embedded index parses and known ids resolve.
- An item absent from the index degrades to name-only without panicking.
- The cache is driven by a fake fetcher, so no test touches the network.

Image fetching itself is not tested against the live API.

## Out of scope

Build presets and the visual redesign. This sub-project adds images and
attributes to the existing screens without changing their layout beyond
the room the icons need.
