// Section: a run that ends with every goal met. The whole square: 58 steps from the start, the robot
// back on its first cell, facing east. Once the last map has passed, the success panel opens (step 3d).
const WHOLE_SQUARE = [...SAMPLE_SQUARE, "2,1"];
const FOUR_PRINTS = () => consolePrints("Sides done: 1", "Sides done: 2", "Sides done: 3", "Sides done: 4");

const passedWindow = ({ header, maps, state, message, overlay }) => appWindow({
  header,
  maps,
  editor: { lines: SAMPLE_CODE(), state, tone: "passed", steps: 58, cursor: "Ln 13, Col 32" },
  map: appWorld({ painted: WHOLE_SQUARE, robot: "2,1", facing: "east", lit: "success" }),
  panels: { tab: "Console", content: FOUR_PRINTS() + consolePassed(message) },
  overlay,
});

SECTIONS.push(["A map passed, the level done", () => `<div class="page-row">
  <div class="page-slot"><span>1280 × 720, map 1 passed, two others remain: the eyes lit, its tab teal</span>${passedWindow({ maps: { shown: 1, passed: [1] }, state: "Ended · map 1 passed", message: "Map 1 passed. Try map 2." })}</div>
  <div class="page-slot"><span>1280 × 720, the last map passed, just before the success panel</span>${passedWindow({ maps: { shown: 3, passed: [1, 2, 3] }, state: "Ended · level done", message: "Map 3 passed. Level done!" })}</div>
</div>`]);
