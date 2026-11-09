// Section: the open decision on the shortcuts of the controls, and a tester of the keys, because only
// the real browser on the real keyboard can say which key a page receives.
const KEY_ROWS = [["Run, Pause, Resume", "run"], ["Step", "step"], ["Reset", "reset"], ["Toggle breakpoint", "breakpoint"]];
const KEY_CRITERIA = [
  ["F5 still reloads the page", [true, false, true]],
  ["Run and Reset without Fn on a laptop", [true, false, true]],
  ["Step on one key, pressed again and again", [true, true, false]],
];

// What the official list of Chrome (Windows and Linux) says of each key; the last column is measured here.
const KEY_TESTS = [
  ["Ctrl+Enter", "Address bar only"],
  ["Ctrl+Shift+Enter", "Address bar only"],
  ["Ctrl+Shift+Backspace", "Nothing listed"],
  ["F5", "Reloads the page"],
  ["Shift+F5", "Reloads, ignoring the cache"],
  ["F9", "Nothing listed"],
  ["F10", "Focus on the toolbar"],
  ["Ctrl+B", "Nothing listed"],
  ["F1", "Opens the Chrome help"],
  ["Ctrl+Y", "Nothing listed"],
  ["Ctrl+:", "Nothing listed"],
  ["Ctrl+/", "Nothing listed"],
  ["Alt+↑", "Nothing listed"],
  ["Alt+↓", "Nothing listed"],
  ["Ctrl+Space", "Nothing listed"],
];

const keysReached = new Set();
const browserName = () => /firefox/i.test(navigator.userAgent) ? "Firefox" : /chrome/i.test(navigator.userAgent) ? "Chrome" : "this browser";

// The name of a key press, as written in the menus. Shift is dropped for a punctuation, which it
// already produced: Ctrl+: on an AZERTY keyboard and Ctrl+Shift+; on a QWERTY one give the same name.
function keyPressName(event) {
  const special = { ArrowUp: "↑", ArrowDown: "↓", " ": "Space" }[event.key];
  const key = special ?? (event.key.length === 1 ? event.key.toUpperCase() : event.key);
  const punctuation = event.key.length === 1 && !/[a-z0-9 ]/i.test(event.key);
  return [event.ctrlKey && "Ctrl", event.altKey && "Alt", event.shiftKey && !punctuation && "Shift", key].filter(Boolean).join("+");
}

document.addEventListener("keydown", event => {
  if (!event.target.closest?.(".keys-tester")) return;
  const name = keyPressName(event);
  if (["Control", "Shift", "Alt", "Meta", "AltGraph"].includes(event.key)) return;
  document.querySelector(".keys-last").textContent = `Last key: ${name}`;
  if (!KEY_TESTS.some(([key]) => key === name)) return;
  event.preventDefault();
  keysReached.add(name);
  for (const cell of document.querySelectorAll("[data-key-test]")) {
    if (cell.dataset.keyTest === name) { cell.textContent = "Reached the page"; cell.classList.add("is-reached"); }
  }
});

SECTIONS.push(["Shortcuts of the controls", () => {
  const proposals = Object.entries(KEY_PROPOSALS);
  const head = `<tr><th></th>${proposals.map(([id, proposal]) => `<th class="${id === chosen.keys ? "is-chosen" : ""}">${proposal.title}</th>`).join("")}</tr>`;
  const rows = KEY_ROWS.map(([label, role]) => `<tr><td>${label}</td>${proposals.map(([id, proposal]) => `<td class="${id === chosen.keys ? "is-chosen" : ""}">${proposal[role]}</td>`).join("")}</tr>`).join("");
  const criteria = KEY_CRITERIA.map(([label, answers]) => `<tr><td>${label}</td>${answers.map(answer => `<td class="${answer ? "is-yes" : "is-no"}">${answer ? "Yes" : "No"}</td>`).join("")}</tr>`).join("");
  const tests = KEY_TESTS.map(([key, chrome]) => `<tr><td>${key}</td><td>${chrome}</td><td data-key-test="${key}" class="${keysReached.has(key) ? "is-reached" : ""}">${keysReached.has(key) ? "Reached the page" : "Not tried"}</td></tr>`).join("");
  return `
    <p class="keys-intro">Pick a proposal with the tabs <b>Shortcuts</b> at the top: the menus, the hover of the controls and the panel of the shortcuts follow it. <b>F11</b> stays the full screen of the browser, for the projector of a classroom: no proposal uses it. The shortcuts of the controls work everywhere on the level screen, even when the focus isn't in the editor.</p>
    <table class="keys-table">${head}${rows}${criteria}</table>
    <h3>Try the keys in this browser (${browserName()})</h3>
    <p class="keys-intro">Click the box, then press each key: the page keeps it for itself, it doesn't reload. A key that stays <b>Not tried</b> after pressing it was taken by the browser or the system. To be done in Firefox and in Chrome, on the AZERTY keyboard.</p>
    <div class="keys-tester" tabindex="0">Click here, then press the keys</div><span class="keys-last"></span>
    <table class="keys-table"><tr><th>Key</th><th>In Chrome (official list)</th><th>In this browser</th></tr>${tests}</table>`;
}]);
