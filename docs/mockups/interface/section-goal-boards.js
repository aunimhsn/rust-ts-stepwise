// Section: the boards of the bubble of the goal: the fifteen fonts on the same layout, then five
// bolder versions in Manrope. The tab Font applies a font to the whole interface below.
const GOAL_HOT = [["hot-1", "A louder title"], ["hot-2", "The number as a big mark"], ["hot-3", "The robot says it"], ["hot-4", "A stroke of paint"], ["hot-5", "A mission card"]];
const goalBoard = (title, options) => `<div><span class="goal-board-title">${title}</span><div class="app-window goal-board">${appGoal(options)}</div></div>`;

SECTIONS.push(["The bubble of the goal", () => `
  <p class="goal-group">Fifteen fonts</p>
  <div class="goal-boards">${Object.entries(FONTS).map(([font, { name }], index) => goalBoard(`${index + 1} · ${name}`, { font })).join("")}</div>
  <p class="goal-group">Five bolder versions</p>
  <div class="goal-boards">${GOAL_HOT.map(([kind, title], index) => goalBoard(`${index + 16} · ${title}`, { kind, font: "manrope" })).join("")}</div>`]);
