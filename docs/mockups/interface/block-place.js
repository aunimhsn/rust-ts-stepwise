// The place of the level, in the middle of the header: its chapter above its number and title, a
// button that opens its goal.
const appPlace = open =>
  `<button type="button" class="app-place ${open ? "is-open" : ""}" title="Show the goal"><small>Functions</small><b>18. Paint a square</b></button>`;
