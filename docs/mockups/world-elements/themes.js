// The styles of the sheet: their title and their description.
// The elements of a style are the SVG files of its folder.
const THEMES = {
  "daylight": { title: "Daylight", text: "The style chosen in step 1a: a light gray floor, slate walls, deep paints. Step 1b refines it element by element, one group of tabs per open decision." },
};

// The shades of the paints, decided in step 1b.
const PAINT_SHADES = { red: "#dc2f34", yellow: "#ffb300", blue: "#2459d6" };

// The item, a hexagon, and its slot (the goal "put an item here"), in the candidate colors; drawn
// here while the color is open, then written to daylight/item.svg and daylight/goal-item.svg.
const ITEM_COLORS = { teal: "#5fd0b0", ivory: "#f4efe6", violet: "#8f6ad8" };
const INK = "#222935";
const SLOT = 'fill="#c3cad4" stroke="#a9b3c0" stroke-width="1.5" stroke-linejoin="round"';
const ITEM_SHAPES = {
  hexagon: {
    item: fill => `<path d="M32 19 L43.3 25.5 L43.3 38.5 L32 45 L20.7 38.5 L20.7 25.5 Z" fill="${fill}" stroke="${INK}" stroke-width="2.5" stroke-linejoin="round"/>
      <path d="M32 26.5 L36.8 29.2 L36.8 34.8 L32 37.5 L27.2 34.8 L27.2 29.2 Z" fill="${INK}"/>`,
    slot: `<path d="M32 16 L45.9 24 L45.9 40 L32 48 L18.1 40 L18.1 24 Z" ${SLOT}/>`,
  },
};
