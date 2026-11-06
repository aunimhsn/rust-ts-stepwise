// The map of the samples: 10 × 7, with its walls and the goal of the level: a red square of side 3.
const SAMPLE_WALLS = ["7,0", "7,1", "7,2", "0,6", "1,6", "8,5"];
const SAMPLE_SQUARE = ["2,1", "3,1", "4,1", "5,1", "5,2", "5,3", "5,4", "4,4", "3,4", "2,4", "2,3", "2,2"];

// The row on top of the block of the map: a tab per map, then the button Legend.
const appMapRow = (shownMap, passed) => `<div class="app-map-row">${[1, 2, 3].map(number =>
  `<button type="button" class="app-map-tab ${number === shownMap ? "is-shown" : ""} ${passed.includes(number) ? "is-passed" : ""}">Map ${number}</button>`).join("")}<button type="button" class="app-chip">Legend</button></div>`;

// `painted` lists the cells painted in red, in the order of the path of the robot; `robot` is its
// cell and `facing` its direction.
function appWorld({ painted = [], robot, facing }) {
  const rows = Array.from({ length: 7 }, (_, y) => Array.from({ length: 10 }, (_, x) =>
    SAMPLE_WALLS.includes(`${x},${y}`) ? "#" : painted.includes(`${x},${y}`) ? "r" : ".").join(""));
  const extras = Object.fromEntries(SAMPLE_SQUARE.map(cell => [cell, { goalPaint: "red" }]));
  extras[robot] = { ...extras[robot], robot: facing };
  return `<div class="app-world">${appMap(rows, extras, 48, pathLinks(...painted))}</div>`;
}
