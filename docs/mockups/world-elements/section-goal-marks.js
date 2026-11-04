// Section: the goal marks of a level, before the run, halfway and once every goal is met.

// The goal: paint an L in blue from the robot, put the item on the slot, then stand in the corner.
const GOAL_ROWS = [
  "#######",
  "#.....#",
  "#.###.#",
  "#.....#",
  "#######",
];
const GOAL_BLUE = ["1,1", "2,1", "3,1", "4,1", "5,1", "5,2", "5,3"];
const goalExtras = (done, extras) => Object.fromEntries([
  ...GOAL_BLUE.map(cell => [cell, { goalPaint: "blue" }]),
  ["3,3", { goalItem: true, ...(done.item ? { item: true } : {}) }],
  ["1,3", { goalStand: true }],
  ...Object.entries(extras),
]);
// The rows painted so far, as the robot left them: `b` a blue paint, `r` a wrong red one.
const paintedRows = painted => GOAL_ROWS.map((row, y) => [...row].map((char, x) => painted[`${x},${y}`] ?? char).join(""));

const GOAL_STAGES = [
  { title: "Before the run", painted: {}, links: [], done: {}, extras: { "1,1": { robot: "east" }, "2,3": { item: true } } },
  {
    title: "Halfway, with a wrong red cell",
    painted: { "1,1": "b", "2,1": "b", "3,1": "b", "4,1": "r" },
    links: pathLinks("1,1", "2,1", "3,1"),
    done: {},
    extras: { "5,1": { robot: "south", goalPaint: "blue" }, "2,3": { item: true } },
  },
  {
    title: "Every goal met",
    painted: Object.fromEntries(GOAL_BLUE.map(cell => [cell, "b"])),
    links: pathLinks(...GOAL_BLUE),
    done: { item: true },
    extras: { "1,3": { robot: "west", goalStand: true } },
  },
];

SECTIONS.push(["Goal marks in a level", () => `
  <p>Paint the blue L, put the item on its mark, then stand in the bottom left corner. The tabs <b>Goal paint</b>, <b>Goal stand</b> and <b>Goal item</b> change the marks.</p>
  <div class="page-row">${GOAL_STAGES.map(({ title, painted, links, done, extras }) => `
    <div class="page-slot"><span>${title}</span>${appMap(paintedRows(painted), goalExtras(done, extras), 48, links)}</div>`).join("")}
  </div>`]);
