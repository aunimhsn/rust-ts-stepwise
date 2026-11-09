// Section: the program paused on line 5, in its second side. Every value follows from the sample:
// 21 steps from the start, the first side painted and one cell of the second.
const PAUSED_VARIABLES = () => appVariables("paint_side", [["length", "3"]], [["main program", 10], ["paint_side", 5]]);

const pausedWindow = overrides => appWindow({
  header: overrides?.header,
  size: overrides?.size,
  maps: { shown: 2, passed: [1, 2] },
  editor: { lines: SAMPLE_CODE(), next: 5, breakpoints: [10], controls: { main: "resume" }, state: "Paused at line 5 · Reset to edit", tone: "paused", steps: 21, cursor: "Ln 5, Col 9", ...overrides?.editor },
  map: appWorld({ painted: ["2,1", "3,1", "4,1", "5,1", "5,2"], robot: "5,2", facing: "south", size: overrides?.mapSize }),
  panels: overrides?.panels ?? { tab: "Variables", content: PAUSED_VARIABLES() },
  overlay: overrides?.overlay,
});

SECTIONS.push(["Paused", () => `<div class="page-row"><div class="page-slot"><span>1280 × 720, paused on line 5, map 1 and map 2 passed, map 2 shown</span>${pausedWindow()}</div></div>`]);
