// Section: a tester of the keys, because only the real browser on the real keyboard can say which key
// a page receives. It is a check of the shortcuts, not a part of the interface.
const KEY_TESTS = [
  ["Ctrl+Enter", "Address bar only"],
  ["Ctrl+Shift+Enter", "Address bar only"],
  ["F9", "Nothing listed"],
  ["F10", "Focus on the toolbar"],
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
  if (["Control", "Shift", "Alt", "Meta", "AltGraph"].includes(event.key)) return;
  const name = keyPressName(event);
  document.querySelector(".keys-last").innerHTML = `Last key: ${appKeyLabel(name)}`;
  if (!KEY_TESTS.some(([key]) => key === name)) return;
  event.preventDefault();
  keysReached.add(name);
  for (const cell of document.querySelectorAll("[data-key-test]")) {
    if (cell.dataset.keyTest === name) { cell.textContent = "Reached the page"; cell.classList.add("is-reached"); }
  }
});

SECTIONS.push(["Try the keys", () => {
  const tests = KEY_TESTS.map(([key, chrome]) => `<tr><td>${appKeyLabel(key)}</td><td>${chrome}</td><td data-key-test="${key}" class="${keysReached.has(key) ? "is-reached" : ""}">${keysReached.has(key) ? "Reached the page" : "Not tried"}</td></tr>`).join("");
  return `
    <p class="keys-intro">In ${browserName()}: click once in the box, then press each key. The page asks the browser to keep out of it. <b>Reached the page</b> means that the page got the key, not that the browser kept out: note it if the browser still does something, such as opening a menu. A key that stays <b>Not tried</b> after pressing it was taken before the page. To be done in Firefox and in Chrome, on the AZERTY keyboard.</p>
    <div class="keys-tester" tabindex="0">Click here once, then press the keys</div><span class="keys-last"></span>
    <table class="keys-table"><tr><th>Key</th><th>In Chrome (official list)</th><th>In this browser</th></tr>${tests}</table>`;
}]);
