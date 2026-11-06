// The map of the samples: 10 × 7, with its walls and the goal of the level: a red square of side 3.
const SAMPLE_WALLS = ["7,0", "7,1", "7,2", "0,6", "1,6", "8,5"];
const SAMPLE_SQUARE = ["2,1", "3,1", "4,1", "5,1", "5,2", "5,3", "5,4", "4,4", "3,4", "2,4", "2,3", "2,2"];

// The row on top of the block of the map: a tab per map, then the button Legend.
const appMapRow = (shownMap, passed) => `<div class="app-map-row">${[1, 2, 3].map(number =>
  `<button type="button" class="app-map-tab ${number === shownMap ? "is-shown" : ""} ${passed.includes(number) ? "is-passed" : ""}">Map ${number}</button>`).join("")}<button type="button" class="app-chip">Legend</button></div>`;

// The life of the map, cell by cell. The lines: a thicker line every five cells, counted from the cell at
// the bottom left, half of it on each side of the edge between two cells.
const GRID_WIDTHS = { thin: 3, medium: 5, strong: 7 };
function appFiveLines(x, y, height) {
  const half = GRID_WIDTHS[chosen.gridLine] / 2;
  const fromBottom = height - 1 - y;
  const rects = [];
  if (x > 0 && x % 5 === 0) rects.push(`<rect x="0" y="0" width="${half}" height="64"/>`);
  if (x % 5 === 4) rects.push(`<rect x="${64 - half}" y="0" width="${half}" height="64"/>`);
  if (fromBottom > 0 && fromBottom % 5 === 0) rects.push(`<rect x="0" y="${64 - half}" width="64" height="${half}"/>`);
  if (fromBottom % 5 === 4) rects.push(`<rect x="0" y="0" width="64" height="${half}"/>`);
  return rects.length ? `<svg viewBox="0 0 64 64" fill="#b3bcc8" aria-hidden="true">${rects.join("")}</svg>` : "";
}

// The shadows: every wall drawn again, moved away from the sun, and kept on this floor cell. Together, the
// walls cast one flat shadow, without a darker overlap.
const SUN_DIRECTIONS = { "top-left": [1, 1], top: [0, 1], "top-right": [-1, 1] };
const SHADOW_LENGTHS = { short: 8, medium: 14, long: 22 };
function appWallShadow(x, y, rows) {
  const length = SHADOW_LENGTHS[chosen.shadow];
  const [dx, dy] = SUN_DIRECTIONS[chosen.sun].map(side => side * length);
  const rects = [];
  for (const ny of [y - 1, y, y + 1]) for (const nx of [x - 1, x, x + 1]) {
    if (rows[ny]?.[nx] !== "#") continue;
    const [left, top] = [(nx - x) * 64 + dx, (ny - y) * 64 + dy];
    if (left < 64 && left > -64 && top < 64 && top > -64) rects.push(`<rect x="${left}" y="${top}" width="64" height="64"/>`);
  }
  return rects.length ? `<svg viewBox="0 0 64 64" aria-hidden="true"><g fill="#222935" opacity=".2">${rects.join("")}</g></svg>` : "";
}

// The lines and the shadows of every floor cell of a map, to give to appMap.
function appMapLife(rows) {
  const extras = {};
  rows.forEach((row, y) => [...row].forEach((char, x) => {
    if (char !== "#") extras[`${x},${y}`] = { lines: appFiveLines(x, y, rows.length), shade: appWallShadow(x, y, rows) };
  }));
  return extras;
}

// `painted` lists the cells painted in red, in the order of the path of the robot; `robot` is its
// cell and `facing` its direction.
function appWorld({ painted = [], robot, facing }) {
  const rows = Array.from({ length: 7 }, (_, y) => Array.from({ length: 10 }, (_, x) =>
    SAMPLE_WALLS.includes(`${x},${y}`) ? "#" : painted.includes(`${x},${y}`) ? "r" : ".").join(""));
  const extras = appMapLife(rows);
  for (const cell of SAMPLE_SQUARE) extras[cell] = { ...extras[cell], goalPaint: "red" };
  extras[robot] = { ...extras[robot], robot: facing };
  return `<div class="app-world">${appMap(rows, extras, 48, pathLinks(...painted))}</div>`;
}
