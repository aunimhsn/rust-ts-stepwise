// Section: how a tab shows that its map has passed, for the two states of the shown map: on the left,
// the shown map has passed; on the right, it hasn't.
const tabRow = (key, passed) => `<div class="app-window is-strip">${appMapRow(2, passed, key)}<div class="app-world"></div></div>`;

SECTIONS.push(["A map that has passed", () => `<div class="tab-bar-rows">${Object.entries(VARIANTS.tabBar.options).map(([key, title]) => `
  <div><span class="tab-bar-title">${title} · map 2 shown, passed</span>${tabRow(key, [1, 2])}</div>
  <div><span class="tab-bar-title">${title} · map 2 shown, not passed yet</span>${tabRow(key, [1])}</div>`).join("")}</div>`]);
