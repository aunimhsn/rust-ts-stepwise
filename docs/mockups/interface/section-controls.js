// Section: the three controls alone, in the corner of an editor, numbered once.
const controlsBoards = () => [
  ["Ready or ended: Run", {}],
  ["Running: Pause, Step greyed", { main: "pause", stepOff: true }],
  ["Paused: Resume", { main: "resume" }],
  ["While the learner types: pale", { pale: true }],
  ["Hover on Run", { tip: 0 }],
  ["Hover on Pause", { main: "pause", stepOff: true, tip: 0 }],
  ["Hover on Step", { main: "resume", tip: 1 }],
  ["Hover on Reset", { tip: 2 }],
];

SECTIONS.push(["The three controls", () => `<div class="controls-boards">${controlsBoards().map(([title, controls], index) => `
  <div><span class="controls-board-title">${index + 1} · ${title}</span><div class="app-window controls-board"><div class="app-code">${appControls(controls)}</div></div></div>`).join("")}</div>`]);
