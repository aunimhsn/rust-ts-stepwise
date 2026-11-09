// Section: the shortcuts of comfort of the editor, on a small program, each one after its key.
const COMFORT_SAMPLE = ["repeat 4:", "    forward()", "    turn_right()", "print(\"Done\")"];

const comfortBoards = () => [
  ["Two lines selected", { lines: COMFORT_SAMPLE, selected: [2, 3], cursor: "Ln 3, Col 17" }],
  ["Tab: four spaces more on each line", { lines: ["repeat 4:", "        forward()", "        turn_right()", "print(\"Done\")"], selected: [2, 3], cursor: "Ln 3, Col 21" }],
  ["Shift+Tab: four spaces less", { lines: ["repeat 4:", "forward()", "turn_right()", "print(\"Done\")"], selected: [2, 3], cursor: "Ln 3, Col 13" }],
  [`${KEYS_EDIT.comment} comments the lines, or uncomments them`, { lines: ["repeat 4:", "    # forward()", "    # turn_right()", "print(\"Done\")"], selected: [2, 3], cursor: "Ln 3, Col 19" }],
  [`${KEYS_EDIT.lineDown}: the line goes down, and ${KEYS_EDIT.lineUp} up`, { lines: ["repeat 4:", "    turn_right()", "    forward()", "print(\"Done\")"], caret: { line: 3, col: 14 }, cursor: "Ln 3, Col 14" }],
  ["Ctrl held: the name under the mouse underlined", { lines: COMFORT_SAMPLE, underlines: [{ line: 2, col: 5, length: 7, kind: "link" }], caret: { line: 4, col: 14 }, cursor: "Ln 4, Col 14" }],
];

SECTIONS.push(["Shortcuts of comfort of the editor", () => `<div class="comfort-boards">${comfortBoards().map(([title, editor]) => `
  <div><span class="controls-board-title">${title}</span><div class="app-window comfort-board">${appEditor({ state: "Ready", steps: 0, controls: { pale: true }, ...editor })}</div></div>`).join("")}</div>`]);
