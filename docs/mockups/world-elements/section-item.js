// Section: the item, alone, in its slot, on the paints and carried by the robot.

const ITEM_ROWS = [
  "#########",
  "#.......#",
  "#.rrr.bb#",
  "#..yyy..#",
  "#########",
];
const ITEM_EXTRAS = {
  "1,1": { item: true },
  "3,1": { goalItem: true },
  "5,1": { goalItem: true, item: true },
  "7,1": { robot: "east", carrying: true },
  "3,2": { item: true },
  "6,2": { item: true },
  "4,3": { item: true },
  "1,3": { robot: "north", carrying: true },
  "7,3": { robot: "south", carrying: true },
};

SECTIONS.push(["Item", () => `
  <p>The tabs <b>Item</b> and <b>Item color</b> change the item; its slot (the goal "put an item here") follows its shape. On the map: an item, an empty slot, a full slot, the robot carrying an item in three directions, and items on the three paints.</p>
  <div class="page-row">${appMap(ITEM_ROWS, ITEM_EXTRAS, 56)}</div>
  <h3>Carrying an item, in the four directions (tab <b>Carry</b>)</h3>
  <div class="page-row" id="carry-row">${[64, 32].map(size => `<div class="page-row" style="gap:8px">${["north", "east", "south", "west"].map(robot =>
    appCell({ robot, carrying: true }, size)).join("")}</div>`).join("")}</div>
  <div class="page-row" style="margin-top:16px">${PAINTS_VISIONS.map(vision => `
    <div class="page-slot is-vision-${vision}"><span>${vision[0].toUpperCase() + vision.slice(1)}</span>${appMap(ITEM_ROWS, ITEM_EXTRAS, 32)}</div>`).join("")}
  </div>`]);
