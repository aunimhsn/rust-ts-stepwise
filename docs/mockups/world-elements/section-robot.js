// Section: the robot, in its four directions, empty and carrying an item, at three sizes.

const ROBOT_ROWS = [
  "#######",
  "#.....#",
  "#.yyy.#",
  "#.....#",
  "#######",
];
const ROBOT_EXTRAS = {
  "1,1": { robot: "east" },
  "3,1": { robot: "south", carrying: true },
  "5,1": { goalItem: true },
  "1,3": { robot: "north", carrying: true, goalStand: true },
  "3,2": { robot: "west" },
  "5,3": { item: true },
};

SECTIONS.push(["Robot", () => `
  <p>The bay at its back is larger, so that the item it carries reads at 32 px. The tab <b>Grippers</b> changes the grippers, folded at its front: they store the item in the bay (the animation comes with the mini-scenes of step 3).</p>
  ${[96, 48, 32].map(size => `<div class="page-row" style="gap:8px;margin-bottom:8px">${["north", "east", "south", "west"].flatMap(robot =>
    [appCell({ robot }, size), appCell({ robot, carrying: true }, size)]).join("")}</div>`).join("")}
  <div class="page-row" style="margin-top:12px">${appMap(ROBOT_ROWS, ROBOT_EXTRAS, 48)}${appMap(ROBOT_ROWS, ROBOT_EXTRAS, 32)}</div>`]);
