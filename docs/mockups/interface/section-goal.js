// Section: the first visit of a level, its goal open over the starting code.
SECTIONS.push(["First visit, goal open", () => `<div class="page-row"><div class="page-slot"><span>1280 × 720, the bubble of the goal</span>${appWindow({
  header: { goalOpen: true },
  editor: { lines: ["# Paint a square of side 3", "function paint_side(length):", "    # Paint one side here"], state: "Ready", steps: 0, cursor: "Ln 3, Col 26" },
  map: appWorld({ robot: "2,1", facing: "east" }),
  panels: { tab: "Console", content: '<span class="app-weak">Run your program to see what it prints.</span>' },
  overlay: appGoal(),
})}</div></div>`]);
