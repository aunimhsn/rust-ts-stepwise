// Section: the bubble of the goal, which takes the fonts chosen at the top of the page: alone, then in
// the first visit of a level, over the starting code.
const goalBoard = title => `<div><span class="goal-board-title">${title}</span><div class="app-window goal-board">${appGoal()}</div></div>`;

const firstVisitWindow = () => appWindow({
  header: { goalOpen: true },
  editor: { lines: ["# Paint a square of side 3", "function paint_side(length):", "    # Paint one side here"], state: "Ready", steps: 0, cursor: "Ln 3, Col 26" },
  map: appWorld({ robot: "2,1", facing: "east" }),
  panels: { tab: "Console", content: '<span class="app-weak">Run your program to see what it prints.</span>' },
  overlay: appGoal(),
});

SECTIONS.push(["The bubble of the goal", () => `
  <p class="goal-group">Its fonts: the lists at the top of the page</p>
  ${goalBoard("Alone")}
  <div class="goal-real"><span class="goal-board-title">With the interface behind: the first visit of a level, 1280 × 720</span>${firstVisitWindow()}</div>`]);
