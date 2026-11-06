// The place of the level, in the middle of the header: its chapter above its number and title, a
// button that opens its goal.
const appPlace = open =>
  `<button type="button" class="app-place ${open ? "is-open" : ""}" title="Show the goal"><small>Functions</small><b>Paint a square</b></button>`;

// The number of the level: a circle where the header meets the two columns.
const appLevelMark = () => '<button type="button" class="app-level-mark" title="Level 18: show the goal">18</button>';
