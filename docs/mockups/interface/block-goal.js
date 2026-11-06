// The bubble of the goal: the robot says the title of the level, the goal and its links under it, what
// is new in a dark column on the right, and Let's go along the whole bottom.
function appGoal() {
  return `<div class="app-popup app-goal">
    <div class="goal-columns">
      <div class="goal-main">
        <div class="goal-says"><img class="goal-robot" src="world-elements/daylight/logo.svg" alt=""><h3>Paint a square</h3></div>
        <p class="goal-text">Paint a red square of side 3, starting where the robot stands. Write one side as a function, then call it for each side.</p>
        <div class="goal-links"><button type="button" class="goal-link">Show hint</button><button type="button" class="goal-link" title="Opens the manual at the section of this level">Read the manual</button></div>
      </div>
      <aside class="goal-new">
        <h4>New</h4>
        <span>function</span>
        <span>parameters</span>
      </aside>
    </div>
    <button type="button" class="app-close" title="Close">${ICONS.close(15)}</button>
    <button type="button" class="goal-go"><span>Let's go</span></button>
  </div>`;
}
