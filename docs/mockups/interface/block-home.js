// The home screen: the path through the levels, laid out from their list, and the level chosen.

// [title, number of maps] of each level; the titles are samples until the levels are designed.
const HOME_CHAPTERS = [
  { name: "First steps", idea: "forward(), turn_left(), turn_right()", levels: [["Hello, robot", 1], ["Around the wall", 1], ["The long way", 1]] },
  { name: "Repeat", idea: "repeat and its block", levels: [["Again and again", 1], ["The corridor", 2], ["Stairs", 2]] },
  { name: "Paint", idea: "the brush and the eraser", levels: [["First stroke", 1], ["The eraser", 2]] },
  { name: "Decide", idea: "if, else if, else, the sensors", levels: [["Look ahead", 2], ["Left or right", 3], ["The fork", 2]] },
  { name: "Loop until", idea: "while, not, break", levels: [["Until the wall", 2], ["Spiral", 1]] },
  { name: "Carry", idea: "pick_up(), drop()", levels: [["Pick it up", 2], ["Delivery", 3]] },
  { name: "Remember", idea: "let, print", levels: [["Count the steps", 2], ["Keep the score", 2]] },
  { name: "Functions", idea: "function, parameters, return", levels: [["Paint a square", 3], ["Two squares", 3], ["Paint the town", 2]] },
];

// Four more chapters, to see the path once it no longer fits.
const HOME_MORE = [...HOME_CHAPTERS, ...["Go further", "Long roads", "Big maps", "Grand tour"].map(name => (
  { name, idea: "a chapter added later", levels: [["First of them", 1], ["Second of them", 2], ["Third of them", 1]] }))];

const HOME_TO_PLAY = 18;
const HOME_MAPS_PASSED = 1;
const homeStatus = number => number < HOME_TO_PLAY ? "done" : number === HOME_TO_PLAY ? "play" : "locked";

// The rows of the path: stones 105 apart, 24 more between two chapters. A chapter that doesn't fit
// goes to the next row, which starts under the end of the previous one, after a half-turn.
const HOME_STEP = 105;
const HOME_GAP = 129;
const HOME_ROW = 130;
const HOME_RANGE = [100, 800];
const HOME_FIRST_ROW = 98;

function homeLayout(chapters) {
  const stones = [];
  const bands = [];
  let [x, y, direction, number] = [HOME_RANGE[0], HOME_FIRST_ROW, 1, 0];
  chapters.forEach((chapter, index) => {
    const length = (chapter.levels.length - 1) * HOME_STEP;
    const start = x + direction * HOME_GAP;
    const end = start + direction * length;
    if (index > 0 && end >= HOME_RANGE[0] && end <= HOME_RANGE[1]) x = start;
    else if (index > 0) { y += HOME_ROW; direction = -direction; }
    const first = x;
    chapter.levels.forEach(([title, maps], place) => {
      x = first + direction * place * HOME_STEP;
      number += 1;
      stones.push({ number, title, maps, x, y, chapter: index, direction });
    });
    bands.push({ left: Math.min(first, x) - 52, right: Math.max(first, x) + 52, top: y - 82, chapter, number: index + 1 });
  });
  return { stones, bands, height: y + 46 };
}

// The stretch of the path from a stone to the next: a line, or a half-turn on the side the row was going.
const homePiece = (from, to) => from.y === to.y ? `L${to.x} ${to.y}` : `A${HOME_ROW / 2} ${HOME_ROW / 2} 0 0 ${from.direction > 0 ? 1 : 0} ${to.x} ${to.y}`;
const homeStretch = stones => stones.length < 2 ? "" : `M${stones[0].x} ${stones[0].y}${stones.slice(1).map((stone, index) => homePiece(stones[index], stone)).join("")}`;

// One small square per map, astride the bottom of the stone.
const homeSquares = ({ number, maps }) => `<span class="home-squares">${Array.from({ length: maps }, (_, index) => {
  const status = homeStatus(number);
  const passed = status === "done" || (status === "play" && index < HOME_MAPS_PASSED);
  return `<i class="${passed ? "is-passed" : status === "locked" ? "is-locked" : ""}"></i>`;
}).join("")}</span>`;

