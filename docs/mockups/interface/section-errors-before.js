// Section: the errors found before running: two independent mistakes, so that the checker reports
// exactly these two. Run lists them in the console; the map stays at its start.
const CODE_WITH_ERRORS = SAMPLE_CODE().map((line, index) => index === 4 ? "        foward()" : index === 9 ? "    paint_side(3, 4)" : line);

SECTIONS.push(["Errors before running", () => `<div class="page-row"><div class="page-slot"><span>1280 × 720, Run clicked: the words underlined, the message of the first one on hover</span>${appWindow({
  editor: {
    lines: CODE_WITH_ERRORS,
    underlines: [{ line: 5, col: 9, length: 6 }, { line: 10, col: 5, length: 16 }],
    hint: { line: 5, col: 9, title: "The name foward doesn't exist.", help: "Did you mean forward?" },
    state: "Ready", errors: "2 errors", steps: 0, cursor: "Ln 5, Col 9",
  },
  map: appWorld({ robot: "2,1", facing: "east" }),
  panels: { tab: "Console", content: [
    appConsoleError("Ln 5", "The name foward doesn't exist.", "Did you mean forward?"),
    appConsoleError("Ln 10", "paint_side takes 1 value, not 2.", "Remove a value from the call."),
    consoleFix(2),
  ].join("") },
})}</div></div>`]);
