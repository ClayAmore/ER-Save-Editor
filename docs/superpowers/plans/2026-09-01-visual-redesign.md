# Visual Redesign Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Give the editor the dark, gold-accented, serif look of the reference screenshots, and replace the flat menu with a numbered wizard.

**Architecture:** Three layers applied in order — a theme module that owns every colour and text style, a small set of reusable components the screens compose, and a wizard that replaces the route list. Existing screens keep their behaviour throughout.

**Tech Stack:** Rust, eframe/egui 0.26, rust-embed for the font files. No new dependencies.

**Spec:** `docs/superpowers/specs/2026-09-01-visual-redesign-design.md`

## Before starting

**This plan assumes the app stays in egui.** The spec explains why. If pixel fidelity to the reference screenshots is required instead, stop: that is a rewrite on a different stack, and none of the tasks below apply.

## Global Constraints

- No new crates.
- No screen may construct a raw `Color32`. Every colour comes from `ui::theme`.
- No FromSoftware artwork is committed. Backgrounds ship as painted gradients or load from a user-supplied local file.
- Fonts must be Open Font License and committed with their licence file.
- Behaviour does not change. Every existing test keeps passing after every task.

---

### Task 1: The theme module

**Files:**
- Create: `src/ui/theme.rs`
- Modify: `src/ui/mod.rs`
- Modify: `src/main.rs` (apply the theme where visuals are set today)

**Interfaces:**
- Consumes: nothing
- Produces:
  - `pub struct Palette { pub ink: Color32, pub parchment: Color32, pub gold: Color32, pub gold_dim: Color32, pub panel: Color32, pub panel_edge: Color32, pub danger: Color32, pub muted: Color32 }`
  - `pub fn palette() -> Palette`
  - `pub fn apply(ctx: &egui::Context)`

- [ ] **Step 1: Write the failing test**

In `src/ui/theme.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_palette_is_dark_with_a_light_accent() {
        let p = palette();
        // Guards against someone "fixing" the theme into stock egui grey.
        let ink_sum = p.ink.r() as u32 + p.ink.g() as u32 + p.ink.b() as u32;
        let gold_sum = p.gold.r() as u32 + p.gold.g() as u32 + p.gold.b() as u32;
        assert!(ink_sum < 150, "the background should be near black");
        assert!(gold_sum > 400, "the accent should be bright");
        assert!(p.gold.r() > p.gold.b(), "the accent should be warm");
    }

    #[test]
    fn every_role_is_distinct() {
        let p = palette();
        assert_ne!(p.panel, p.ink);
        assert_ne!(p.gold, p.gold_dim);
        assert_ne!(p.muted, p.parchment);
    }
}
```

- [ ] **Step 2: Run the test to verify it fails**

Run: `cargo test --bin er-save-editor ui::theme`
Expected: FAIL to compile, `cannot find function palette`.

- [ ] **Step 3: Write the implementation**

In `src/ui/theme.rs`:

```rust
use eframe::egui::{self, Color32, Rounding, Stroke};

// One place owns colour. Screens read from here so a palette change is
// a one-file change rather than a search across the UI.
pub struct Palette {
    pub ink: Color32,
    pub parchment: Color32,
    pub gold: Color32,
    pub gold_dim: Color32,
    pub panel: Color32,
    pub panel_edge: Color32,
    pub danger: Color32,
    pub muted: Color32,
}

pub fn palette() -> Palette {
    Palette {
        ink: Color32::from_rgb(12, 10, 8),
        parchment: Color32::from_rgb(232, 224, 206),
        gold: Color32::from_rgb(206, 172, 106),
        gold_dim: Color32::from_rgb(138, 114, 70),
        panel: Color32::from_rgb(24, 20, 16),
        panel_edge: Color32::from_rgb(58, 48, 34),
        danger: Color32::from_rgb(158, 62, 52),
        muted: Color32::from_rgb(140, 130, 114),
    }
}

pub fn apply(ctx: &egui::Context) {
    let p = palette();
    let mut visuals = egui::Visuals::dark();

    visuals.override_text_color = Some(p.parchment);
    visuals.panel_fill = p.ink;
    visuals.window_fill = p.panel;
    visuals.extreme_bg_color = p.ink;
    visuals.window_stroke = Stroke::new(1.0, p.panel_edge);
    visuals.window_rounding = Rounding::same(3.0);
    visuals.window_highlight_topmost = false;

    visuals.widgets.noninteractive.bg_fill = p.panel;
    visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, p.parchment);
    visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, p.panel_edge);

    visuals.widgets.inactive.bg_fill = p.panel;
    visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, p.parchment);
    visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, p.panel_edge);

    visuals.widgets.hovered.bg_fill = p.gold_dim;
    visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, p.ink);
    visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, p.gold);

    visuals.widgets.active.bg_fill = p.gold;
    visuals.widgets.active.fg_stroke = Stroke::new(1.0, p.ink);

    visuals.selection.bg_fill = p.gold_dim;
    visuals.selection.stroke = Stroke::new(1.0, p.gold);

    ctx.set_visuals(visuals);
}
```