const homeStone = (stone, chosen, tip) => `<button type="button" class="home-stone is-${homeStatus(stone.number)} ${stone.number === tip ? "is-tip" : ""} ${stone.x > 640 ? "is-tip-left" : ""}" style="left:${stone.x}px;top:${stone.y}px" data-level="${stone.number}" aria-pressed="${stone.number === chosen}"><span class="home-disc"><span class="home-number">${stone.number}</span></span>${homeSquares(stone)}<span class="home-tip">${stone.number}. ${stone.title}</span></button>`;

const homeRobotPlace = stone => `left:${stone.x}px;top:${stone.y - 22}px`;

// The three scenes of the panel, the cells a little smaller than on the level screen.
const HOME_CELL = 32;
const homeFloor = (walls = []) => Array.from({ length: 7 }, (_, y) => Array.from({ length: 10 }, (_, x) => walls.includes(`${x},${y}`) ? "#" : ".").join(""));

// The spiral of the learner, from the center of the map to its border.
function homeSpiralCells() {
  const cells = ["4,3"];
  let [x, y] = [4, 3];
  for (const [dx, dy, length] of [[1, 0, 1], [0, -1, 1], [-1, 0, 2], [0, 1, 2], [1, 0, 3], [0, -1, 3], [-1, 0, 4], [0, 1, 4], [1, 0, 5], [0, -1, 5]]) {
    for (let step = 0; step < length; step += 1) { x += dx; y += dy; cells.push(`${x},${y}`); }
  }
  return cells;
}

function homeSpiral() {
  const cells = homeSpiralCells();
  const rows = homeFloor().map((row, y) => [...row].map((char, x) => cells.includes(`${x},${y}`) ? "b" : char).join(""));
  return appRowsWorld(rows, { [cells.at(-1)]: { robot: "north" } }, HOME_CELL, pathLinks(...cells));
}

// The ring of a square of side 3 cells, from its top left corner.
const homeRing = (left, top) => [[0, 0], [1, 0], [2, 0], [2, 1], [2, 2], [1, 2], [0, 2], [0, 1]].map(([x, y]) => `${left + x},${top + y}`);

function homeTwoSquares() {
  const marks = { "4,5": { robot: "east" } };
  for (const cell of homeRing(1, 1)) marks[cell] = { goalPaint: "red" };
  for (const cell of homeRing(6, 2)) marks[cell] = { goalPaint: "blue" };
  return appRowsWorld(homeFloor(["5,0", "5,1", "9,6"]), marks, HOME_CELL);
}

// Eyelids over the eyes of the robot, to blink while it waits (block-home.css).
const HOME_EYELIDS = '<svg class="home-eyelids" viewBox="0 0 64 64" aria-hidden="true"><rect x="22" y="20" width="20" height="7" fill="#222935"/></svg>';

