// Section: paused on a breakpoint, before the second call of paint_side: 16 steps from the start, the
// first side painted, the robot turned to the south.
SECTIONS.push(["Paused on a breakpoint", () => `<div class="page-row"><div class="page-slot"><span>1280 × 720, the dot and the highlighted line together, the tab Variables opened by itself</span>${appWindow({
  editor: { lines: SAMPLE_CODE(), next: 10, breakpoints: [10], controls: { main: "resume" }, state: "Paused at line 10 · breakpoint · Reset to edit", tone: "paused", steps: 16, cursor: "Ln 10, Col 5" },
  map: appWorld({ painted: ["2,1", "3,1", "4,1", "5,1"], robot: "5,1", facing: "south" }),
  panels: { tab: "Variables", content: appVariables("the main program", [["sides", "1"]], [["main program", 10]]) },
})}</div></div>`]);
