// Section: the four menus, each in its window, then the menus that change with the state of the run.
const menuBoards = () => [
  ["Run, the program ready", appMenuRows("Run", "ready")],
  ["Run, the program running: Step greyed", appMenuRows("Run", "running")],
  ["Run, the program paused", appMenuRows("Run", "paused")],
  ["Edit, the program running: the code is locked", appMenuRows("Edit", "running")],
];

SECTIONS.push(["The four menus", () => `
  <div class="page-row">
    <div class="page-slot"><span>Level: Next level greyed until the level is done</span>${readyWindow({ header: { menu: "Level" }, overlay: appMenu("Level") })}</div>
    <div class="page-slot"><span>Edit</span>${readyWindow({ header: { menu: "Edit" }, overlay: appMenu("Edit", { hovered: 3 }) })}</div>
    <div class="page-slot"><span>Run, paused, with its speeds</span>${pausedWindow({ header: { menu: "Run" }, overlay: appMenu("Run", { run: "paused", hovered: 4 }) + appSpeedMenu() })}</div>
    <div class="page-slot"><span>Help</span>${readyWindow({ header: { menu: "Help" }, overlay: appMenu("Help") })}</div>
  </div>
  <h3>They follow the run</h3>
  <div class="menu-boards">${menuBoards().map(([title, rows]) => `<div><span class="controls-board-title">${title}</span><div class="app-window menu-board">${appDropdown(0, rows, undefined, 0)}</div></div>`).join("")}</div>`]);
