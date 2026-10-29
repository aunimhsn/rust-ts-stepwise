# Design journal

One entry per application: the design choices made with the owner, the ones rejected, and why.

**This journal is a source of ideas, not a set of rules.** Each application has its own users, subject and constraints: its design starts from its own wireframes and explores new directions (see the mock-up phase of [process.md](process.md)). An entry of this journal may inspire one of the styles compared, never all of them, and nothing here is a default.

## rust-exile-market (2026-10)

A small desktop window, a companion to Path of Exile 2, that prices a copied item ([mock-up](https://github.com/aunimhsn/rust-exile-market/blob/main/docs/mockups/narrow-styles.html), [specs](https://github.com/aunimhsn/rust-exile-market/blob/main/specs.md)).

### Context

- Used next to a dark, busy game, often on a second screen; the player glances at it between two items.
- Many kinds of data in a narrow window (420 px): item, price in three currencies, stats, filters, offers.
- The game already has strong color codes (item rarities), which the interface had to respect.

### How it went

1. A first coded mock-up, all at once (layout, content and style together): correct, but generic and never exactly what the owner wanted.
2. Editable Excalidraw wireframes, iterated with the owner: the owner drew, Claude Code rearranged and completed. The layout became personal.
3. Three styles on one HTML page, switched with tabs: A · Forge (engraved capitals, gold, close to the game), B · Slate (geometric, modern), C · Compact (condensed, dense). The owner picked Slate.
4. About a hundred small decisions on Slate, each compared on the page with a temporary tab, decided by the owner on their own screen, then the tab removed.

### Choices kept

- **Bands, not cards**: full-width bands in two near-black grays, without borders or rounded frames ("rounded cards look old").
- **White as the main action color**; the colors only carry a meaning: item rarities, the softcore (blue) or hardcore (red) league, warnings (dark yellow bands), better or worse (green, red).
- **Flat square action blocks** glued to the edges (league, settings, search, refresh); **small information elements rounded by 3 px** (tags, inputs, checkboxes); **secondary actions as pills** with a verb ("Check on trade", "Show 6 stats", "Copy whisper").
- **Minimalism**: every redundant text, counter or icon removed; e.g. "Show the next 10" became a bare double chevron, the stats chevron became a pill.
- **Manrope** for the text, **JetBrains Mono** for the numbers that line up; a point as decimal separator in every language, no thousands separator.
- **Subtle hover motion**: an icon grows, the refresh icon turns a quarter, a chevron slides 2 px.
- A **vivid orange** (`#ff8c1a`) for the best tier, the only accent that "pops".

### Choices rejected

- A blue accent: it clashed with the blue of the magic items.
- Bronze tints and a glow on the item name: "looks old", "hurts the eyes".
- Poppins: its uneven character widths looked irregular in a narrow window.
- Badges for the tiers: too heavy; plain colored text was enough.
- Icons next to text when they never looked aligned: replaced by pills or removed.

### Lessons

- Pixel alignments (icon and text) are tuned **by eye on the owner's real screen**, with a tab of offsets (0, 0.5, 1… px); headless screenshots at another display scale only give the direction.
- Icons are aligned on the lowercase letters, not on the box of the text.
- Text symbols (⌕, ↻, ✓) render small and off-center: draw the icons as SVG lines.
- A class name shared by two components (`top`, `online`) silently broke the layout twice: prefix the state classes.
