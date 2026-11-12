// Section: two colors meeting along a path, when the robot changes the color of its brush or
// paints over a stroke of another color (open decision of step 8: A or B, side by side).

const MEETING_WAYS = {
  halves: "A · Half in each color",
  cut: "B · Cut at the border",
};

// The first example, frame by frame: the robot starts on the left of the row, facing east.
const MEETING_FRAMES = [
  {
    code: `start_painting(red)
repeat 2:
    forward()`,
    note: "The robot has painted three cells in red, joined by two passages. A and B are the same.",
    rows: [".......", ".rrr...", "......."],
    links: pathLinks("1,1", "2,1", "3,1"),
    robot: { "3,1": { robot: "east" } },
  },
  {
    code: `start_painting(blue)`,
    note: "The brush changes color: the cell of the robot turns blue at once (the robot is left out here, to show its cell). The passage between the last red cell and this one was painted red: A keeps it, half red and half blue; B no longer draws it.",
    rows: [".......", ".rrb...", "......."],
    links: pathLinks("1,1", "2,1", "3,1"),
    robot: {},
  },
  {
    code: `repeat 2:
    forward()
stop_painting()`,
    note: "The robot goes on in blue. A: one stroke that changes color where the blue begins. B: two strokes, with a gap where the color changed.",
    rows: [".......", ".rrbbb.", "......."],
    links: pathLinks("1,1", "2,1", "3,1", "4,1", "5,1"),
    robot: { "5,1": { robot: "east" } },
  },
];

// The second example, at its end only.
const MEETING_CROSS = {
  code: `start_painting(red)
repeat 4:
    forward()
stop_painting()
turn_left()
forward()
turn_left()
repeat 2:
    forward()
turn_left()
start_painting(blue)
repeat 2:
    forward()
stop_painting()`,
  rows: [".......", "...b...", ".rrbrr.", "...b...", "......."],
  links: [...pathLinks("1,2", "2,2", "3,2", "4,2", "5,2"), ...pathLinks("3,1", "3,2", "3,3")],
  robot: { "3,3": { robot: "south" } },
};

const meetingBoth = ({ rows, robot, links }) => Object.entries(MEETING_WAYS).map(([way, label]) =>
  `<div class="page-slot"><span>${label}</span>${appMap(rows, robot, 40, links, way)}</div>`).join("");

SECTIONS.push(["Two colors meeting on a path: A or B?", () => `
  <p>The robot paints a cell in red, then the same cell in blue: what becomes of the red passage that led to it?
  Nothing changes for the sensors and the goals, which only look at the cells: this is a matter of drawing.
  <b>A</b>: each cell draws its half of a passage in its own color. <b>B</b>: a passage is drawn only between two
  cells of the same color.</p>
  <h3>The color changed on the way, step by step</h3>
  ${MEETING_FRAMES.map(({ code, note, ...frame }, index) => `
  <div class="page-row">
    <div class="page-slot" style="width:340px;flex:none"><span>${index + 1}. The program runs</span><pre class="page-code">${code}</pre><p>${note}</p></div>
    ${meetingBoth(frame)}
  </div>`).join("")}
  <h3>A blue stroke across a red one</h3>
  <div class="page-row">
    <div class="page-slot" style="width:340px;flex:none"><span>The program (the robot starts on the left of the middle row, facing east)</span><pre class="page-code">${MEETING_CROSS.code}</pre>
    <p>A: the red line goes under the blue cell. B: the red line is cut in two.</p></div>
    ${meetingBoth(MEETING_CROSS)}
  </div>`]);