In `src/ui/mod.rs`, add:

```rust
pub mod theme;
```

In `src/main.rs`, replace the visuals block inside `run_native`'s creation closure. It currently reads:

```rust
        let mut visuals = creation_context.egui_ctx.style().visuals.clone();
        let rounding = 3.;
        visuals.window_rounding = Rounding::default().at_least(rounding);
        visuals.window_highlight_topmost = false;
        creation_context.egui_ctx.set_visuals(visuals);
```

with:

```rust
        ui::theme::apply(&creation_context.egui_ctx);
```

- [ ] **Step 4: Run the test to verify it passes**

Run: `cargo test --bin er-save-editor ui::theme`
Expected: PASS, 2 tests.

- [ ] **Step 5: Look at it**

Run: `cargo run --release`
Expected: the whole app is now near-black with warm accents. Nothing has moved; only colour changed.

- [ ] **Step 6: Verify nothing regressed**

Run: `cargo test -- --test-threads=1`
Expected: PASS.

- [ ] **Step 7: Commit**

```bash
git add src/ui/theme.rs src/ui/mod.rs src/main.rs
git commit -m "Add a theme module and apply a dark gold palette"
```

---

### Task 2: Display typography

**Files:**
- Create: `fonts/` with `Cinzel-Regular.ttf`, `EBGaramond-Regular.ttf` and their `OFL.txt`
- Modify: `src/ui/theme.rs`
- Modify: `src/main.rs`

**Interfaces:**
- Consumes: `theme::apply`
- Produces: `pub fn install_fonts(ctx: &egui::Context)`, and the named styles `TextStyle::Name("display")` and `TextStyle::Name("section")`

- [ ] **Step 1: Fetch the fonts and their licence**

Download Cinzel and EB Garamond from Google Fonts into `fonts/`, and put each family's `OFL.txt` beside it. Both are Open Font License, which permits redistribution; that is why they were chosen over anything used by the game.

Verify before continuing:

```bash
ls fonts/
```
Expected: both `.ttf` files and at least one `OFL.txt`.

- [ ] **Step 2: Write the implementation**

Add to `src/ui/theme.rs`:

```rust
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "fonts/"]
#[include = "*.ttf"]
struct Fonts;

// Display type carries most of the difference between stock egui and
// the look being aimed at, so it is installed before anything draws.
pub fn install_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
    egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Fill);

    for (key, file) in [("display", "Cinzel-Regular.ttf"), ("body", "EBGaramond-Regular.ttf")] {
        if let Some(data) = Fonts::get(file) {
            fonts.font_data.insert(
                key.to_owned(),
                egui::FontData::from_owned(data.data.to_vec()),
            );
            if let Some(family) = fonts.families.get_mut(&egui::FontFamily::Proportional) {
                if key == "body" {
                    family.insert(0, key.to_owned());
                }
            }
            fonts
                .families
                .entry(egui::FontFamily::Name(key.into()))
                .or_default()
                .insert(0, key.to_owned());
        }
    }

    ctx.set_fonts(fonts);

    let mut style = (*ctx.style()).clone();
    style.text_styles.insert(
        egui::TextStyle::Name("display".into()),
        egui::FontId::new(28.0, egui::FontFamily::Name("display".into())),
    );
    style.text_styles.insert(
        egui::TextStyle::Name("section".into()),
        egui::FontId::new(15.0, egui::FontFamily::Name("display".into())),
    );
    ctx.set_style(style);
}
```

