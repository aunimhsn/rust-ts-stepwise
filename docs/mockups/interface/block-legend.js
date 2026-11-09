// The legend of the map, in a bubble under its button: each element present on the map, with its
// drawing and one sentence. Each entry: its name, its sentence and what the cell draws.
const LEGEND_ELEMENTS = {
  floor: ["Floor", "The robot can move on it.", {}],
  wall: ["Wall", "The robot bumps into it.", { wall: true }],
  paint: ["Paint", "Left by the brush of the robot.", { paint: "red", around: ["e", "w"] }],
  goalPaint: ["Paint to do", "Paint this cell in this color.", { goalPaint: "red", goalAround: ["e", "w"] }],
  item: ["Item", "The robot can pick it up and drop it.", { item: true }],
  slot: ["Item slot", "Drop an item here.", { goalItem: true }],
  robot: ["Robot", "Its eyes show where it faces.", { robot: "east" }],
  stand: ["Where to stand", "End the program here.", { goalStand: true }],
  missed: ["Goal not met", "Shown after a run that missed it.", { missed: true }],
};

// `elements` lists the keys of the elements of the map, `fresh` those introduced by this level.
function appLegend(elements, fresh = []) {
  const row = key => {
    const [name, sentence, cell] = LEGEND_ELEMENTS[key];
    return `<div class="legend-row">
      ${appCell(cell, 36)}
      <div><b>${name}</b>${fresh.includes(key) ? '<i class="legend-new">New</i>' : ""}<span>${sentence}</span></div>
    </div>`;
  };
  return `<div class="app-popup app-legend">
    <div class="legend-head"><b>Legend of the map</b><button type="button" title="Close">${ICONS.close(16)}</button></div>
    ${elements.map(row).join("")}
    <button type="button" class="goal-link legend-link">The map in the manual</button>
  </div>`;
}
