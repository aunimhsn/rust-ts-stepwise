// The elements of the console: the lines written by print, the errors (a click puts the cursor on their
// place), and the messages of the level: a goal not met, with a flag as on the map, a map passed.
const consolePrints = (...lines) => `<div class="console-prints">${lines.map(line => `<div>${line}</div>`).join("")}</div>`;
const consoleGoal = text => `<div class="console-note is-goal"><i>${ICONS.flag(14)}</i><span>${text}</span></div>`;
// Under the errors found before running: why nothing moved on the map.
const consoleFix = count => `<div class="console-fix app-weak">Fix ${count === 1 ? "this error" : `these ${count} errors`} to run the program.</div>`;
const consolePassed = text => `<div class="console-note is-passed"><i>${ICONS.check(14)}</i><span>${text}</span></div>`;
// Keeps the last lines in sight, as a console does while a program writes.
const consoleLast = content => `<div class="console-last"><div>${content}</div></div>`;
