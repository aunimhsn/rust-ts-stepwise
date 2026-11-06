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

// The bar of the header: the name, the menus, the way to the settings and, on one bar, the place of the
// level and its maps; on two bars, they go to the second bar (block-second-bar.js). `menu` is the menu
// open, `goalOpen` the bubble of the goal, and `place` the proposal shown.
function appHeader({ shownMap = 1, passed = [], menu, goalOpen = false, place = chosen.place } = {}) {
  const inSecondBar = place.startsWith("bar-");
  const hasProgress = !inSecondBar || ["bar-tabs", "bar-mix-center", "bar-mix-left"].includes(place);
  const isCombination = place.startsWith("mix-");
  const menus = ["Level", "Edit", "Run", ...(chosen.settings === "view" ? ["View"] : []), "Help"];
  const name = chosen.settings === "name"
    ? `<button type="button" class="app-name-menu ${menu === "Stepwise" ? "is-open" : ""}">${appWordmark(21)}<i>${ICONS.down(11)}</i></button>`
    : appWordmark(21);
  const gear = position => `<button type="button" class="app-gear ${position}" title="Settings">${ICONS.gear(18)}</button>`;
  const inPlace = isCombination || place === "bar-mix-center" ? appPlace("stacked", goalOpen)
    : inSecondBar || place === "bar-mix-center" ? ""
    : place === "crumb" ? appCrumb(goalOpen) : appPlace(place, goalOpen);
  const maps = inSecondBar || place === "mix-block" ? ""
    : isCombination ? `<div class="app-header-tabs">${appMapTabs(shownMap, passed)}</div>`
    : appMapButtons(shownMap, passed);
  return `<header class="app-header ${inSecondBar ? "" : "is-one-bar"} is-place-${place} ${hasProgress ? "" : "is-alone"}">
    ${name}
    <nav class="app-menus">${menus.map(item =>
      `<button type="button" class="app-menu ${item === menu ? "is-open" : ""}">${item}</button>`).join("")}</nav>
    ${chosen.settings === "gear-left" ? gear("is-left") : ""}
    ${inPlace}
    ${maps}
    ${chosen.settings === "gear" ? gear("is-right") : ""}
    ${hasProgress ? appProgress() : ""}
  </header>
  ${inSecondBar ? appSecondBar(place, { shownMap, passed, goalOpen }) : ""}`;
}
