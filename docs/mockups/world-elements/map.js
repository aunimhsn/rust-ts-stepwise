// Cells and maps drawn from the SVG files of the chosen style, and the paint drawn as a stroke.

const element = name => `world-elements/${chosen.theme}/${name}.svg`;

// The shade of a paint, decided in step 1b (themes.js).
const paintShade = color => PAINT_SHADES[color];

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
  return `<svg viewBox="0 0 64 64" aria-hidden="true">${strokeShape(around, STROKE, paintShade(color))}</svg>`;
}

// The goal "this color here": the stroke to paint, in transparency. A paint already on the cell
// stays visible through it.
function appGoalPaint(color, around = []) {
  const shade = paintShade(color);
  return `<svg viewBox="0 0 64 64" aria-hidden="true"><g opacity=".35">${strokeShape(around, STROKE, shade)}</g></svg>`;
}

// The robot at the end of a run (`lit`): "success" when every goal is met, "failure" when not, as
// the tabs "Success" and "Failure" say. The eyes are drawn over those of robot.svg.
const LIGHT = "#ffd23f";
const EYES = [26.5, 37.5];
function appLit(lit) {
  if (lit === "success" && chosen.success !== "halo") {
    return `<svg viewBox="0 0 64 64" aria-hidden="true">${EYES.map(x => `<circle cx="${x}" cy="23.5" r="3.4" fill="${LIGHT}"/>`).join("")}</svg>`;
  }
  if (lit === "failure" && chosen.failure !== "unchanged") {
    const eye = x => chosen.failure === "dim"
      ? `<circle cx="${x}" cy="23.5" r="2.8" fill="#7d8798"/>`
      : `<circle cx="${x}" cy="23.5" r="3.2" fill="${"#222935"}"/><path d="M${x - 2.6} 24.5 L${x + 2.6} 24.5" stroke="#7d8798" stroke-width="2.2" stroke-linecap="round"/>`;
    return `<svg viewBox="0 0 64 64" aria-hidden="true">${EYES.map(eye).join("")}</svg>`;
  }
  return "";
}
const appHalo = () => `<svg viewBox="0 0 64 64" aria-hidden="true" style="overflow:visible"><defs><radialGradient id="halo">
  <stop offset=".4" stop-color="${LIGHT}" stop-opacity=".8"/><stop offset="1" stop-color="${LIGHT}" stop-opacity="0"/></radialGradient></defs>
  <circle cx="32" cy="32" r="50" fill="url(#halo)"/></svg>`;

// A cell: its ground (floor or wall), then the paint, the goal marks, the item and the robot.
// `robot` is a direction (north, east, south, west), `carrying` adds the item in its bay, `pincer`
// its pincer, out only while it picks up or drops an item.
// `around` and `goalAround` list the sides where the stroke of the paint and of the goal go on.
// `missed` marks a goal not met once the program has ended.
// `wallEdge` adds the darker edge at the bottom of a wall above a floor cell, and `wallRim` the light
// rim at the top of a wall below a floor cell: a flat hint of height.
function appCell({ wall = false, wallEdge = true, wallRim = true, paint, around = [], goalPaint, goalAround = [], goalItem = false, item = false, goalStand = false, robot, carrying = false, pincer = false, lit, missed = false } = {}, size = 48) {
  const images = [`<img src="${element(wall ? "wall" : "floor")}" alt="">`];
  if (wall && wallRim) images.push(`<img src="${element("wall-rim")}" alt="">`);
  if (wall && wallEdge) images.push(`<img src="${element("wall-edge")}" alt="">`);
  if (paint) images.push(appPaint(paint, around));
  // A goal met hides its mark: the paint alone shows that the cell is done.
  if (goalPaint && goalPaint !== paint) images.push(appGoalPaint(goalPaint, goalAround));
  for (const [shown, name] of [[goalItem, "goal-item"], [item, "item"], [goalStand, "goal-stand"]]) {
    if (shown) images.push(`<img src="${element(name)}" alt="">`);
  }
  // The robot covers the paint of its cell: the learner follows it through the code.
  // The item it carries lies in its bay, the light panel at its back, and turns with it.
  const carried = carrying ? `<img class="is-carried" src="${element("item")}" alt="">` : "";
  // Its yellow light spreads on the floor around it, under it.
  if (robot && lit === "success" && chosen.success !== "eyes") images.push(appHalo());
  if (robot) images.push(`<div class="app-robot is-${robot}"><img src="${element("robot")}" alt="">${pincer ? `<img src="${element("pincer")}" alt="">` : ""}${carried}${appLit(lit)}</div>`);
  // A goal not met: a small red flag planted in the corner of its cell, over the robot. Its red,
  // distinct from the red paint, may follow the error color of the design system (step 4).
  if (missed) images.push(`<img src="${element("goal-missed")}" alt="">`);
  // A lit cell lies over its neighbors, so that its light spreads on them.
  return `<div class="app-cell${lit === "success" ? " is-lit" : ""}" style="--cell:${size}px">${images.join("")}</div>`;
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
    // Outside the grid counts as a wall: a wall on the bottom row has no edge, on the top row no rim.
    const floorAt = (nx, ny) => rows[ny]?.[nx] !== undefined && rows[ny][nx] !== "#";
    return appCell({
      wall: char === "#",
      wallEdge: floorAt(x, y + 1),
      wallRim: floorAt(x, y - 1),
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

// The contrast ratio of a shade against the floor (WCAG), to check that a paint stands out.
const luminance = hex => [1, 3, 5].map(index => parseInt(hex.slice(index, index + 2), 16) / 255)
  .map(value => value <= 0.03928 ? value / 12.92 : ((value + 0.055) / 1.055) ** 2.4)
  .reduce((sum, value, index) => sum + value * [0.2126, 0.7152, 0.0722][index], 0);
const contrast = (first, second) => {
  const [light, dark] = [luminance(first), luminance(second)].sort((a, b) => b - a);
  return ((light + 0.05) / (dark + 0.05)).toFixed(1);
};
const FLOOR = "#e1e5ea";

const appSwatches = () => `<div class="page-row">${Object.keys(PAINT_SHADES).map(color =>
  `<span class="app-swatch"><i style="background:${paintShade(color)}"></i><code>${color}</code> ${paintShade(color)} · ${contrast(paintShade(color), FLOOR)}:1 on the floor</span>`).join("")}</div>`;
