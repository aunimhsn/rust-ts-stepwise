// A second bar under the header, for the breadcrumb of the level and its maps: the variants compared
// in step 3a, from the calmest to the boldest. The first bar keeps the name, the menus and the gear.
const appCrumb = open => `<button type="button" class="app-crumb ${open ? "is-open" : ""}" title="Show the goal"><span>Functions</span><i>${ICONS.chevron(11)}</i><b>18. Paint a square</b></button>`;

// The levels of the chapters, for the variant that shows them all: 17 done, the 18th to play.
const CHAPTER_LEVELS = [3, 3, 2, 3, 2, 2, 2, 3];
function appAllLevels() {
  let level = 0;
  return `<div class="app-all-levels" title="17 of 20 levels done">${CHAPTER_LEVELS.map(count => `<span>${Array.from({ length: count }, () => {
    level += 1;
    return `<i class="${level < 18 ? "is-done" : level === 18 ? "is-current" : ""}"></i>`;
  }).join("")}</span>`).join("")}</div>`;
}

const appMapTabs = (shownMap, passed) => [1, 2, 3].map(number =>
  `<button type="button" class="app-map-tab ${number === shownMap ? "is-shown" : ""} ${passed.includes(number) ? "is-passed" : ""}">Map ${number}</button>`).join("");

const SECOND_BARS = {
  plain: ({ crumb, maps }) => `${crumb}<div class="app-maps">${maps}</div>${appProgress()}`,
  columns: ({ crumb, maps }) => `<div class="app-bar-half">${crumb}</div><div class="app-bar-half is-center"><div class="app-maps">${maps}</div></div>${appProgress()}`,
  tabs: ({ crumb, tabs }) => `<div class="app-bar-half">${crumb}</div><div class="app-bar-half is-tabs">${tabs}</div>`,
  goal: ({ crumb, maps }) => `${crumb}<p class="app-bar-goal">Paint a red square of side 3, starting where the robot stands.</p><div class="app-maps">${maps}</div>${appProgress()}`,
  levels: ({ crumb, maps }) => `${crumb}${appAllLevels()}<div class="app-maps">${maps}</div>`,
};

const appSecondBar = (place, { shownMap, passed, goalOpen }) => {
  const kind = place.replace("bar-", "");
  return `<div class="app-second-bar is-${kind}">${SECOND_BARS[kind]({ crumb: appCrumb(goalOpen), maps: appMapButtons(shownMap, passed), tabs: appMapTabs(shownMap, passed) })}</div>`;
};
