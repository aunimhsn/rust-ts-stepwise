// Section: the place of the word Map, on pills in pale teal and on tabs; map 1 has passed, map 2 is shown.
SECTIONS.push(["The word Map", () => `<div class="tab-bar-rows">${Object.entries(VARIANTS.mapWord.options).map(([word, title]) => `
  <div><span class="tab-bar-title">${title} · pills</span>${mapRow("teal-soft", "pills", [1], word)}</div>
  <div><span class="tab-bar-title">${title} · tabs</span>${mapRow("teal-soft", "tabs", [1], word)}</div>`).join("")}</div>`]);
