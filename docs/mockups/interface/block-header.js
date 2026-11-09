// The name beside the logo, on the line of the menus. As decided on the sheet of the world elements,
// the head is a quarter taller than the capitals, and its middle meets the middle of the lowercase
// letters.
function appWordmark(fontSize) {
  const context = document.createElement("canvas").getContext("2d");
  context.font = "600 100px Fredoka, sans-serif";
  const letterHeight = letter => context.measureText(letter).actualBoundingBoxAscent / 100;
  const height = letterHeight("S") * fontSize * 1.25;
  const middleOfText = letterHeight("x") * fontSize / 2 + 0.5 / 44 * fontSize;
  return `<div class="app-name" style="font-size:${fontSize}px"><img src="world-elements/daylight/logo.svg" height="${height.toFixed(1)}" alt="" style="margin-right:${(fontSize * 0.18).toFixed(1)}px;transform:translateY(${(height / 2 - middleOfText).toFixed(2)}px)"><span>stepwise</span></div>`;
}

// The header: the name, the menus, the place of the level (block-place.js), the gear of the settings at
// its right end, the thin bar of the progress. `menu` is the menu open and `goalOpen` the bubble of the goal.
// The sandbox has no goal and no progress.
function appHeader({ menu, goalOpen = false, sandbox = false } = {}) {
  return `<header class="app-header">
    ${appWordmark(21)}
    <nav class="app-menus">${["Level", "Edit", "Run", "Help"].map(item =>
      `<button type="button" class="app-menu ${item === menu ? "is-open" : ""}">${item}</button>`).join("")}</nav>
    ${sandbox ? appPlaceSandbox() : appPlace(goalOpen)}
    <button type="button" class="app-gear" title="Settings">${ICONS.gear(18)}</button>
    ${sandbox ? "" : '<div class="app-progress" title="17 of 20 levels done"><i style="width:85%"></i></div>'}
  </header>`;
}
