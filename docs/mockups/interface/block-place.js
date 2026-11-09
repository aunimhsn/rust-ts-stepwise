// The place of the level, in the middle of the header: its chapter above its number and title, a
// button that opens its goal.
const SAMPLE_PLACE = { chapter: "Functions", title: "Paint a square", number: 18 };
const appPlace = (open, { chapter, title } = SAMPLE_PLACE) =>
  `<button type="button" class="app-place ${open ? "is-open" : ""}" title="Show the goal"><small>${chapter}</small><b>${title}</b></button>`;

// The number of the level: a circle where the header meets the two columns.
const appLevelMark = (number = SAMPLE_PLACE.number) => `<button type="button" class="app-level-mark" title="Level ${number}: show the goal">${number}</button>`;

// The sandbox has no goal: its place in the header isn't a button.
const appPlaceSandbox = () => `<div class="app-place is-sandbox"><i>${ICONS.diamond(14)}</i><b>Sandbox</b></div>`;
