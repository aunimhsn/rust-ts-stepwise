// Section: every element of the console, each in a block at its real size, numbered once.
const CONSOLE_PRINTS = ["Sides done: 1", "Sides done: 2", "Sides done: 3", "Sides done: 4"];
const CONSOLE_LONG = Array.from({ length: 30 }, (_, index) => `Step ${index + 1}`);

const consoleBoards = () => [
  ["Nothing yet", { content: '<span class="app-weak">Run your program to see what it prints.</span>' }],
  ["The lines of print", { content: consolePrints(...CONSOLE_PRINTS) }],
  ["Every kind of value, and a long line", { content: consolePrints("3 true red", "x: 12 y: 4", "A long line goes on to the next one without leaving the block, and nothing has to scroll sideways to read it.") }],
  ["A long output: the last lines in sight", { content: consoleLast(consolePrints(...CONSOLE_LONG)) }],
  ["A syntax error, before running", { content: appConsoleError("Line 9, col 17", "Expected ':' at the end of the while line.", "Add ':' after the condition.") }],
  ["Several errors, before running", { content: [
    appConsoleError("Line 8, col 5", "The name sids doesn't exist.", "Declare it with let, or check its spelling."),
    appConsoleError("Line 10, col 5", "paint_side takes 1 value, not 2.", "Remove a value from the call."),
    appConsoleError("Line 14, col 10", "Names use snake_case: paint_side, not paintSide.", "Rename this function."),
  ].join("") }],
  ["An error while running, after some lines", { dot: true, content: consolePrints("Sides done: 1") + appConsoleError("Line 5, col 9", "The robot bumped into a wall.", "forward() needs a free cell in front of the robot.") }],
  ["A value that can't be used", { dot: true, content: appConsoleError("Line 6, col 15", "You can't divide by zero.", "sides is 0 here: check its value before dividing.") }],
  ["Too many calls", { dot: true, content: appConsoleError("Line 3, col 5", "200 calls in progress: the limit is reached.", "paint_side keeps calling itself: give it a way to stop.") }],
  ["A goal not met", { content: consolePrints(...CONSOLE_PRINTS) + consoleGoal("The goal isn't met: 7 cells of the square aren't red.") }],
  ["Several goals not met", { content: [consoleGoal("The goal isn't met: 7 cells of the square aren't red."), consoleGoal("The goal isn't met: the robot isn't on its square.")].join("") }],
  ["A map passed, others remain", { content: consolePrints(...CONSOLE_PRINTS) + consolePassed("Map 1 passed. Try map 2.") }],
  ["The last map passed", { content: consolePassed("Map 3 passed. Level done!") }],
];

SECTIONS.push(["The elements of the console", () => `<div class="console-boards">${consoleBoards().map(([title, panel], index) => `
  <div><span class="console-board-title">${index + 1} · ${title}</span><div class="app-window console-board">${appPanels({ tab: "Console", ...panel })}</div></div>`).join("")}</div>`]);
