// Section: the sizes of the window. From about 1600 pixels wide, the panels stand side by side and the
// editor stops at about 80 characters; the text keeps its size, the cells of the map reach 64 pixels.
const LONG_COMMENT = "# Paint each side with its own call, then count the sides until the square is done";
const LARGE_CODE = [...SAMPLE_CODE(), "", LONG_COMMENT];

const largeWindow = ([width, height], hint) => pausedWindow({
  size: [width, height],
  mapSize: 64,
  editor: {
    lines: LARGE_CODE,
    underlines: [{ line: 15, col: 81, length: LONG_COMMENT.length - 80, kind: "long" }],
    hint: hint ? { line: 15, col: 40, title: "Lines are easier to read under 80 characters." } : undefined,
  },
  panels: { side: { console: consolePrints("Sides done: 1"), variables: PAUSED_VARIABLES() } },
});

// The largest map: 16 × 12, its cells smaller so that the whole map stays in its block.
const LARGEST_ROWS = [
  "................",
  ".######..######.",
  ".#............#.",
  ".#..rrrrrr....#.",
  ".#.......r....#.",
  ".#.......r....#.",
  ".#.......r....#.",
  ".#............#.",
  ".######..######.",
  "................",
  "................",
  "................",
];

const largestWindow = () => appWindow({
  editor: { lines: SAMPLE_CODE(), state: "Ready", steps: 0, cursor: "Ln 13, Col 32" },
  map: appRowsWorld(LARGEST_ROWS, { "9,7": { robot: "south" }, "3,10": { item: true }, "12,10": { item: true }, "8,10": { goalStand: true } }, 35),
  panels: { tab: "Console", content: '<span class="app-weak">Run your program to see what it prints.</span>' },
});

SECTIONS.push(["The other sizes", () => `<div class="page-row">
  <div class="page-slot"><span>1920 × 1080, paused: the editor stops at about 80 characters, the thin line at the 80th column, the dots under a longer line and their message on hover</span>${largeWindow([1920, 1080], true)}</div>
  <div class="page-slot"><span>1600 × 900, the size from which the panels stand side by side</span>${largeWindow([1600, 900], false)}</div>
  <div class="page-slot"><span>1280 × 720, the largest map (16 × 12): cells of 35 pixels</span>${largestWindow()}</div>
</div>`]);