In `src/main.rs`, replace the existing font block inside the creation closure:

```rust
        let mut fonts = egui::FontDefinitions::default();
        egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
        egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Fill);
        creation_context.egui_ctx.set_fonts(fonts);
```

with:

```rust
        ui::theme::install_fonts(&creation_context.egui_ctx);
```

Keep `ui::theme::apply` after it.

**Note for the implementer:** `install_fonts` must still register the phosphor icon fonts, because the toolbar buttons use those glyphs. Dropping them makes the icons render as blank boxes.

- [ ] **Step 3: Look at it**

Run: `cargo run --release`
Expected: body text is a serif face, the toolbar icons still render, nothing is a blank box.

- [ ] **Step 4: Verify nothing regressed**

Run: `cargo test -- --test-threads=1`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add fonts/ src/ui/theme.rs src/main.rs
git commit -m "Ship OFL display and body fonts"
```

---

### Task 3: Reusable components

**Files:**
- Create: `src/ui/custom/card.rs`
- Modify: `src/ui/custom/mod.rs`

**Interfaces:**
- Consumes: `ui::theme::palette`
- Produces:
  - `pub fn card<R>(ui: &mut Ui, add: impl FnOnce(&mut Ui) -> R) -> R`
  - `pub fn section_header(ui: &mut Ui, text: &str)`
  - `pub fn display_title(ui: &mut Ui, text: &str)`
  - `pub fn primary_button(ui: &mut Ui, text: &str) -> egui::Response`

- [ ] **Step 1: Write the implementation**

In `src/ui/custom/card.rs`:

```rust
use eframe::egui::{self, Rounding, Stroke, Ui};

use crate::ui::theme::palette;

// The framed panel every screen sits inside. One definition, so the
// screens stop each inventing their own spacing.
pub fn card<R>(ui: &mut Ui, add: impl FnOnce(&mut Ui) -> R) -> R {
    let p = palette();
    egui::Frame::none()
        .fill(p.panel)
        .stroke(Stroke::new(1.0, p.panel_edge))
        .rounding(Rounding::same(4.0))
        .inner_margin(egui::Margin::symmetric(16.0, 14.0))
        .show(ui, add)
        .inner
}

// A heading with a rule running out to either side, as in the reference.
pub fn section_header(ui: &mut Ui, text: &str) {
    let p = palette();
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new(text.to_uppercase())
                .text_style(egui::TextStyle::Name("section".into()))
                .color(p.gold),
        );
        let available = ui.available_width();
        if available > 8.0 {
            let (rect, _) = ui.allocate_exact_size(
                egui::vec2(available, 1.0),
                egui::Sense::hover(),
            );
            ui.painter().hline(
                rect.x_range(),
                rect.center().y,
                Stroke::new(1.0, p.panel_edge),
            );
        }
    });
    ui.add_space(8.0);
}

pub fn display_title(ui: &mut Ui, text: &str) {
    let p = palette();
    ui.vertical_centered(|ui| {
        ui.label(
            egui::RichText::new(text.to_uppercase())
                .text_style(egui::TextStyle::Name("display".into()))
                .color(p.parchment),
        );
    });
    ui.add_space(6.0);
}

