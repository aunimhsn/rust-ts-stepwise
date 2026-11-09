// Section: the sandbox, the level screen without what belongs to a goal. The spiral: 55 steps from the
// start, its blue stroke along the path of the robot, which ends on 6,1 facing south.
const SANDBOX_CODE = [
  "# The sandbox: no goal, try anything",
  "let length = 1",
  "start_painting(blue)",
  "while length < 6:",
  "    repeat length:",
  "        forward()",
  "    turn_right()",
  "    length = length + 1",
  "stop_painting()",
  'print("Spiral done")',
];
const SANDBOX_ROWS = [
  "..........",
  "........#.",
  "........#.",
  "......rr#.",
  "..........",
  "......##..",
  "yyy.......",
];
const SANDBOX_SPIRAL = ["3,3", "4,3", "4,4", "4,5", "3,5", "2,5", "1,5", "1,4", "1,3", "1,2", "1,1", "2,1", "3,1", "4,1", "5,1", "6,1"];
const SANDBOX_LINKS = ["6,3 7,3", "0,6 1,6", "1,6 2,6"];

function sandboxWorld({ path = [], robot, facing }) {
  const rows = SANDBOX_ROWS.map((row, y) => [...row].map((char, x) => path.includes(`${x},${y}`) ? "b" : char).join(""));
  return appRowsWorld(rows, { "2,3": { item: true }, "9,5": { item: true }, [robot]: { robot: facing } }, 48, [...SANDBOX_LINKS, ...pathLinks(...path)]);
}

const sandboxWindow = ({ path, robot, facing, state, steps, console }) => appWindow({
  sandbox: true,
  editor: { lines: SANDBOX_CODE, state, steps, cursor: "Ln 10, Col 21" },
  map: sandboxWorld({ path, robot, facing }),
  panels: { tab: "Console", content: console },
});

SECTIONS.push(["Sandbox", () => `<div class="page-row">
  <div class="page-slot"><span>1280 × 720, ready</span>${sandboxWindow({ robot: "3,3", facing: "east", state: "Ready", steps: 0, console: '<span class="app-weak">Run your program to see what it prints.</span>' })}</div>
  <div class="page-slot"><span>1280 × 720, ended: no goal, so nothing passes or fails</span>${sandboxWindow({ path: SANDBOX_SPIRAL, robot: "6,1", facing: "south", state: "Ended", steps: 55, console: consolePrints("Spiral done") })}</div>
</div>`]);
