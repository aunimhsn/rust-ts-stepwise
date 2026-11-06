// The place of the level in one bar, a button that opens its goal: the proposals 1 to 8, from the
// calmest to the boldest, then the breadcrumb with the maps over their column. The level is the first
// of its chapter of three (18 to 20).
const PLACE_ROBOT = '<img src="world-elements/daylight/logo.svg" height="22" alt="">';
const PLACE_STONES = '<span class="place-stones"><b>18</b><u></u><i>19</i><u></u><i>20</i></span>';

const PLACES = {
  title: () => `<b>18. Paint a square</b><em>${ICONS.down(12)}</em>`,
  stacked: () => "<small>Functions</small><b>18. Paint a square</b>",
  field: () => `<small>Functions</small><b>Paint a square</b><span class="place-number">18</span><em>${ICONS.down(12)}</em>`,
  stone: () => '<span class="place-stone">18</span><b>Paint a square</b><small>Functions</small>',
  path: () => `<small>Functions</small>${PLACE_STONES}<b>Paint a square</b>`,
  tag: () => '<small>Functions</small><span class="place-tag-line"><span class="place-big">18</span><b>Paint a square</b></span>',
  robot: () => `${PLACE_ROBOT}<span class="place-bubble">Level 18: let's paint a square!</span>`,
};

const appPlace = (place, open) =>
  `<button type="button" class="app-place is-${place} ${open ? "is-open" : ""}" title="Show the goal">${PLACES[place]()}</button>`;

const appCrumb = open => `<button type="button" class="app-crumb ${open ? "is-open" : ""}" title="Show the goal"><span>Functions</span><i>${ICONS.chevron(11)}</i><b>18. Paint a square</b></button>`;

// The word Map and the buttons of the maps, each with its bar, green once the map has passed.
const appMapButtons = (shownMap, passed) => `<div class="app-maps"><span>Map</span>${[1, 2, 3].map(number =>
  `<button type="button" class="app-map-button ${number === shownMap ? "is-shown" : ""} ${passed.includes(number) ? "is-passed" : ""}"><span>${number}</span></button>`).join("")}</div>`;

const appProgress = () => '<div class="app-progress" title="17 of 20 levels done"><i style="width:85%"></i></div>';
