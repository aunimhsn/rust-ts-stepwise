// Section: each element alone, large and at its size on the map. The overlays lie on a floor.

const ELEMENTS = [
  ["Ground", [["Floor", {}], ["Wall", { wall: true }]]],
  ["Paints", ["red", "yellow", "blue"].map(color => [`<code>${color}</code>`, { paint: color }])],
  ["Item", [["Item", { item: true }]]],
  ["Goal marks", [
    ["Stand here", { goalStand: true }],
    ...["red", "yellow", "blue"].map(color => [`<code>${color}</code> here`, { goalPaint: color }]),
    ["Item here", { goalItem: true }],
    ["Red here, painted blue", { paint: "blue", goalPaint: "red" }],
    ["Item here, done", { goalItem: true, item: true }],
    ["Goal not met", { goalItem: true, missed: true }],
  ]],
  ["Robot", [
    ...["north", "east", "south", "west"].map(direction => [`Facing ${direction}`, { robot: direction }]),
    ["Carrying an item", { robot: "north", carrying: true }],
    ["On a paint", { robot: "north", paint: "yellow" }],
    ["On its goal", { robot: "north", goalStand: true }],
  ]],
];

SECTIONS.push(["Elements alone", () => ELEMENTS.map(([group, slots]) => `
  <h3>${group}</h3>
  <div class="page-row">${slots.map(([label, cell]) => `
    <div class="page-slot"><span>${label}</span><div class="page-row" style="gap:8px;align-items:flex-end">${appCell(cell, 128)}${appCell(cell, 48)}</div></div>`).join("")}
  </div>`).join("")]);
