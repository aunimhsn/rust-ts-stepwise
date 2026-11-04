// The styles of the sheet: their title and their description.
// The elements of a style are the SVG files of its folder.
const THEMES = {
  "daylight": { title: "Daylight", text: "The style chosen in step 1a: a light gray floor, slate walls, deep paints. Step 1b refines it element by element, one group of tabs per open decision." },
};

// The shades of the paints, decided in step 1b.
const PAINT_SHADES = { red: "#dc2f34", yellow: "#ffb300", blue: "#2459d6" };

// The candidate items, one shape and one color, each shape with its slot (the goal "put an item
// here"); drawn here while open, then written to daylight/item.svg and daylight/goal-item.svg.
const ITEM_COLORS = { teal: "#5fd0b0", ivory: "#f4efe6", violet: "#8f6ad8" };
const INK = "#222935";
const SLOT = 'fill="#c3cad4" stroke="#a9b3c0" stroke-width="1.5" stroke-linejoin="round"';
const ITEM_SHAPES = {
  hexagon: {
    item: fill => `<path d="M32 19 L43.3 25.5 L43.3 38.5 L32 45 L20.7 38.5 L20.7 25.5 Z" fill="${fill}" stroke="${INK}" stroke-width="2.5" stroke-linejoin="round"/>
      <path d="M32 26.5 L36.8 29.2 L36.8 34.8 L32 37.5 L27.2 34.8 L27.2 29.2 Z" fill="${INK}"/>`,
    slot: `<path d="M32 16 L45.9 24 L45.9 40 L32 48 L18.1 40 L18.1 24 Z" ${SLOT}/>`,
  },
  cell: {
    item: fill => `<rect x="28" y="14" width="8" height="5" rx="1.5" fill="${INK}"/>
      <rect x="23.5" y="18" width="17" height="29" rx="4" fill="${fill}" stroke="${INK}" stroke-width="2.5"/>
      <path d="M33.5 23 L28 33 L32 33 L30.5 42 L36 31 L32 31 Z" fill="${INK}" stroke="${INK}" stroke-width="0.8" stroke-linejoin="round"/>`,
    slot: `<rect x="20.5" y="11" width="23" height="39" rx="6" ${SLOT}/>`,
  },
  gem: {
    item: fill => `<path d="M24 19 L40 19 L46 27 L32 46 L18 27 Z" fill="${fill}" stroke="${INK}" stroke-width="2.5" stroke-linejoin="round"/>
      <path d="M18 27 L46 27 M27 27 L32 46 L37 27 M24 19 L27 27 L32 19 L37 27 L40 19" fill="none" stroke="${INK}" stroke-width="1.4" stroke-linejoin="round"/>`,
    slot: `<path d="M22.5 16 L41.5 16 L49 26.8 L32 50 L15 26.8 Z" ${SLOT}/>`,
  },
  orb: {
    item: fill => `<circle cx="32" cy="32" r="13" fill="${fill}" stroke="${INK}" stroke-width="2.5"/>
      <circle cx="32" cy="32" r="5" fill="${INK}"/><path d="M24.5 27 A9 9 0 0 1 29 23" fill="none" stroke="#fff" stroke-width="2.2" stroke-linecap="round"/>`,
    slot: `<circle cx="32" cy="32" r="16.5" ${SLOT}/>`,
  },
};
