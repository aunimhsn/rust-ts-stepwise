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

// The goal "this color here": the stroke to paint, in transparency. A paint already on the cell
// stays visible through it.
function appGoalPaint(color, around = []) {
  const shade = THEMES[chosen.theme].paints[color];
  return `<svg viewBox="0 0 64 64" aria-hidden="true"><g opacity=".35">${strokeShape(around, STROKE, shade)}</g></svg>`;
}

// A cell: its ground (floor or wall), then the paint, the goal marks, the item and the robot.
// `robot` is a direction (north, east, south, west), `carrying` adds the item on its back.
// `around` and `goalAround` list the sides where the stroke of the paint and of the goal go on.
// `missed` marks a goal not met once the program has ended.
function appCell({ wall = false, paint, around = [], goalPaint, goalAround = [], goalItem = false, item = false, goalStand = false, robot, carrying = false, missed = false } = {}, size = 48) {
  const images = [`<img src="${element(wall ? "wall" : "floor")}" alt="">`];
  if (paint) images.push(appPaint(paint, around));
  // A goal met hides its mark: the paint alone shows that the cell is done.
  if (goalPaint && goalPaint !== paint) images.push(appGoalPaint(goalPaint, goalAround));
  for (const [shown, name] of [[goalItem, "goal-item"], [item, "item"], [goalStand, "goal-stand"]]) {
    if (shown) images.push(`<img src="${element(name)}" alt="">`);
  }
  // The robot covers the paint of its cell: the learner follows it through the code.
  if (robot) images.push(`<img class="is-${robot}" src="${element(carrying ? "robot-carrying" : "robot")}" alt="">`);
  // A goal not met: a small red flag planted in the corner of its cell, over the robot. Its red,
  // distinct from the red paint, may follow the error color of the design system (step 4).
  if (missed) images.push(`<img src="${element("goal-missed")}" alt="">`);
  return `<div class="app-cell" style="--cell:${size}px">${images.join("")}</div>`;
}

// The passages of a path through the cells, from a list of "x,y" points.
const pathLinks = (...points) => points.slice(1).map((point, index) => `${points[index]} ${point}`);

// A map from rows of characters: `#` a wall, `.` a floor, `r` `y` `b` a paint.
// `extras` adds the goal marks, the items and the robot, keyed by "x,y".
// `links`, when given, lists the passages painted by the robot ("x,y x,y"): the stroke joins two
// cells only there. Without it, the stroke joins every neighbor of the same color.
function appMap(rows, extras = {}, size = 48, links) {
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
      around: paint ? sides(x, y, (nx, ny) => paintAt(nx, ny) === paint
        && (!links || links.includes(`${x},${y} ${nx},${ny}`) || links.includes(`${nx},${ny} ${x},${y}`))) : [],
      // The goal stroke goes on to the goals and to the paints of its color, as the finished drawing would.
      goalAround: goal ? sides(x, y, (nx, ny) => goalAt(nx, ny) === goal || paintAt(nx, ny) === goal) : [],
      ...extras[`${x},${y}`],
    }, size);
  }));
  return `<div class="app-map" style="--columns:${rows[0].length};--cell:${size}px">${cells.join("")}</div>`;
}

const appSwatches = () => `<div class="page-row">${Object.entries(THEMES[chosen.theme].paints).map(([name, shade]) =>
  `<span class="app-swatch"><i style="background:${shade}"></i><code>${name}</code> ${shade}</span>`).join("")}</div>`;
