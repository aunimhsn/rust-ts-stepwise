// A second bar under the header, for the breadcrumb of the level and its maps: the proposals 9 to 13,
// from the calmest to the boldest. The first bar keeps the name, the menus and the gear.

// The levels of the chapters, for the proposal that shows them all: 17 done, the 18th to play.
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
  plain: ({ crumb, maps }) => `${crumb}${maps}${appProgress()}`,
  columns: ({ crumb, maps }) => `<div class="app-bar-half">${crumb}</div><div class="app-bar-half is-center">${maps}</div>${appProgress()}`,
  tabs: ({ crumb, tabs }) => `<div class="app-bar-half">${crumb}</div><div class="app-bar-half is-tabs">${tabs}</div>`,
  goal: ({ crumb, maps }) => `${crumb}<p class="app-bar-goal">Paint a red square of side 3, starting where the robot stands.</p>${maps}${appProgress()}`,
  levels: ({ crumb, maps }) => `${crumb}${appAllLevels()}${maps}`,
};

const appSecondBar = (place, { shownMap, passed, goalOpen }) => {
  const kind = place.replace("bar-", "");
  return `<div class="app-second-bar is-${kind}">${SECOND_BARS[kind]({ crumb: appCrumb(goalOpen), maps: appMapButtons(shownMap, passed), tabs: appMapTabs(shownMap, passed) })}</div>`;
};
