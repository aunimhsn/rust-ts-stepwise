// Section: a run that ends without meeting its goal. With paint_side(2): 50 steps, a square of side 2
// painted, 7 of the 12 cells of the goal left, each with its flag.
const SMALL_SQUARE = ["2,1", "3,1", "4,1", "4,2", "4,3", "3,3", "2,3", "2,2", "2,1"];
const CELLS_MISSED = SAMPLE_SQUARE.filter(cell => !SMALL_SQUARE.includes(cell));

SECTIONS.push(["Goal not met", () => `<div class="page-row"><div class="page-slot"><span>1280 × 720, a flag on each cell missed, the eyes of the robot dimmed</span>${appWindow({
  editor: { lines: SAMPLE_CODE(2), state: "Ended · goal not met", steps: 50, cursor: "Ln 10, Col 17" },
  map: appWorld({ painted: SMALL_SQUARE, robot: "2,1", facing: "east", lit: "failure", missed: CELLS_MISSED }),
  panels: { tab: "Console", content: consolePrints("Sides done: 1", "Sides done: 2", "Sides done: 3", "Sides done: 4") + consoleGoal(`The goal isn't met: ${CELLS_MISSED.length} cells of the square aren't red.`) },
})}</div></div>`]);
