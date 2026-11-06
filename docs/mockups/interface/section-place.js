// Section: the place of the level, every proposal together, numbered, on one bar then on two bars.
// The tab Place also changes it in the windows below.
const placeStrips = keys => `<div class="place-strips">${keys.map(key => `
  <div><span class="place-strip-title">${VARIANTS.place.options[key]}</span><div class="app-window is-strip">${appHeader({ shownMap: 2, passed: [1], place: key })}</div></div>`).join("")}</div>`;

SECTIONS.push(["Place of the level", () => {
  const keys = Object.keys(VARIANTS.place.options);
  return `<p class="place-group">On one bar</p>${placeStrips(keys.filter(key => !key.startsWith("bar-")))}
    <p class="place-group">On two bars</p>${placeStrips(keys.filter(key => key.startsWith("bar-")))}`;
}]);
