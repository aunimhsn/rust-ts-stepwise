// The styles of the sheet: their title and their description.
// The elements of a style are the SVG files of its folder.
const THEMES = {
  "daylight": { title: "Daylight", text: "The style chosen in step 1a: a light gray floor, slate walls, deep paints. Step 1b refines it element by element, one group of tabs per open decision." },
};

// The candidate shades of each paint, one group of tabs per color still open; the first is the
// default. Red and blue are decided.
const PAINT_SHADES = {
  red: { scarlet: "#dc2f34" },
  yellow: { amber: "#f2a200", bright: "#ffb300", sunflower: "#f7c600" },
  blue: { cobalt: "#2459d6" },
};
