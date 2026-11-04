// Section: the shades of the three paints, side by side in the four visions.

const PAINTS_MAP = [
  "..........",
  ".rrrr.yyy.",
  ".r..#...y.",
  ".r..#.bby.",
  ".bbbb.b...",
  "..........",
];
const PAINTS_EXTRAS = {
  "6,1": { robot: "west" },
  "2,3": { goalPaint: "red" },
  "3,3": { goalPaint: "red" },
  "8,4": { goalPaint: "yellow" },
  "6,0": { goalPaint: "blue" },
  "7,0": { goalPaint: "blue" },
  "2,4": { missed: true },
  "8,0": { item: true },
};
const PAINTS_VISIONS = ["normal", "protanopia", "deuteranopia", "tritanopia"];

SECTIONS.push(["Paints", () => `
  <p>Scarlet, bright amber and cobalt: the red darker and the yellow lighter, so that they stay apart in a red-green color blindness. The maps show them in the four visions at once.</p>
  ${appSwatches()}
  <div class="page-row" style="margin-top:16px">${PAINTS_VISIONS.map(vision => `
    <div class="page-slot is-vision-${vision}"><span>${vision[0].toUpperCase() + vision.slice(1)}</span>${appMap(PAINTS_MAP, PAINTS_EXTRAS, 32)}</div>`).join("")}
  </div>
  <h3>Large, in the chosen vision</h3>
  <div class="page-row">${appMap(PAINTS_MAP, PAINTS_EXTRAS, 56)}</div>`]);
