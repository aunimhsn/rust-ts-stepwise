// Section: the scene of the level chosen, still and alive, to compare. The animations are drawn here
// with their timing as a first idea; they are tuned with the others of the map.

const SCENE_EYELIDS = '<svg class="scene-eyelids" viewBox="0 0 64 64" aria-hidden="true"><rect x="22" y="20" width="20" height="7" fill="#222935"/></svg>';

// The robot of the level to play, its eyelids over its eyes: the solution is never shown.
const waitingScene = () => HOME_SCENES.play().replace(/(robot\.svg" alt="">)/, `$1${SCENE_EYELIDS}`);

// The program of the learner replayed: the stroke grows behind the robot along the same path.
function replayScene() {
  const middle = cell => cell.split(",").map(value => (Number(value) + 0.5) * HOME_CELL).join(" ");
  const line = `M${homeSpiralCells().map(middle).join("L")}`;
  const [width, height] = [10 * HOME_CELL, 7 * HOME_CELL];
  return `${appRowsWorld(homeFloor(), {}, HOME_CELL)}
    <div class="scene-replay" style="width:${width}px;height:${height}px">
      <svg viewBox="0 0 ${width} ${height}" aria-hidden="true"><path d="${line}" pathLength="1" stroke="${paintShade("blue")}" stroke-width="${STROKE * HOME_CELL / 64}"/></svg>
      <img src="${element("robot")}" alt="" style="width:${HOME_CELL}px;height:${HOME_CELL}px;offset-path:path('${line}')">
    </div>`;
}

const sceneSlot = (label, scene, alive) => `<div class="page-slot"><span>${label}</span><div class="home-scene ${alive ? "is-alive" : ""}">${scene}</div></div>`;

SECTIONS.push(["The scene of the level chosen: still or alive", () => `
  <h3>Level to play</h3>
  <div class="page-row scene-board">
    ${sceneSlot("Still, as in the windows below", HOME_SCENES.play())}
    ${sceneSlot("Alive: the robot waits, blinks and looks around; the goal stays to be done", waitingScene(), true)}
  </div>
  <h3>Level done</h3>
  <div class="page-row scene-board">
    ${sceneSlot("Still: the final drawing", HOME_SCENES.done())}
    ${sceneSlot("Alive: the learner's own program replayed, then again", replayScene(), true)}
  </div>`]);
