// A menu of the header, at `left` pixels: rows of [label, shortcut or "›"], null for a line.
const appDropdown = (left, rows, hovered) => `<div class="app-popup app-dropdown" style="left:${left}px">${rows.map((row, index) => row
  ? `<div class="app-menu-row ${index === hovered ? "is-hovered" : ""}"><span>${row[0]}</span>${row[1] === "›" ? `<i>${ICONS.chevron(12)}</i>` : `<kbd>${row[1]}</kbd>`}</div>`
  : '<div class="app-menu-line"></div>').join("")}</div>`;

const appGoal = () => `<div class="app-popup app-goal">
  <h3>Functions · 18. Paint a square</h3>
  <button type="button" class="app-close" title="Close">${ICONS.close(15)}</button>
  <p>Paint a red square of side 3, starting where the robot stands. Write one side as a function, then call it for each side.</p>
  <p class="app-weak">New here: <code>function</code>, parameters.</p>
  <div class="app-goal-links"><button type="button" class="app-chip">Show hint</button><button type="button" class="app-chip">Manual: Functions ${ICONS.external(11)}</button></div>
  <div class="app-goal-end"><button type="button" class="app-primary">Let's go</button></div>
</div>`;
