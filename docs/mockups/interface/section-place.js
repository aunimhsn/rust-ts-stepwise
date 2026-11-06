// Section: the place of the level in the header, every variant together, from the calmest to the
// boldest. The tab Place also changes it in the windows below.
SECTIONS.push(["Place of the level", () => `<div class="place-strips">${Object.entries(VARIANTS.place.options).map(([key, title]) => `
  <div><span class="place-strip-title">${title}</span><div class="app-window is-strip">${appHeader({ shownMap: 2, passed: [1], place: key })}</div></div>`).join("")}</div>`]);
