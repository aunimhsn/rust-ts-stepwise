// Section: the combinations of the two lines (2) and the map tabs (11), each window cut to its top. It
// needs pausedWindow: it draws when the page renders, once every script has loaded.
const combinationStrips = keys => `<div class="place-strips">${keys.map(key => `
  <div><span class="place-strip-title">${VARIANTS.place.options[key]}</span><div class="mix-crop">${pausedWindow({ header: { place: key } })}</div></div>`).join("")}</div>`;

SECTIONS.push(["Two lines with map tabs", () => `
  <p class="place-group">On one bar</p>${combinationStrips(["mix-right", "mix-split", "mix-left", "mix-block"])}
  <p class="place-group">On two bars</p>${combinationStrips(["bar-mix-center", "bar-mix-left"])}`]);