pub fn primary_button(ui: &mut Ui, text: &str) -> egui::Response {
    let p = palette();
    ui.add(
        egui::Button::new(egui::RichText::new(text).color(p.ink).strong())
            .fill(p.gold)
            .rounding(Rounding::same(3.0))
            .min_size(egui::vec2(160.0, 32.0)),
    )
}
```

In `src/ui/custom/mod.rs`, add:

```rust
pub mod card;
```

- [ ] **Step 2: Use them on one screen**

Convert `src/ui/general.rs` to use `display_title`, `section_header` and `card`, leaving its behaviour untouched. This is the smallest screen and proves the components before the rest depend on them.

- [ ] **Step 3: Look at it**

Run: `cargo run --release`
Open the General tab. Expected: a centred display title, a gold section rule, and the fields inside a framed card. The name field and the gender and Torrent radios still work.

- [ ] **Step 4: Verify nothing regressed**

Run: `cargo test -- --test-threads=1`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add src/ui/custom/card.rs src/ui/custom/mod.rs src/ui/general.rs
git commit -m "Add card, section header and title components"
```

---

### Task 4: Convert the remaining screens

**Files:**
- Modify: `src/ui/stats.rs`, `src/ui/equipment.rs`, `src/ui/inventory/browse.rs`, `src/ui/inventory/add.rs`, `src/ui/events.rs`, `src/ui/regions.rs`, `src/ui/none.rs`

**Interfaces:**
- Consumes: `ui::custom::card::*`
- Produces: nothing

- [ ] **Step 1: Convert one screen at a time**

For each file, in this order — `none.rs`, `stats.rs`, `regions.rs`, `events.rs`, `equipment.rs`, `browse.rs`, `add.rs` — replace its bare `ui.heading(...)` with `display_title`, group its content in `card`, and label each group with `section_header`. Change no behaviour and no widget wiring.

After each file: `cargo run --release`, look at that screen, then move on. Converting all seven before looking at any of them makes a mistake hard to place.

- [ ] **Step 2: Check for stray colours**

Run: `grep -rn "Color32::" src/ui/ | grep -v "src/ui/theme.rs"`
Expected: no results outside the theme module. Anything found is moved into the palette and referenced from there.

- [ ] **Step 3: Verify nothing regressed**

Run: `cargo test -- --test-threads=1`
Expected: PASS.

- [ ] **Step 4: Commit**

```bash
git add src/ui/
git commit -m "Restyle the remaining screens onto the shared components"
```

---

### Task 5: The wizard

**Files:**
- Modify: `src/ui/menu.rs`
- Modify: `src/main.rs`

**Interfaces:**
- Consumes: `ui::theme::palette`, `ui::custom::card::primary_button`
- Produces:
  - `pub enum Step { Champion, Arsenal, Attributes, Forge }`
  - `pub fn steps() -> [Step; 4]`
  - `pub fn can_advance(step: Step, save_loaded: bool) -> bool`

- [ ] **Step 1: Write the failing test**

In `src/ui/menu.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_steps_are_in_order() {
        let s = steps();
        assert_eq!(s.len(), 4);
        assert_eq!(s[0], Step::Champion);
        assert_eq!(s[3], Step::Forge);
    }

    #[test]
    fn no_step_past_the_first_is_reachable_without_a_save() {
        assert!(can_advance(Step::Champion, false) == false);
        assert!(can_advance(Step::Champion, true));
        assert!(can_advance(Step::Arsenal, false) == false);
    }
}
```

- [ ] **Step 2: Run the test to verify it fails**

Run: `cargo test --bin er-save-editor ui::menu`
Expected: FAIL to compile, `cannot find type Step`.

- [ ] **Step 3: Write the implementation**

Add to `src/ui/menu.rs`:

```rust
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Step {
    Champion,
    Arsenal,
    Attributes,
    Forge,
}

impl Step {
    pub fn numeral(&self) -> &'static str {
        match self {
            Step::Champion => "I",
            Step::Arsenal => "II",
            Step::Attributes => "III",
            Step::Forge => "IV",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Step::Champion => "Champion",
            Step::Arsenal => "Arsenal",
            Step::Attributes => "Attributes",
            Step::Forge => "The Forge",
        }
    }
}

pub fn steps() -> [Step; 4] {
    [Step::Champion, Step::Arsenal, Step::Attributes, Step::Forge]
}

// Every step past choosing a character needs a save behind it, so the
// footer refuses rather than opening a screen with nothing in it.
pub fn can_advance(_step: Step, save_loaded: bool) -> bool {
    save_loaded
}
```

