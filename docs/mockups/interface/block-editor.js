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

// The three controls. `main` is the first one (run, pause or resume), `stepOff` greys Step while the
// program runs, `pale` is their look while the learner types, `tip` the index of the control hovered.
// The shortcuts come from the wireframe and are decided in step 3c.
const CONTROL_MAIN = { run: ["play", "Run"], pause: ["pause", "Pause"], resume: ["play", "Resume"] };

function appControls({ main = "run", stepOff = false, pale = false, tip } = {}) {
  const [icon, name] = CONTROL_MAIN[main];
  const controls = [
    { icon, name, key: "F5", kind: "is-main" },
    { icon: "step", name: "Step", key: "F11", kind: stepOff ? "is-off" : "" },
    { icon: "reset", name: "Reset", key: "Shift+F5", kind: "", help: "Puts the map back. Your code stays." },
  ];
  return `<div class="app-controls ${pale ? "is-pale" : ""}">${controls.map((control, index) =>
    `<button type="button" class="app-control ${control.kind} ${index === tip ? "is-hovered" : ""}">${ICONS[control.icon](20)}<span class="app-tip"><b>${control.name}</b><kbd>${control.key}</kbd>${control.help ? `<small>${control.help}</small>` : ""}</span></button>`).join("")}</div>`;
}

// `next` is the line of the next step, `error` the line of an error while running, `breakpoints` the
// lines with a breakpoint. `underlines` lists { line, col, length, kind }: an error, or "long" for the
// dots past the 80th column; `hint` is the message shown on hover, { line, col, title, help }.
// `controls` gives appControls. `state` and `tone` give the status bar (paused, failed, flash, passed),
// `errors` the number of errors, `steps` its counter, `cursor` its position.
function appEditor({ lines, next, error, breakpoints = [], underlines = [], hint, controls, state, tone = "", errors, steps, cursor }) {
  const row = (text, index) => {
    const number = index + 1;
    const dot = breakpoints.includes(number) ? '<span class="app-breakpoint"></span>' : "";
    const mark = number === next ? dot + ICONS.next(13) : number === error ? ICONS.close(13) : dot;
    const kind = number === next ? "is-next" : number === error ? "is-error" : "";
    const marks = underlines.filter(underline => underline.line === number).map(underline =>
      `<span class="app-underline is-${underline.kind ?? "error"}" style="left:calc(14px + ${underline.col - 1}ch);width:${underline.length}ch"></span>`).join("");
    const note = hint?.line === number ? `<div class="app-popup app-hover-note" style="left:calc(14px + ${hint.col - 1}ch)"><b>${hint.title}</b>${hint.help ? `<br>${hint.help}` : ""}</div>` : "";
    return `<div class="app-line ${kind}"><div class="app-margin"><span class="app-mark ${dot && number === next ? "has-both" : ""}">${mark}</span><span class="app-number">${number}</span></div><div class="app-text">${highlightCode(text)}${marks}${note}</div></div>`;
  };
  return `<div class="app-editor">
    <div class="app-code">
      <div class="app-ruler"></div>
      ${lines.map(row).join("")}
      ${appControls(controls)}
    </div>
    <div class="app-status ${tone ? `is-${tone}` : ""}">
      <b>${state}</b>
      ${errors ? `<span class="app-status-errors">${errors}</span>` : ""}
      <span>${steps} steps</span>
      <div class="app-speed"><span>Speed</span>${[1, 2, 3, 4, 5].map(speed => `<i class="${speed <= 3 ? "is-on" : ""}"></i>`).join("")}</div>
      <span class="app-cursor">${cursor}</span>
    </div>
  </div>`;
}
