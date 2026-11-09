// The map of the samples: 10 × 7, with its walls and the goal of the level: a red square of side 3.
const SAMPLE_WALLS = ["7,0", "7,1", "7,2", "0,6", "1,6", "8,5"];
const SAMPLE_SQUARE = ["2,1", "3,1", "4,1", "5,1", "5,2", "5,3", "5,4", "4,4", "3,4", "2,4", "2,3", "2,2"];

// The row on top of the block of the map: a tab per map, none in the sandbox, then the button Legend,
// `legend` being "new" while it carries its dot and "open" while its bubble is open.
const appMapRow = (shownMap, passed, sandbox = false, legend) => `<div class="app-map-row">${(sandbox ? [] : [1, 2, 3]).map(number =>
  `<button type="button" class="app-map-tab ${number === shownMap ? "is-shown" : ""} ${passed.includes(number) ? "is-passed" : ""}">Map ${number}</button>`).join("")}<button type="button" class="app-chip ${legend === "open" ? "is-open" : ""}">Legend${legend === "new" ? '<i class="legend-dot"></i>' : ""}</button></div>`;

// The life of the map, cell by cell, in units out of 64 for a cell. The lines: a thicker line every five
// cells, counted from the cell at the bottom left, half of it on each side of the edge between two cells.
const FIVE_LINE_WIDTH = 4.5;
const FIVE_LINE_OPACITY = 0.1;
function appFiveLines(x, y, height) {
  const half = FIVE_LINE_WIDTH / 2;
  const fromBottom = height - 1 - y;
  const rects = [];
  if (x > 0 && x % 5 === 0) rects.push(`<rect x="0" y="0" width="${half}" height="64"/>`);
  if (x % 5 === 4) rects.push(`<rect x="${64 - half}" y="0" width="${half}" height="64"/>`);
  if (fromBottom > 0 && fromBottom % 5 === 0) rects.push(`<rect x="0" y="${64 - half}" width="64" height="${half}"/>`);
  if (fromBottom % 5 === 4) rects.push(`<rect x="0" y="0" width="64" height="${half}"/>`);
  return rects.length ? `<svg viewBox="0 0 64 64" aria-hidden="true"><g fill="#222935" opacity="${FIVE_LINE_OPACITY}">${rects.join("")}</g></svg>` : "";
}

// The shadows: every wall drawn again, moved away from the sun, at the top right, and kept on this floor
// cell. Together, the walls cast one flat shadow, without a darker overlap.
const SHADOW_LENGTH = 10;
const SHADOW_OPACITY = 0.1;
const SHADOW_OFFSET = SHADOW_LENGTH * Math.SQRT1_2;
function appWallShadow(x, y, rows) {
  const rects = [];
  for (const ny of [y - 1, y, y + 1]) for (const nx of [x - 1, x, x + 1]) {
    if (rows[ny]?.[nx] !== "#") continue;
    const [left, top] = [(nx - x) * 64 - SHADOW_OFFSET, (ny - y) * 64 + SHADOW_OFFSET];
    if (left < 64 && left > -64 && top < 64 && top > -64) rects.push(`<rect x="${left.toFixed(2)}" y="${top.toFixed(2)}" width="64" height="64"/>`);
  }
  return rects.length ? `<svg viewBox="0 0 64 64" aria-hidden="true"><g fill="#222935" opacity="${SHADOW_OPACITY}">${rects.join("")}</g></svg>` : "";
}

// The lines and the shadows of every floor cell of a map, to give to appMap.
function appMapLife(rows) {
  const extras = {};
  rows.forEach((row, y) => [...row].forEach((char, x) => {
    if (char !== "#") extras[`${x},${y}`] = { lines: appFiveLines(x, y, rows.length), shade: appWallShadow(x, y, rows) };
  }));
  return extras;
}

// A map from its rows, with its lines and shadows; `marks` adds to each cell what appCell draws on it.
function appRowsWorld(rows, marks, size, links) {
  const extras = appMapLife(rows);
  for (const [cell, mark] of Object.entries(marks)) extras[cell] = { ...extras[cell], ...mark };
  return `<div class="app-world">${appMap(rows, extras, size, links)}</div>`;
}

// The sides of a cell of the goal where its stroke goes on: toward the cells before and after it on the
// square, never toward a paint beside it that isn't part of the goal.
function squareGoalSides(index) {
  const point = cell => cell.split(",").map(Number);
  const [x, y] = point(SAMPLE_SQUARE[index]);
  return [-1, 1].map(step => point(SAMPLE_SQUARE[(index + step + SAMPLE_SQUARE.length) % SAMPLE_SQUARE.length]))
    .map(([nx, ny]) => nx > x ? "e" : nx < x ? "w" : ny > y ? "s" : "n");
}

// `painted` lists the cells painted in red, in the order of the path of the robot; `robot` is its
// cell and `facing` its direction. `lit` is the end of a run (success or failure), `missed` the cells
// of the goals not met, `size` the side of a cell.
function appWorld({ painted = [], robot, facing, lit, missed = [], size = 48 }) {
  const rows = Array.from({ length: 7 }, (_, y) => Array.from({ length: 10 }, (_, x) =>
    SAMPLE_WALLS.includes(`${x},${y}`) ? "#" : painted.includes(`${x},${y}`) ? "r" : ".").join(""));
  const marks = {};
  SAMPLE_SQUARE.forEach((cell, index) => { marks[cell] = { goalPaint: "red", goalAround: squareGoalSides(index) }; });
  for (const cell of missed) marks[cell] = { ...marks[cell], missed: true };
  marks[robot] = { ...marks[robot], robot: facing, lit };
  return appRowsWorld(rows, marks, size, pathLinks(...painted));
}
