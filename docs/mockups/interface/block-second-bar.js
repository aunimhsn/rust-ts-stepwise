// The second bar, under the header, each half over its column: the breadcrumb of the level, a button
// that opens its goal, over the code, and the maps right above the map they show.
const appCrumb = open => `<button type="button" class="app-crumb ${open ? "is-open" : ""}" title="Show the goal"><span>Functions</span><i>${ICONS.chevron(11)}</i><b>18. Paint a square</b></button>`;

// The word Map and the buttons of the maps, each with its bar, green once the map has passed.
const appMapButtons = (shownMap, passed) => `<span>Map</span>${[1, 2, 3].map(number =>
  `<button type="button" class="app-map-button ${number === shownMap ? "is-shown" : ""} ${passed.includes(number) ? "is-passed" : ""}"><span>${number}</span></button>`).join("")}`;

const appSecondBar = ({ shownMap, passed, goalOpen }) => `<div class="app-second-bar">
  <div class="app-bar-half">${appCrumb(goalOpen)}</div>
  <div class="app-bar-half is-center"><div class="app-maps">${appMapButtons(shownMap, passed)}</div></div>
  <div class="app-progress" title="17 of 20 levels done"><i style="width:85%"></i></div>
</div>`;
