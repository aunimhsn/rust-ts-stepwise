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

// `shownMap` is the map on the screen (from 1), `passed` the maps that have passed, `menu` the menu
// open and `goalOpen` the bubble of the goal.
function appHeader({ shownMap = 1, passed = [], menu, goalOpen = false } = {}) {
  return `<header class="app-header">
    ${appWordmark(21)}
    <nav class="app-menus">${["Level", "Edit", "Run", "Help"].map(item =>
      `<button type="button" class="app-menu ${item === menu ? "is-open" : ""}">${item}</button>`).join("")}</nav>
    <button type="button" class="app-place ${goalOpen ? "is-open" : ""}"><i>${ICONS.target(15)}</i><span>Functions</span><i>${ICONS.chevron(11)}</i><b>18. Paint a square</b></button>
    <div class="app-maps"><span>Map</span>${[1, 2, 3].map(number =>
      `<button type="button" class="app-map-button ${number === shownMap ? "is-shown" : ""} ${passed.includes(number) ? "is-passed" : ""}">${number}</button>`).join("")}</div>
    <div class="app-progress" title="17 of 20 levels done"><i style="width:85%"></i></div>
  </header>`;
}
