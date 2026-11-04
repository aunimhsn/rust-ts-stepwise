// Cells and maps drawn from the SVG files of the chosen style, and the paint drawn as a stroke.

const element = name => `world-elements/${chosen.theme}/${name}.svg`;

// The width of the paint stroke, out of 64 for a cell (the robot is 42 wide with its tracks).
const STROKE = 28;
const SIDES = { n: [32, 0], e: [64, 32], s: [32, 64], w: [0, 32] };

// The shape of a stroke in a cell: from the center to each side listed in `around` (n, e, s, w),
// where it meets the stroke of the neighbor, and a round end in the middle of the cell.
const strokeShape = (around, width, color) =>
  Object.entries(SIDES).filter(([side]) => around.includes(side)).map(([, [x, y]]) =>
    `<path d="M32 32 L${x} ${y}" stroke="${color}" stroke-width="${width}"/>`).join("")
  + `<circle cx="32" cy="32" r="${width / 2}" fill="${color}"/>`;

// The paint of a cell: a brush stroke through the centers, which crosses the edge to a painted
// neighbor of the same color and stops, rounded, in the middle of the cell at the ends.
function appPaint(color, around = []) {
  return `<svg viewBox="0 0 64 64" aria-hidden="true">${strokeShape(around, STROKE, THEMES[chosen.theme].paints[color])}</svg>`;
}

// The goal "this color here", in the shape of the stroke to paint, as the "Goal paint" tab says:
// the stroke in transparency (ghost), or its outline (a mask cuts the inside, so that a paint
// already on the cell stays visible through it).
let maskCount = 0;
function appGoalPaint(color, around = []) {
  const shade = THEMES[chosen.theme].paints[color];
  if (chosen.goalPaint === "ghost") {
    return `<svg viewBox="0 0 64 64" aria-hidden="true"><g opacity=".35">${strokeShape(around, STROKE, shade)}</g></svg>`;
  }
  const id = `goal-mask-${maskCount += 1}`;
  return `<svg viewBox="0 0 64 64" aria-hidden="true"><mask id="${id}" maskUnits="userSpaceOnUse" x="0" y="0" width="64" height="64">
    ${strokeShape(around, STROKE + 2, "#fff")}${strokeShape(around, STROKE - 5, "#000")}</mask>
    <rect width="64" height="64" fill="${shade}" mask="url(#${id})"/></svg>`;
}

// A cell: its ground (floor or wall), then the paint, the goal marks, the item and the robot.
// `robot` is a direction (north, east, south, west), `carrying` adds the item on its back.
// `around` and `goalAround` list the sides where the stroke of the paint and of the goal go on.
function appCell({ wall = false, paint, around = [], goalPaint, goalAround = [], goalItem = false, item = false, goalStand = false, robot, carrying = false } = {}, size = 48) {
  const images = [`<img src="${element(wall ? "wall" : "floor")}" alt="">`];
  if (paint) images.push(appPaint(paint, around));
  if (goalPaint) images.push(appGoalPaint(goalPaint, goalAround));
  for (const [shown, name] of [[goalItem, "goal-item"], [item, "item"], [goalStand, "goal-stand"]]) {
    if (shown) images.push(`<img src="${element(name)}" alt="">`);
  }
  if (robot) {
    const shade = paint && THEMES[chosen.theme].paints[paint];
    const mode = chosen.onPaint;
    const overlay = body => images.push(`<svg class="is-${robot}" viewBox="0 0 64 64" aria-hidden="true">${body}</svg>`);
    const carried = (x, y, width) => `<image href="${element("item")}" x="${x}" y="${y}" width="${width}" height="${width}"/>`;
    if (shade && mode === "halo") overlay(`<circle cx="32" cy="32" r="26" fill="${shade}"/>`);
    // With the item in front, the robot is drawn empty-handed and the item ahead of it, on every cell.
    const itemInFront = carrying && mode === "front";
    images.push(`<img class="is-${robot}" src="${element(carrying && !itemInFront ? "robot-carrying" : "robot")}" alt="">`);
    if (shade && (mode === "sensor" || mode === "front")) {
      // The sensor: the panel on the back of the robot lights with the color of its cell.
      overlay(`<rect x="23" y="34" width="18" height="12" rx="3" fill="${shade}"/>`);
      // Carried on the back, the item is drawn again over the panel (scale 0.6 around 32, 40).
      if (carrying && mode === "sensor") overlay(carried(12.8, 20.8, 38.4));
    }
    if (shade && mode === "visor") {
      // The visor lights with the color of the cell, and the eyes turn dark on it.
      overlay(`<rect x="20" y="18" width="24" height="11" rx="5.5" fill="${shade}"/><circle cx="26.5" cy="23.5" r="2.8" fill="#222935"/><circle cx="37.5" cy="23.5" r="2.8" fill="#222935"/>`);
    }
    if (itemInFront) overlay(carried(17.6, -3.4, 28.8));
  }
  return `<div class="app-cell" style="--cell:${size}px">${images.join("")}</div>`;
}

// A map from rows of characters: `#` a wall, `.` a floor, `r` `y` `b` a paint.
// `extras` adds the goal marks, the items and the robot, keyed by "x,y".
function appMap(rows, extras = {}, size = 48) {
  const paints = { r: "red", y: "yellow", b: "blue" };
  const offsets = { n: [0, -1], e: [1, 0], s: [0, 1], w: [-1, 0] };
  const paintAt = (x, y) => paints[rows[y]?.[x]];
  const goalAt = (x, y) => extras[`${x},${y}`]?.goalPaint;
  const sides = (x, y, matches) => Object.keys(offsets).filter(side => matches(x + offsets[side][0], y + offsets[side][1]));
  const cells = rows.flatMap((row, y) => [...row].map((char, x) => {
    const paint = paintAt(x, y);
    const goal = goalAt(x, y);
    return appCell({
      wall: char === "#",
      paint,
      around: paint ? sides(x, y, (nx, ny) => paintAt(nx, ny) === paint) : [],
      // The goal stroke goes on to the goals and to the paints of its color, as the finished drawing would.
      goalAround: goal ? sides(x, y, (nx, ny) => goalAt(nx, ny) === goal || paintAt(nx, ny) === goal) : [],
      ...extras[`${x},${y}`],
    }, size);
  }));
  return `<div class="app-map" style="--columns:${rows[0].length};--cell:${size}px">${cells.join("")}</div>`;
}

const appSwatches = () => `<div class="page-row">${Object.entries(THEMES[chosen.theme].paints).map(([name, shade]) =>
  `<span class="app-swatch"><i style="background:${shade}"></i><code>${name}</code> ${shade}</span>`).join("")}</div>`;
