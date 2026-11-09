// Section: the command menu, opened on an empty line of the sample, filtered, and what it inserts.
const COMMAND_LINES = extra => {
  const lines = SAMPLE_CODE();
  lines[4] = extra;
  return lines;
};

const COMMAND_FORWARD = { name: "forward()", text: "Moves the robot one cell in front. In front of a wall, it bumps and the program stops.", example: ["repeat 3:", "    forward()", "turn_left()"] };
const COMMAND_PAINT = { name: "start_painting(color)", text: "Puts the brush down with the color: paints every cell the robot moves into.", example: ["start_painting(red)", "forward()"] };
const COMMAND_IF = { name: "if", text: "Runs its block when the condition is true.", example: ["if front_is_clear():", "    forward()"] };
const COMMAND_SIDES = { name: "sides", text: "A variable of your program.", example: ["sides = sides + 1"] };

const commandMenuMove = options => appCommandMenu({ category: "Move", groups: [[null, ["forward()", "turn_left()", "turn_right()"]]], chosen: "forward()", details: COMMAND_FORWARD, ...options });
const commandMenuFiltered = options => appCommandMenu({ category: "Paint", counts: { Paint: 2, Sense: 1 }, groups: [["PAINT", ["start_painting(color)", "stop_painting()"]], ["SENSE", ["is_painting()"]]], chosen: "start_painting(color)", details: COMMAND_PAINT, ...options });

const commandBoards = () => [
  ["Control: the statements", appCommandMenu({ category: "Control", groups: [[null, ["if", "else if", "else", "while", "repeat", "break"]]], chosen: "if", details: COMMAND_IF, left: 0, top: 0 })],
  ["Your names: what can be used at the cursor", appCommandMenu({ category: "Your names", groups: [[null, ["paint_side(length)", "sides"]]], chosen: "sides", details: COMMAND_SIDES, left: 0, top: 0 })],
  ["Nothing found", appCommandMenu({ category: "Move", counts: {}, groups: [], empty: "No command found with “pizza”.", left: 0, top: 0 })],
];

// Inserting `if` opens a block: the cursor waits where the condition goes, the next line is indented.
const commandInsertedLines = () => {
  const lines = SAMPLE_CODE();
  lines.splice(4, 1, "        if :", "            ");
  return lines;
};

SECTIONS.push(["Command menu", () => `
  <div class="page-row">
    <div class="page-slot"><span>1280 × 720, Ctrl+Space on an empty line</span>${readyWindow({ editor: { lines: COMMAND_LINES("        "), caret: { line: 5, col: 9 }, cursor: "Ln 5, Col 9" }, overlay: commandMenuMove() })}</div>
    <div class="page-slot"><span>1280 × 720, pai typed: the code changes, the list follows</span>${readyWindow({ editor: { lines: COMMAND_LINES("        pai"), caret: { line: 5, col: 12 }, cursor: "Ln 5, Col 12" }, overlay: commandMenuFiltered() })}</div>
    <div class="page-slot"><span>1280 × 720, if inserted: the cursor on what to fill in</span>${readyWindow({ editor: { lines: commandInsertedLines(), caret: { line: 5, col: 12 }, cursor: "Ln 5, Col 12" } })}</div>
  </div>
  <h3>The panel alone</h3>
  <div class="commands-boards">${commandBoards().map(([title, panel]) => `<div><span class="controls-board-title">${title}</span><div class="app-window commands-board">${panel}</div></div>`).join("")}</div>`]);
