// The styles of the sheet: their title, their description and the shades of their paints.
// The elements of a style are the SVG files of its folder.
const THEMES = {
  "daylight": { title: "Daylight", text: "The style chosen in step 1a: a light gray floor, slate walls, deep paints. Step 1b refines it element by element, one group of tabs per open decision.", paints: { red: "#ec4a3e", yellow: "#f0b400", blue: "#2f7de1" } },
};

// The candidate shades of each paint, one group of tabs per color; the first is the shade of step 1a.
const PAINT_SHADES = {
  red: { vermilion: "#ec4a3e", scarlet: "#dc2f34", coral: "#f2694f", ruby: "#c62f45" },
  yellow: { golden: "#f0b400", sunflower: "#f7c600", amber: "#f2a200", mustard: "#d9ad00" },
  blue: { azure: "#2f7de1", cobalt: "#2459d6", sky: "#3a96ee", ultramarine: "#4053d6" },
};
