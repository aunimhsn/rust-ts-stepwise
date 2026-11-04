// The styles of the sheet: their title, their description and the shades of their paints.
// The elements of a style are the SVG files of its folder.
const THEMES = {
  "blueprint": { title: "1 · Blueprint", text: "A night-blue floor with a fine grid, solid blocks for the walls, bright paints that stand out. The closest to the clarity of Shapez 2.", paints: { red: "#ff6257", yellow: "#ffd54a", blue: "#58b8ff" } },
  "graphite": { title: "2 · Graphite", text: "Blueprint on a graphite floor, as in Shapez 2: a neutral gray lets the three paints carry all the color, blue included.", paints: { red: "#ff5a4f", yellow: "#ffd23f", blue: "#4aa8ff" } },
  "daylight": { title: "3 · Daylight", text: "Blueprint by day: a light gray floor, slate walls, deeper paints so that the yellow keeps its contrast; the robot gets an outline.", paints: { red: "#ec4a3e", yellow: "#f0b400", blue: "#2f7de1" } },
  "ink": { title: "4 · Ink", text: "A darker Blueprint, close to black, with the cross marks of a technical drawing at the corners of the cells; the walls are quieter.", paints: { red: "#ff6b5e", yellow: "#ffd75e", blue: "#5cb6ff" } },
  "pulse": { title: "5 · Pulse", text: "Blueprint pushed further: a graphite floor with marks at the corners, chamfered walls lit on one edge, glowing paints that each carry a pattern (stripes, dots, lines), the eyes and the back light of the robot glow.", paints: { red: "#ff5a4f", yellow: "#ffd23f", blue: "#4aa8ff" } },
  "toy": { title: "6 · Toy", text: "Thick rounded shapes, a flat offset shadow, bold colors, a round and friendly robot.", paints: { red: "#f0483e", yellow: "#f9c22e", blue: "#2e7fe8" } },
  "neon": { title: "7 · Neon", text: "A night floor, quiet walls; only the paints, the item and the face of the robot glow. Futuristic but calm.", paints: { red: "#ff4d6d", yellow: "#ffd60a", blue: "#3d9bff" } },
  "synthwave": { title: "8 · Synthwave", text: "An arcade at night: a violet grid that glows, walls like holograms with their scanlines, paints like neon tubes, a chrome robot with a pink visor.", paints: { red: "#ff2e4d", yellow: "#ffe02e", blue: "#2f6bff" } },
};
