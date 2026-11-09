// Section: the legend, on the first level with an item (level 14): the item and its slot are new.
const CARRY_PLACE = { chapter: "Carry", title: "Pick it up", number: 14 };
const CARRY_CODE = ["# Bring the item to its slot", "forward()", "forward()", "pick_up()", "turn_right()", "forward()", "drop()", "forward()"];
const CARRY_ELEMENTS = ["floor", "wall", "item", "slot", "robot", "stand", "missed"];

function carryWorld() {
  const rows = Array.from({ length: 7 }, (_, y) => Array.from({ length: 10 }, (_, x) => SAMPLE_WALLS.includes(`${x},${y}`) ? "#" : ".").join(""));
  return appRowsWorld(rows, { "2,2": { robot: "east" }, "4,2": { item: true }, "4,3": { goalItem: true }, "4,4": { goalStand: true } }, 48);
}

const carryWindow = (legend, overlay) => appWindow({
  header: { place: CARRY_PLACE },
  maps: { legend },
  editor: { lines: CARRY_CODE, state: "Ready", steps: 0, cursor: "Ln 8, Col 10" },
  map: carryWorld(),
  panels: { tab: "Console", content: '<span class="app-weak">Run your program to see what it prints.</span>' },
  overlay,
});

SECTIONS.push(["Legend of the map", () => `
  <div class="page-row">
    <div class="page-slot"><span>1280 × 720, closed: the dot says that the map has new elements not yet seen</span>${carryWindow("new")}</div>
    <div class="page-slot"><span>1280 × 720, open, from its button or Help › Legend of the map: it blocks nothing</span>${carryWindow("open", appLegend(CARRY_ELEMENTS, ["item", "slot"]))}</div>
  </div>
  <h3>The legend alone</h3>
  <div class="page-row">
    <div class="page-slot"><span>A level that paints: nothing new</span><div class="app-window legend-board">${appLegend(["floor", "wall", "paint", "goalPaint", "robot", "missed"])}</div></div>
    <div class="page-slot"><span>The longest: every element on one map</span><div class="app-window legend-board">${appLegend(Object.keys(LEGEND_ELEMENTS), ["item"])}</div></div>
  </div>`]);
