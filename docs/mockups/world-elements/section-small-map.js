// Section: every element together on a small map, to check that they fit.

const SMALL_MAP = [
  "#........",
  "..rrr..#.",
  "..#ry..#.",
  ".b#.y....",
  ".bb...bb.",
  "###...bb.",
];

const SMALL_MAP_EXTRAS = {
  "1,4": { robot: "east" },
  "5,4": { item: true },
  "4,0": { item: true },
  "5,1": { goalPaint: "red" },
  "6,1": { goalPaint: "red" },
  "4,4": { goalPaint: "yellow" },
  "6,5": { goalPaint: "red" },
  "8,1": { goalItem: true },
  "8,3": { goalItem: true },
  "8,5": { goalStand: true },
};

// The passages the robot painted (see pathLinks in section-painted-paths.js).
const SMALL_MAP_LINKS = [
  ...pathLinks("2,1", "3,1", "4,1"), ...pathLinks("3,1", "3,2"),
  ...pathLinks("4,2", "4,3"),
  ...pathLinks("1,3", "1,4", "2,4"),
  ...pathLinks("6,4", "7,4", "7,5", "6,5"),
];

SECTIONS.push(["Small map", () => `
  <div class="page-row">
    <div class="page-slot"><span>Cells of 48 px</span>${appMap(SMALL_MAP, SMALL_MAP_EXTRAS, 48, SMALL_MAP_LINKS)}</div>
    <div class="page-slot"><span>Cells of 32 px</span>${appMap(SMALL_MAP, SMALL_MAP_EXTRAS, 32, SMALL_MAP_LINKS)}</div>
  </div>
  <h3>Paints</h3>${appSwatches()}`]);
