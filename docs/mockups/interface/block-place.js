// The place of the level and its maps, in the one bar of the header. Two readings are compared:
// `crumb` puts the breadcrumb after the menus, over the code, and the maps right above the map they
// show; `stacked` puts the chapter above the title in the middle, and the maps on the right.
const appCrumb = open => `<button type="button" class="app-crumb ${open ? "is-open" : ""}" title="Show the goal"><span>Functions</span><i>${ICONS.chevron(11)}</i><b>18. Paint a square</b></button>`;

const appStacked = open => `<button type="button" class="app-stacked ${open ? "is-open" : ""}" title="Show the goal"><small>Functions</small><b>18. Paint a square</b></button>`;

// The word Map and the buttons of the maps, each with its bar, green once the map has passed.
const appMapButtons = (shownMap, passed) => `<div class="app-maps"><span>Map</span>${[1, 2, 3].map(number =>
  `<button type="button" class="app-map-button ${number === shownMap ? "is-shown" : ""} ${passed.includes(number) ? "is-passed" : ""}"><span>${number}</span></button>`).join("")}</div>`;

const appProgress = () => '<div class="app-progress" title="17 of 20 levels done"><i style="width:85%"></i></div>';
