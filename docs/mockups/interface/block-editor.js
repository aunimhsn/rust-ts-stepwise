// The colors of the code: a small highlighter for the samples of the mock-up.
const CODE_KEYWORDS = new Set(["function", "let", "while", "repeat", "if", "else", "return", "break", "not", "and", "or"]);
const CODE_VALUES = new Set(["red", "yellow", "blue", "true", "false"]);

function highlightCode(line) {
  return line.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/(#.*$)|(&lt;|"(?:[^"\\]|\\.)*")|\b(\d+)\b|\b([a-z_][a-z0-9_]*)\b(\()?/g,
    (match, comment, text, number, word, call) => {
      if (comment) return `<span class="code-comment">${comment}</span>`;
      if (text === "&lt;") return text;
      if (text) return `<span class="code-string">${text}</span>`;
      if (number) return `<span class="code-number">${number}</span>`;
      if (CODE_KEYWORDS.has(word)) return `<span class="code-keyword">${word}</span>${call ?? ""}`;
      if (CODE_VALUES.has(word)) return `<span class="code-value">${word}</span>${call ?? ""}`;
      return call ? `<span class="code-function">${word}</span>(` : word;
    });
}

// `next` is the line of the next step, `error` the line of an error while running, `breakpoints`
// the lines with a breakpoint; `main` is the icon of the first control (Run, Pause or Resume).
// `state` and `isError` give the status bar, `steps` its counter, `cursor` its position.
function appEditor({ lines, next, error, breakpoints = [], main = "play", state, isError = false, steps, cursor }) {
  const row = (text, index) => {
    const number = index + 1;
    const mark = number === next ? ICONS.next(13)
      : number === error ? ICONS.close(13)
      : breakpoints.includes(number) ? '<span class="app-breakpoint"></span>' : "";
    const kind = number === next ? "is-next" : number === error ? "is-error" : "";
    return `<div class="app-line ${kind}"><div class="app-margin"><span class="app-mark">${mark}</span><span class="app-number">${number}</span></div><div class="app-text">${highlightCode(text)}</div></div>`;
  };
  return `<div class="app-editor">
    <div class="app-code">
      ${lines.map(row).join("")}
      <div class="app-controls">
        <button type="button" class="app-control is-main" title="Resume">${ICONS[main](20)}</button>
        <button type="button" class="app-control" title="Step">${ICONS.step(20)}</button>
        <button type="button" class="app-control" title="Reset: puts the map back. Your code stays.">${ICONS.reset(20)}</button>
      </div>
    </div>
    <div class="app-status">
      <b class="${isError ? "is-error" : ""}">${state}</b>
      <span>${steps} steps</span>
      <div class="app-speed"><span>Speed</span>${[1, 2, 3, 4, 5].map(speed => `<i class="${speed <= 3 ? "is-on" : ""}"></i>`).join("")}</div>
      <span class="app-cursor">${cursor}</span>
    </div>
  </div>`;
}
