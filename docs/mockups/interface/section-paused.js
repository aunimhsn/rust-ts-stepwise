// Section: the program paused on line 5, in its second side. Every value follows from the sample:
// 21 steps from the start, the first side painted and one cell of the second.
const pausedWindow = overrides => appWindow({
  header: overrides?.header,
  maps: { shown: 2, passed: [1, 2] },
  editor: { lines: SAMPLE_CODE(), next: 5, breakpoints: [10], state: "Paused at line 5 · Reset to edit", steps: 21, cursor: "Ln 5, Col 9" },
  map: appWorld({ painted: ["2,1", "3,1", "4,1", "5,1", "5,2"], robot: "5,2", facing: "south" }),
  panels: { tab: "Variables", content: appVariables("paint_side", [["length", "3"]], [["main program", 10], ["paint_side", 5]]) },
  overlay: overrides?.overlay,
  markFit: overrides?.markFit,
});

SECTIONS.push(["Paused", () => `<div class="page-row"><div class="page-slot"><span>1280 × 720, paused on line 5, map 1 and map 2 passed, map 2 shown</span>${pausedWindow()}</div></div>`]);
