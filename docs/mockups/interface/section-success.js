// Section: the success panel, alone, then over the level that has just ended.

const SUCCESS_SAMPLE = { number: 18, level: "Paint a square", maps: "its 3 maps passed", used: ["function", "parameters", "while"], lines: 13, steps: 58, next: "19. Two squares" };
const SUCCESS_LAST = { ...SUCCESS_SAMPLE, number: 20, level: "Paint the town", used: ["function", "parameters", "while", "if"], next: undefined };
const LAST_PLACE = { chapter: "Functions", title: "Paint the town", number: 20 };

const successOver = (panel, place) => passedWindow({ header: { place }, maps: { shown: 3, passed: [1, 2, 3] }, state: "Ended · level done", message: "Map 3 passed. Level done!", overlay: `<div class="app-veil">${panel}</div>` });

SECTIONS.push(["Success panel", () => `
  <button type="button" class="page-replay" onclick="replaySuccess()">Replay the appearance</button>
  <div class="page-row">
    <div class="page-slot"><span>1280 × 720, the level done: Enter for the next level, Escape to stay</span>${successOver(appSuccess(SUCCESS_SAMPLE))}</div>
    <div class="page-slot"><span>1280 × 720, the last level of the game: no next level</span>${successOver(appSuccess(SUCCESS_LAST), LAST_PLACE)}</div>
  </div>
  <h3>The panel alone</h3>
  <div class="app-window success-board">${appSuccess(SUCCESS_SAMPLE)}</div>`]);
