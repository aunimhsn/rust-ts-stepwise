// The place of the level in the header, a button that opens its goal: the variants compared in step
// 3a, from the calmest to the boldest. The level is the first of its chapter of three (18 to 20).
const PLACE_ROBOT = '<img src="world-elements/daylight/logo.svg" height="22" alt="">';
const PLACE_STONES = `<span class="place-stones"><b>18</b><u></u><i>19</i><u></u><i>20</i></span>`;

const PLACES = {
  title: () => `<b>18. Paint a square</b><em>${ICONS.down(12)}</em>`,
  stacked: () => `<small>Functions</small><b>18. Paint a square</b>`,
  field: () => `<small>Functions</small><b>Paint a square</b><span class="place-number">18</span><em>${ICONS.down(12)}</em>`,
  stone: () => `<span class="place-stone">18</span><b>Paint a square</b><small>Functions</small>`,
  path: () => `<small>Functions</small>${PLACE_STONES}<b>Paint a square</b>`,
  tag: () => `<small>Functions</small><span class="place-tag-line"><span class="place-big">18</span><b>Paint a square</b></span>`,
  robot: () => `${PLACE_ROBOT}<span class="place-bubble">Level 18: let's paint a square!</span>`,
};

const appPlace = (place, open) =>
  `<button type="button" class="app-place is-${place} ${open ? "is-open" : ""}">${PLACES[place]()}</button>`;
