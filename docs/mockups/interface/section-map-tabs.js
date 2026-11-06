// Section: how a pill shows that its map has passed, for the two states of the shown map: on the left,
// the shown map has passed; on the right, it hasn't. The pale teal on tabs comes first, to compare.
const mapRow = (bar, shape, passed) => `<div class="app-window is-strip">${appMapRow(2, passed, bar, shape)}<div class="app-world"></div></div>`;
const mapRows = (bar, shape, title) => `
  <div><span class="tab-bar-title">${title} · map 2 shown, passed</span>${mapRow(bar, shape, [1, 2])}</div>
  <div><span class="tab-bar-title">${title} · map 2 shown, not passed yet</span>${mapRow(bar, shape, [1])}</div>`;

SECTIONS.push(["A map that has passed, as pills", () => `<div class="tab-bar-rows">
  ${mapRows("teal-soft", "tabs", "For reference: pale teal on tabs")}
  ${Object.entries(VARIANTS.tabBar.options).map(([bar, title]) => mapRows(bar, "pills", `${title} pill`)).join("")}</div>`]);
