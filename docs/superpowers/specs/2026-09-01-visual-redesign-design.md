# Visual Redesign

**Status:** design written without a brainstorming dialogue, at the
user's request for plans only. The central decision below was made by
me. If the user disagrees with it, the plan is void and needs rewriting,
not patching.

Third of three sub-projects. The others are item images and attributes
and build presets.

## Goal

Move the editor from stock egui widgets to the look of the reference
screenshots: a dark ornate frame, gold on near-black, serif display
type, and a numbered wizard down the left instead of a flat menu.

## The decision this rests on

The reference screenshots are of **Elden Build**, a different
application on a different stack. What they show is achievable to
different degrees:

| Wanted | In egui |
|---|---|
| Dark palette, gold accents | Yes, `Visuals` covers it |
| Serif display type | Yes, with a font file shipped |
| Framed cards with rules and headers | Yes, `Frame` plus custom painting |
| Background artwork behind panels | Yes, painted into the panel background |
| Numbered wizard navigation | Yes, it is a state machine and a side panel |
| Pixel-exact reproduction of those screenshots | No |

**Decision: stay in egui.** Rewriting the UI in a web stack would mean
either embedding a browser or splitting the app into a Rust core plus a
front end, and either way every screen already written gets rebuilt. The
palette, the type and the wizard carry most of the visual difference,
and they are all reachable from where the app already is.

**If the user wants pixel fidelity to those screenshots, this plan is
the wrong plan** and the work is a rewrite, not a restyle. That is the
one question to settle before any task here is started.

## What cannot be copied

The screenshots use Elden Ring artwork as a background. That art is
FromSoftware's and cannot ship in this repository. The background layer
is built to take an image, and ships with either a plain painted
gradient or an image the user supplies locally. This is the same rule
already applied to extracted item icons.

Fonts must be shipped, so they must be licensed for it. Open Font
License faces such as Cinzel for display and EB Garamond for body text
are the intended choice; neither is FromSoftware's and both may be
redistributed.

## Approach

Three layers, each usable on its own, in this order:

1. **Theme.** One module owning the palette, spacing, rounding and text
   styles. Every screen reads from it; no screen hardcodes a colour.
   Applying it alone already changes the whole app.
2. **Components.** A small set of reusable pieces the screens compose:
   a framed card, a section header with rules either side, a primary
   button, a slot tile. Written once, so the screens stop repeating
   layout code.
3. **Navigation.** The flat route list becomes a numbered wizard with a
   persistent footer carrying Back and Continue, and the existing
   screens hang off it unchanged.

Layer 1 lands visible improvement with almost no risk. Layer 3 is the
one that touches control flow, so it comes last.

## What stays

Every existing screen keeps its behaviour. This sub-project changes how
things look and how they are reached, not what they do. The save format
work, the media work and the presets are untouched.

## Testing

A visual redesign is not unit-testable in any honest sense, and
pretending otherwise with assertions on colour values would be theatre.
What is worth testing:

- The theme module returns a complete palette; no screen may reach for
  a raw `Color32` outside it, checked by a grep in review.
- The wizard's step order and its guards: a step that requires a loaded
  save refuses to advance without one.
- Every existing test keeps passing, which is what proves the restyle
  did not disturb behaviour.

The rest is checked by running the app and looking at it, and that is
stated plainly rather than dressed up.

## Out of scope

Images and attributes, build presets, and any change to save parsing or
writing.
