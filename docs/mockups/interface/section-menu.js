// Section: a menu open: the menu Run, or the one that gives the settings, following the tab Settings.
// The shortcuts come from the wireframe and are decided in step 3b.
const RUN_MENU = [["Resume", "F5"], ["Step", "F11"], ["Reset", "Shift+F5"], null, ["Speed", "›"]];
const OPEN_MENUS = {
  gear: ["Run", 255, RUN_MENU, 1],
  "gear-left": ["Run", 255, RUN_MENU, 1],
  name: ["Stepwise", 8, [["All levels", ""], ["Sandbox", ""], null, ["Settings…", "Ctrl+,"]], 3],
  view: ["View", 301, [["Theme", "›"], ["Sounds", "Off"], null, ["Settings…", "Ctrl+,"]], 0],
};

SECTIONS.push(["Menu open", () => {
  const [menu, left, rows, hovered] = OPEN_MENUS[chosen.settings];
  return `<div class="page-row"><div class="page-slot"><span>1280 × 720, the menu ${menu}</span>${pausedWindow({ header: { menu }, overlay: appDropdown(left, rows, hovered) })}</div></div>`;
}]);
