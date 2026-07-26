# Design system

This is the rulebook for this fork's interface.
It exists because the interface had grown by accretion:
every new feature added its own spacing, its own button, its own casing,
and the result looked cluttered even though each piece was reasonable on its own.

Every rule below is a constraint, not a suggestion.
When a rule and a habit disagree, the rule wins.

## What was wrong

Observed on real screenshots of v0.32.0, not in theory:

| Problem | Evidence |
|---|---|
| Navigation shouted in two voices | Four tabs in `ALL CAPS`, two in `Sentence case` |
| Covers in two aspect ratios | Portrait covers looked right; wide Steam headers were squeezed into a 32 px column and became unreadable smears |
| Rows jumped in height | Rows with a cover were taller than rows without, so the list had no rhythm |
| Titles fought their own row | Titles centred while covers sat on the left, giving no stable reading edge |
| Control soup | Up to eight controls in one custom-game row |
| Ad-hoc spacing | Padding values of 2, 5, 10, 15, 20 and 35 in the same screens |
| Flat typography | Section headings the same size as the content beneath them |
| Colour as decoration | Saturated blue on every button, so nothing stood out; red used both for "delete" and for "dismiss" |
| Raw machine text as content | `C:/Users/.../Gpg4win [Dateien: 1, Größe: 38 B, geändert: 2026-06-10]` as a primary line |
| Boxes inside boxes | Up to three nested containers per section |

## Principles

1. **One accent per view.** Exactly one action may look primary. Everything else recedes.
2. **Alignment beats decoration.** A stable left edge does more for perceived quality than any colour.
3. **Rhythm over density.** Equal row heights and one spacing scale; empty space is not wasted space.
4. **Colour carries meaning only.** If a colour isn't saying "this is good/bad/active", it doesn't belong.
5. **Machine text is secondary.** Paths, IDs and byte counts never lead a line.
6. **Three controls, then a menu.** Anything beyond three per row goes into the overflow menu.

## Tokens

### Spacing
Only these values, in points: **4, 8, 16, 24, 32**.

* `4` — inside a control (icon to its label)
* `8` — between related controls in a row
* `16` — between rows, and padding inside a card
* `24` — between a section heading and its card
* `32` — between sections

Nothing else. No 2, no 5, no 15, no 35.

### Type
| Size | Use |
|---|---|
| 12 | Meta text: timestamps, sizes, counts, hints |
| 14 | Body text, labels, input contents |
| 16 | Game titles, values in a definition row |
| 20 | Section headings |
| 28 | Dashboard figures |

Two adjacent elements must never share a size unless they are the same kind of thing.

### Colour
The theme already defines `text`, `disabled`, `added`, `positive`, `negative`, and the accent.
Use them like this, and nowhere else:

* **Accent, filled** — the one primary action of the view
* **Accent, outline** — the current navigation tab
* **Neutral / no fill** — every other action, including "dismiss", "hide", "refresh"
* **`negative`** — only destructive actions that lose data, and error states
* **`added` / `positive`** — only status: healthy, new, changed

Never use a colour to make something merely noticeable.

### Casing
**Sentence case everywhere**, including navigation, buttons and badges.

An earlier version of this document carved out an exception for badges.
That was wrong: in practice a badge like `BENUTZERDEFINIERT` beside
`DUPLIKATE` shouted louder than the game's own name. The exception is gone.

### Surfaces and depth
Depth comes from **layered surfaces**, never from drop shadows or hard outlines.

* Page background is the darkest (or lightest) layer
* A card or list row sits on a slightly lifted surface, with **no border**
* Corner radius: `12` for cards and rows, `6` for badges, `10` baked into cover art
* No drop shadows anywhere. A 1 px offset shadow is the clearest sign of a dated interface.

### Colour, concretely
One accent hue for the whole application — a violet — used for the active
navigation tab and the single primary action. Blue and violet used to compete
in the same window; that is what made it look assembled rather than designed.

Status colours are muted rather than pure: green for added, a warm red for
problems. Never full-saturation red or green.

### Interactive text
A list item is text, not a control. Titles have **no fill and no border**;
they reveal a faint background on hover to show they can be clicked.
Filling every title turned the list into a stack of input fields.

## Components

### Navigation tab
Sentence case, equal width, outline when active, no fill.

### List row
* Height is **fixed at 56** regardless of content, so the list keeps its rhythm.
* Cover slot is always **40 × 56**, portrait, and always present.
  When there is no image, the slot stays empty rather than collapsing —
  alignment matters more than saving 40 points.
* Covers are stored as **2:3 portrait** images. A wide source image is
  centre-cropped, never squeezed.
* Title: size 16, **left-aligned**, truncated with an ellipsis if too long.
* Then at most three controls, then the overflow menu.

### Card
One level of nesting. A card never contains another card.
Padding `16`, and it holds either a definition list or a set of controls, not both.

### Definition row
Label right-aligned in a fixed column, value left-aligned after it,
so every value in a card starts at the same x position.
Label at size 14, value at 16.

### Section
Heading at size 20, then `24`, then exactly one card.

## Rules for text

* Never show a full path as the leading text of a row.
  Show the last part, and the whole path in a tooltip.
* Numbers get units and no more precision than a person needs:
  `38 B`, not `38 Bytes (38)`.
* No square brackets, no key/value dumps in prose.

## Checking the work

Before a UI change is done, confirm:

- [ ] Every spacing value is 4, 8, 16, 24 or 32
- [ ] Exactly one filled accent control in the view
- [ ] All rows in a list have the same height
- [ ] Labels form a single vertical edge
- [ ] Every text size is from the scale, and neighbours differ
- [ ] No card inside a card
- [ ] No colour used for emphasis alone
- [ ] Sentence case throughout
- [ ] No raw path or ID leading a line
- [ ] At most three controls before the overflow menu

## Where this doesn't reach yet

Being honest about scope: the rules are applied to the navigation,
the game list, the dashboard, the settings sections and the
custom-games rows. The modals have not been revisited, and the
expanded part of a custom-games row still stacks its own controls.
Those are the next passes, not exceptions to the rules.

## Layout width

A window wider than **1400** lays the dashboard panels and the settings
cards out in two columns. Below that they stack. Nothing else changes:
the same cards, the same order, read top to bottom in either case.
