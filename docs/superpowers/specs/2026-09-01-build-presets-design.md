# Build Presets

**Status:** design written without a brainstorming dialogue, at the
user's request for plans only. The decisions below were made by me, not
agreed with the user. Anything marked **assumption** is a place where I
picked a default and the user may want a different one.

Second of three sub-projects. The others are item images and attributes
(planned) and the visual redesign (planned).

## Goal

Save a character's build to a file, and apply that file to any
character, so a build can be rebuilt without repeating the same clicking.

## What a preset holds

**Assumption:** a preset is a recipe, not a save. It carries what makes
a build, and nothing about where the character is in the game:

- The eight attributes and the starting class
- Each equipment slot: armament, armour piece, talisman, spell, arrow
- Per armament, its upgrade level and affinity
- Character name is **not** included; applying a preset never renames

Deliberately excluded: inventory contents beyond what the equipment
needs, event flags, regions, position, runes, playtime. A preset that
carried those would be a save file with extra steps.

## The constraint that shapes everything

An item cannot be equipped unless the character owns it. The save models
equipment as references into the character's `ga_items` table, so
`update_equipment` on a slot holding an item the character never had
produces a save the game will not read.

Applying a preset is therefore ordered:

1. Add every item the preset references to the inventory, skipping any
   the character already has.
2. Set the equipment slots to those items.
3. Set the attributes.

Step 1 already exists as `InventoryViewModel::add_to_inventory`, and
step 3 as the stats path in `update_save`. The new work is the recipe
format, the ordering, and the screen to drive it.

## Storage

**Assumption:** one JSON file per build in `%LOCALAPPDATA%\er-save-editor\builds\`,
falling back beside the executable, matching where the image cache
already goes.

JSON rather than a binary blob so a build can be read, diffed, edited by
hand and posted in a forum message. A preset is small, a few hundred
bytes.

**Assumption:** sharing is a file, not a service. The reference
screenshots show a build code like `ERB-XXXXXX`, which implies a server
to resolve codes against. That is a different project with hosting,
moderation and abuse questions attached. Export and import of the file
itself, or of the same JSON base64-encoded onto the clipboard, covers
sharing between two people with none of that.

## Versioning

Every preset records a `format` integer and the game patch it was made
on. A loader that meets a `format` it does not know refuses the file
with a message rather than guessing, because a half-applied build is
worse than a refused one.

Param ids are stable across patches, so a preset made on 1.16 applies on
1.17. An id that no longer exists in the loaded save's regulation is
reported and skipped, not silently dropped.

## Failure

Applying is the only destructive operation here, and it is not atomic:
the inventory changes before the equipment does. The design keeps it
recoverable rather than transactional.

- Applying only ever mutates the view model. Nothing reaches the save
  file until the user saves, which is the existing flow.
- An item id absent from the regulation is skipped, counted, and named
  in a summary shown after applying.
- A preset that fails to parse changes nothing at all.

The summary after applying says what was added, what was equipped and
what was skipped. Silence after a destructive edit is the thing to
avoid.

## Modules

| Module | Responsibility | Depends on |
|---|---|---|
| `build/preset.rs` | The `BuildPreset` type, serialise and deserialise | serde |
| `build/store.rs` | Listing, reading and writing preset files on disk | std::fs |
| `build/apply.rs` | Capture from a slot, apply to a slot, produce a report | vm |
| `ui/builds.rs` | The screen: list, save current, apply, export, import | ui |

`apply.rs` works on `SlotViewModel` only. It never touches `SaveType`,
so the existing `update_save` remains the single path from view model to
save file, and applying a preset is testable without a save.

## Testing

- A preset round trips through JSON unchanged.
- An unknown `format` is refused and changes nothing.
- Applying to a slot sets the attributes and the equipment slots the
  preset names.
- Applying a preset naming an item the character lacks adds it to the
  inventory first.
- Applying a preset naming an id absent from the regulation skips it,
  reports it, and still applies the rest.
- Capture then apply onto an empty slot reproduces the captured build.

Tests use a real save when one is in `saves/`, and skip otherwise, the
way the existing suite does.

## Out of scope

Images, the visual redesign, and any server-hosted build sharing.
