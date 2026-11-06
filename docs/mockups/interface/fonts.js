// The fonts to try, each under the SIL Open Font License, for every part of the interface.
const TEXT_FONTS = ["Manrope", "Inter", "Instrument Sans", "DM Sans", "Figtree", "Plus Jakarta Sans", "Outfit", "Fredoka", "Space Grotesk", "Lexend", "Nunito Sans", "Atkinson Hyperlegible Next", "IBM Plex Sans", "Work Sans", "Source Sans 3"];
const CODE_FONTS = ["JetBrains Mono", "Fira Code", "Source Code Pro", "IBM Plex Mono", "Geist Mono", "DM Mono", "Red Hat Mono"];

// The parts that can take their own font: [label, CSS variable, group of the top of the page].
const FONT_ROLES = {
  goalTitle: ["Title", "--font-goal-title", "Goal"],
  goalText: ["Goal text", "--font-goal-text"],
  goalLinks: ["Hint, manual", "--font-goal-links"],
  goalNew: ["New", "--font-goal-new"],
  goalGo: ["Let's go", "--font-goal-go"],
  menus: ["Menus", "--font-menus", "Screen"],
  placeChapter: ["Chapter", "--font-place-chapter"],
  placeTitle: ["Level", "--font-place-title"],
  tabs: ["Maps, Legend", "--font-tabs"],
  status: ["Status bar", "--font-status"],
};

// Single quotes: a value can also go into a style attribute.
const fontStack = (family, fallback = "sans-serif") => `'${family}', ${fallback}`;