const HOME_SCENES = {
  play: () => appWorld({ robot: SAMPLE_SQUARE[0], facing: "east", size: HOME_CELL }).replace(/(robot\.svg" alt="">)/, `$1${HOME_EYELIDS}`),
  done: homeSpiral,
  locked: homeTwoSquares,
};

const HOME_GOALS = {
  play: "Paint a red square of side 3, starting where the robot stands. Write one side as a function, then call it four times.",
  done: "Paint a spiral from the center of the map to its border, with a while loop.",
};

const homeMapChips = ({ number, maps }) => `<div class="home-maps">${Array.from({ length: maps }, (_, index) => {
  const status = homeStatus(number);
  const passed = status === "done" || (status === "play" && index < HOME_MAPS_PASSED);
  return `<span class="home-map ${passed ? "is-passed" : status === "locked" ? "is-locked" : ""}">Map ${index + 1}</span>`;
}).join("")}</div>`;

const homeCode = codeError => `<div class="home-code">
  <label for="home-code-field">Have a code?</label>
  <div class="home-code-row"><input class="${codeError ? "is-wrong" : ""}" spellcheck="false" autocomplete="off" placeholder="GREEN-OWL" value="${codeError ? "BLUE-OWL" : ""}"><button type="button" class="app-chip">Open</button></div>
  ${codeError ? '<p class="home-code-error">No level has this code. Check its two words.</p>' : "<p>A code opens its level and every level before it.</p>"}
</div>`;

const HOME_BUTTONS = {
  play: () => '<button type="button" class="goal-go home-play"><span>Play</span></button>',
  done: () => '<button type="button" class="app-chip home-go">Play again</button>',
  locked: () => '<button type="button" class="home-go is-locked" disabled>Locked</button>',
};

function homePanel(chapters, stone, codeError) {
  const status = homeStatus(stone.number);
  const toPlay = homeLayout(chapters).stones[HOME_TO_PLAY - 1];
  const text = status === "locked" ? `Finish ${toPlay.number}. ${toPlay.title} to open this level.` : HOME_GOALS[status];
  return `<aside class="home-panel">
    <p class="home-chapter">${stone.chapter + 1}. ${chapters[stone.chapter].name}</p>
    <h3>${stone.number}. ${stone.title}</h3>
    <div class="home-scene is-${status}">${HOME_SCENES[status]()}</div>
    <p class="home-goal">${text}</p>
    ${status === "locked" ? homeCode(codeError) : ""}
    <div class="home-bottom">${homeMapChips(stone)}${HOME_BUTTONS[status]()}</div>
  </aside>`;
}

// `chosen` is the level shown on the right, `tip` a stone hovered, `long` the path with more levels
// than fit, and `size` the window in pixels.
function homeWindow({ chosen: level = HOME_TO_PLAY, tip, long = false, codeError = false, size = [1280, 720] } = {}) {
  const chapters = long ? HOME_MORE : HOME_CHAPTERS;
  const { stones, bands, height } = homeLayout(chapters);
  const shown = stones[level - 1];
  const done = HOME_TO_PLAY - 1;
  return `<div class="app-window home-window done-h${chosen.homeHoverDone} play-h${chosen.homeHoverPlay}" ${long ? "data-long" : ""} style="width:${size[0]}px;height:${size[1]}px">
    <header class="app-header">
      ${appWordmark(21)}
      <span class="home-count">${done} of ${stones.length} levels done</span>
      <nav class="home-links">${["Sandbox", "Manual", "Settings"].map(item => `<button type="button" class="app-menu">${item}</button>`).join("")}</nav>
      <div class="app-progress"><i style="width:${(100 * done / stones.length).toFixed(1)}%"></i></div>
    </header>
    <div class="home-stage"><div class="home-center">
      <div class="home-scroll"><div class="home-path" style="height:${height}px">
        ${bands.map(({ left, right, top, chapter, number }) => `<div class="home-band" style="left:${left}px;top:${top}px;width:${right - left}px"><b>${number}. ${chapter.name}</b><span>${chapter.idea}</span></div>`).join("")}
        <svg class="home-line" aria-hidden="true">
          <path class="is-ghost" d="${homeStretch(stones.slice(HOME_TO_PLAY - 1))}"/>
          <path d="${homeStretch(stones.slice(0, HOME_TO_PLAY))}"/>
        </svg>
        ${stones.map(stone => homeStone(stone, level, tip)).join("")}
        <img class="home-robot" src="world-elements/daylight/robot.svg" alt="" style="${homeRobotPlace(shown)}">
      </div></div>
      ${homePanel(chapters, shown, codeError)}
    </div></div>
  </div>`;
}

// A click on a stone chooses its level: the robot goes there and the panel shows it; only Play opens it.
document.addEventListener("click", event => {
  const clicked = event.target.closest?.(".home-stone");
  if (!clicked) return;
  const screen = clicked.closest(".home-window");
  if (!screen) return;
  const chapters = "long" in screen.dataset ? HOME_MORE : HOME_CHAPTERS;
  const stone = homeLayout(chapters).stones[Number(clicked.dataset.level) - 1];
  screen.querySelectorAll(".home-stone").forEach(other => other.setAttribute("aria-pressed", other === clicked));
  screen.querySelector(".home-robot").style.cssText = homeRobotPlace(stone);
  screen.querySelector(".home-panel").outerHTML = homePanel(chapters, stone, false);
});
