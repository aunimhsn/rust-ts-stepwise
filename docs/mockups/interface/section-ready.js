// Section: the level as it opens once the goal is closed: nothing has run yet.
SECTIONS.push(["Ready", () => `<div class="page-row"><div class="page-slot"><span>1280 × 720, the map at its start, its goal marks in sight</span>${appWindow({
  editor: { lines: SAMPLE_CODE(), state: "Ready", steps: 0, cursor: "Ln 13, Col 32" },
  map: appWorld({ robot: "2,1", facing: "east" }),
  panels: { tab: "Console", content: '<span class="app-weak">Run your program to see what it prints.</span>' },
})}</div></div>`]);
