// The fonts compared for the interface, each under the SIL Open Font License: for the titles and for
// the text. Manrope, the owner's first idea, comes first.
const FONTS = {
  manrope: { name: "Manrope", title: "Manrope", text: "Manrope" },
  inter: { name: "Inter", title: "Inter", text: "Inter" },
  instrument: { name: "Instrument Sans", title: "Instrument Sans", text: "Instrument Sans" },
  dm: { name: "DM Sans", title: "DM Sans", text: "DM Sans" },
  figtree: { name: "Figtree", title: "Figtree", text: "Figtree" },
  jakarta: { name: "Plus Jakarta Sans", title: "Plus Jakarta Sans", text: "Plus Jakarta Sans" },
  outfit: { name: "Outfit", title: "Outfit", text: "Outfit" },
  fredoka: { name: "Fredoka + Manrope", title: "Fredoka", text: "Manrope" },
  grotesk: { name: "Space Grotesk + Manrope", title: "Space Grotesk", text: "Manrope" },
  lexend: { name: "Lexend", title: "Lexend", text: "Lexend" },
  nunito: { name: "Nunito Sans", title: "Nunito Sans", text: "Nunito Sans" },
  atkinson: { name: "Atkinson Hyperlegible", title: "Atkinson Hyperlegible Next", text: "Atkinson Hyperlegible Next" },
  plex: { name: "IBM Plex Sans", title: "IBM Plex Sans", text: "IBM Plex Sans" },
  work: { name: "Work Sans", title: "Work Sans", text: "Work Sans" },
  source: { name: "Source Sans 3", title: "Source Sans 3", text: "Source Sans 3" },
};

// Single quotes: the value also goes into a style attribute.
const fontStack = family => `'${family}', sans-serif`;
