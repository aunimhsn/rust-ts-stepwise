// Cells and maps drawn from the SVG files of the chosen style.

const element = name => `world-elements/${chosen.theme}/${name}.svg`;

// A cell: its ground (floor or wall), then the paint, the goal marks, the item and the robot.
// `robot` is a direction (north, east, south, west), `carrying` adds the item on its back.
function appCell({ wall = false, paint, goalPaint, goalItem = false, item = false, goalStand = false, robot, carrying = false } = {}, size = 48) {
  const layers = [wall ? "wall" : "floor"];
  if (paint) layers.push(`paint-${paint}`);
  if (goalPaint) layers.push(`goal-paint-${goalPaint}`);
  if (goalItem) layers.push("goal-item");
  if (item) layers.push("item");
  if (goalStand) layers.push("goal-stand");
  const images = layers.map(name => `<img src="${element(name)}" alt="">`);
  if (robot) images.push(`<img class="is-${robot}" src="${element(carrying ? "robot-carrying" : "robot")}" alt="">`);
  return `<div class="app-cell" style="--cell:${size}px">${images.join("")}</div>`;
}

// A map from rows of characters: `#` a wall, `.` a floor, `r` `y` `b` a paint.
// `extras` adds the goal marks, the items and the robot, keyed by "x,y".
function appMap(rows, extras = {}, size = 48) {
  const paints = { r: "red", y: "yellow", b: "blue" };
  const cells = rows.flatMap((row, y) => [...row].map((char, x) =>
    appCell({ wall: char === "#", paint: paints[char], ...extras[`${x},${y}`] }, size)));
  return `<div class="app-map" style="--columns:${rows[0].length};--cell:${size}px">${cells.join("")}</div>`;
}

const appSwatches = () => `<div class="page-row">${Object.entries(THEMES[chosen.theme].paints).map(([name, shade]) =>
  `<span class="app-swatch"><i style="background:${shade}"></i><code>${name}</code> ${shade}</span>`).join("")}</div>`;
