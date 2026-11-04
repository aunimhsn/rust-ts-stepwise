// Section: every element together on a small map, to check that they fit.

const SMALL_MAP = [
  "#........",
  "..rrr..#.",
  "..#.y..#.",
  ".b#.y....",
  ".b.......",
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

SECTIONS.push(["Small map", () => `
  <div class="page-row">
    <div class="page-slot"><span>Cells of 48 px</span>${appMap(SMALL_MAP, SMALL_MAP_EXTRAS, 48)}</div>
    <div class="page-slot"><span>Cells of 32 px</span>${appMap(SMALL_MAP, SMALL_MAP_EXTRAS, 32)}</div>
  </div>
  <h3>Paints</h3>${appSwatches()}`]);