- [ ] **Step 4: Run the test to verify it passes**

Run: `cargo test --bin er-save-editor ui::menu`
Expected: PASS, 2 tests.

- [ ] **Step 5: Draw the wizard rail and footer**

In `src/main.rs`, replace the two left side panels with one panel that lists the four steps, each as its numeral in a circle beside its label, highlighting the current one. Add a bottom panel carrying Back and Continue, with Continue disabled when `can_advance` is false.

Map each step onto the screens that already exist: Champion to the character list, Arsenal to equipment and inventory, Attributes to stats and general, Forge to events and regions. No screen is rewritten; only what reaches it changes.

- [ ] **Step 6: Look at it**

Run: `cargo run --release`
Expected: the numbered rail down the left, Back and Continue in the footer, Continue refusing to move with no save loaded, and every screen still reachable.

- [ ] **Step 7: Verify nothing regressed**

Run: `cargo test -- --test-threads=1`
Expected: PASS.

- [ ] **Step 8: Commit**

```bash
git add src/ui/menu.rs src/main.rs
git commit -m "Replace the flat menu with a numbered wizard"
```

---

### Task 6: The background layer

**Files:**
- Create: `src/ui/backdrop.rs`
- Modify: `src/ui/mod.rs`
- Modify: `src/main.rs`

**Interfaces:**
- Consumes: `ui::theme::palette`
- Produces: `pub fn paint(ctx: &egui::Context)`

- [ ] **Step 1: Write the implementation**

In `src/ui/backdrop.rs`:

```rust
use eframe::egui::{self, Color32};

use crate::ui::theme::palette;

// A painted gradient, not artwork. Game art is FromSoftware's and does
// not ship here; a user who wants it supplies their own file locally.
pub fn paint(ctx: &egui::Context) {
    let p = palette();
    let screen = ctx.screen_rect();
    let painter = ctx.layer_painter(egui::LayerId::background());

    painter.rect_filled(screen, 0.0, p.ink);

    // A soft warm pool behind the centre, so the panels sit on something.
    let steps = 24;
    for i in 0..steps {
        let t = i as f32 / steps as f32;
        let radius = screen.height() * (0.15 + t * 0.75);
        let alpha = ((1.0 - t) * 18.0) as u8;
        painter.circle_filled(
            screen.center(),
            radius,
            Color32::from_rgba_unmultiplied(p.gold_dim.r(), p.gold_dim.g(), p.gold_dim.b(), alpha),
        );
    }
}
```

In `src/ui/mod.rs`, add:

```rust
pub mod backdrop;
```

In `src/main.rs`, call it first in `update`, before any panel:

```rust
        ui::backdrop::paint(ctx);
```

Panels must be transparent for it to show. In `theme::apply`, set:

```rust
    visuals.panel_fill = Color32::TRANSPARENT;
```

and rely on `card` for panel fills.

- [ ] **Step 2: Look at it**

Run: `cargo run --release`
Expected: a warm pool behind the cards rather than flat black, with text still legible everywhere. If any screen becomes hard to read, its content needs wrapping in `card`, not a change to the backdrop.

- [ ] **Step 3: Verify nothing regressed**

Run: `cargo test -- --test-threads=1`
Expected: PASS.

- [ ] **Step 4: Commit**

```bash
git add src/ui/backdrop.rs src/ui/mod.rs src/ui/theme.rs src/main.rs
git commit -m "Paint a backdrop behind the panels"
```

---

## Done when

- Every screen draws in the dark gold palette with serif display type.
- No `Color32::` appears in `src/ui/` outside `theme.rs`.
- The four numbered steps replace the flat menu, and Continue refuses to advance with no save loaded.
- The fonts in `fonts/` are Open Font License and their licence files are committed.
- No game artwork is committed.
- `cargo test -- --test-threads=1` passes.
