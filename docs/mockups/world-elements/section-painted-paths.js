// Section: the stroke follows the path of the robot while its brush is down.

// The passages of a path through the cells, from a list of "x,y" points.
const pathLinks = (...points) => points.slice(1).map((point, index) => `${points[index]} ${point}`);

const PATHS_MAP = [
  ".......",
  ".rrrr..",
  ".rrrr..",
  ".......",
];

const PATH_EXAMPLES = [
  {
    title: "A U-turn while painting",
    code: `start_painting(red)
repeat 3:
    forward()
turn_right()
forward()
turn_right()
repeat 3:
    forward()
stop_painting()`,
    links: pathLinks("1,1", "2,1", "3,1", "4,1", "4,2", "3,2", "2,2", "1,2"),
    robot: { "1,2": { robot: "west" } },
  },
  {
    title: "Two rows, the brush lifted in between",
    code: `start_painting(red)
repeat 3:
    forward()
stop_painting()
turn_right()
forward()
turn_right()
start_painting(red)
repeat 3:
    forward()
stop_painting()`,
    links: [...pathLinks("1,1", "2,1", "3,1", "4,1"), ...pathLinks("4,2", "3,2", "2,2", "1,2")],
    robot: { "1,2": { robot: "west" } },
  },
];

SECTIONS.push(["Painted paths: the stroke follows the robot", () => PATH_EXAMPLES.map(({ title, code, links, robot }) => `
  <h3>${title}</h3>
  <div class="page-row">
    <div class="page-slot"><span>The program (the robot starts on the left, facing east)</span><pre class="page-code">${code}</pre></div>
    <div class="page-slot"><span>The stroke joins two cells only where the robot went while painting</span>${appMap(PATHS_MAP, robot, 48, links)}</div>
  </div>`).join("")]);
