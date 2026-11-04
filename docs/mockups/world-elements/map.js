// Cells and maps drawn from the SVG files of the chosen style.

const element = name => `world-elements/${chosen.theme}/${name}.svg`;

// The paint of a cell, drawn as the "Paint" tab says: its own SVG file per cell, the whole
// cell, or joined over the edges to the neighbors of the same color (`around` lists them:
// n, e, s, w and the corners ne, se, sw, nw), so that a painted area reads as one stroke.
function appPaint(color, around = []) {
  if (chosen.paint === "cell") return `<img src="${element(`paint-${color}`)}" alt="">`;
  const has = direction => around.includes(direction);
  const shade = THEMES[chosen.theme].paints[color];
  if (chosen.paint === "stroke") {
    // A brush stroke through the centers: it crosses the edge to a painted neighbor of the same
    // color, and stops in the middle of the cell, rounded, at the ends of the painted area.
    const half = Number(chosen.width) / 2;
    const ends = { n: [32, 0], e: [64, 32], s: [32, 64], w: [0, 32] };
    const lines = Object.entries(ends).filter(([direction]) => has(direction)).map(([, [x, y]]) =>
      `<path d="M32 32 L${x} ${y}" stroke="${shade}" stroke-width="${half * 2}"/>`).join("");
    return `<svg viewBox="0 0 64 64" aria-hidden="true">${lines}<circle cx="32" cy="32" r="${half}" fill="${shade}"/></svg>`;
  }
  const shapes = chosen.paint === "full" ? [[0, 0, 64, 64]] : [
    [5, 5, 54, 54, 7],
    ...[["n", 5, 0, 54, 12], ["s", 5, 52, 54, 12], ["w", 0, 5, 12, 54], ["e", 52, 5, 12, 54]]
      .filter(([direction]) => has(direction)).map(([, ...box]) => box),
    ...[["nw", 0, 0], ["ne", 52, 0], ["se", 52, 52], ["sw", 0, 52]]
      .filter(([corner]) => has(corner) && has(corner[0]) && has(corner[1])).map(([, x, y]) => [x, y, 12, 12]),
  ];
  // "Joined + seams": the edges between two painted cells stay visible, faintly, to count them.
  // Each cell draws half of the line, on its own side of the edge.
  const seams = chosen.paint !== "seams" ? "" : [["n", "M0 0 L64 0"], ["e", "M64 0 L64 64"], ["s", "M0 64 L64 64"], ["w", "M0 0 L0 64"]]
    .filter(([direction]) => has(direction))
    .map(([, line]) => `<path d="${line}" stroke="#ffffff" stroke-opacity=".5" stroke-width="2"/>`).join("");
  return `<svg viewBox="0 0 64 64" aria-hidden="true">${shapes.map(([x, y, width, height, radius = 0]) =>
    `<rect x="${x}" y="${y}" width="${width}" height="${height}" rx="${radius}" fill="${shade}"/>`).join("")}${seams}</svg>`;
}

// A cell: its ground (floor or wall), then the paint, the goal marks, the item and the robot.
// `robot` is a direction (north, east, south, west), `carrying` adds the item on its back.
function appCell({ wall = false, paint, around, goalPaint, goalItem = false, item = false, goalStand = false, robot, carrying = false } = {}, size = 48) {
  const layers = [wall ? "wall" : "floor"];
  if (goalPaint) layers.push(`goal-paint-${goalPaint}`);
  if (goalItem) layers.push("goal-item");
  if (item) layers.push("item");
  if (goalStand) layers.push("goal-stand");
  const images = layers.map(name => `<img src="${element(name)}" alt="">`);
  if (paint) images.splice(1, 0, appPaint(paint, around));
  if (robot) {
    const shade = paint && THEMES[chosen.theme].paints[paint];
    const mode = chosen.onPaint;
    const overlay = (body, extra = "") => images.push(`<svg class="is-${robot}" viewBox="0 0 64 64" aria-hidden="true"${extra}>${body}</svg>`);
    const carried = (x, y, size) => `<image href="${element("item")}" x="${x}" y="${y}" width="${size}" height="${size}"/>`;
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
  const offsets = { n: [0, -1], ne: [1, -1], e: [1, 0], se: [1, 1], s: [0, 1], sw: [-1, 1], w: [-1, 0], nw: [-1, -1] };
  const cells = rows.flatMap((row, y) => [...row].map((char, x) => {
    const around = Object.keys(offsets).filter(direction => {
      const [dx, dy] = offsets[direction];
      return paints[char] && rows[y + dy]?.[x + dx] === char;
    });
    return appCell({ wall: char === "#", paint: paints[char], around, ...extras[`${x},${y}`] }, size);
  }));
  return `<div class="app-map" style="--columns:${rows[0].length};--cell:${size}px">${cells.join("")}</div>`;
}

const appSwatches = () => `<div class="page-row">${Object.entries(THEMES[chosen.theme].paints).map(([name, shade]) =>
  `<span class="app-swatch"><i style="background:${shade}"></i><code>${name}</code> ${shade}</span>`).join("")}</div>`;
