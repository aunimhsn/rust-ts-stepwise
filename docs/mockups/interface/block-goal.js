// The bubble of the goal: the title, the chapter under it, the goal and its links on the left, what is
// new in a column on the right, and Let's go along the whole bottom. `kind` draws a bolder version,
// from hot-1 to hot-5.
const GOAL_ROBOT = '<img class="goal-robot" src="world-elements/daylight/logo.svg" alt="">';

function appGoal({ kind = "" } = {}) {
  const title = {
    "hot-2": '<div class="goal-big"><span>18</span><h3>Paint a square</h3></div>',
    "hot-3": `<div class="goal-says">${GOAL_ROBOT}<h3>18. Paint a square</h3></div>`,
    "hot-5": '<div class="goal-banner"><small>Mission 18 · Functions</small><h3>Paint a square</h3></div>',
  }[kind] ?? "<h3>18. Paint a square</h3>";
  const subtitle = kind === "hot-5" ? "" : `<p class="goal-chapter">Functions</p>`;
  const go = { "hot-3": "Let's go!", "hot-4": "Let's go ›››", "hot-5": "Start the mission ›" }[kind] ?? "Let's go";
  return `<div class="app-popup app-goal ${kind}">
    <div class="goal-columns">
      <div class="goal-main">
        ${title}
        ${subtitle}
        <p class="goal-text">Paint a red square of side 3, starting where the robot stands. Write one side as a function, then call it for each side.</p>
        <div class="goal-links"><button type="button" class="app-chip">Show hint</button><button type="button" class="app-chip">Manual: Functions ${ICONS.external(11)}</button></div>
      </div>
      <aside class="goal-new">
        <h4>New</h4>
        <span>function</span>
        <span>parameters</span>
      </aside>
    </div>
    <button type="button" class="app-close" title="Close">${ICONS.close(15)}</button>
    <button type="button" class="goal-go">${go}</button>
  </div>`;
}
