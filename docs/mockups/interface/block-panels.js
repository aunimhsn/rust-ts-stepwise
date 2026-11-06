// `tab` is the tab shown, `content` its HTML, and `dot` the dot on Variables after an error.
const appPanels = ({ tab, content, dot = false }) => `<div class="app-panels">
  <div class="app-tabs">${["Console", "Variables"].map(name =>
    `<button type="button" class="app-tab ${name === tab ? "is-chosen" : ""} ${dot && name === "Variables" ? "has-dot" : ""}">${name}</button>`).join("")}</div>
  <div class="app-panel">${content}</div>
</div>`;

const appVariables = (where, values, calls) => `
  <div class="app-values"><h4>in ${where}</h4>${values.map(([name, value]) =>
    `<div class="app-value"><span>${name}</span><span>${value}</span></div>`).join("")}</div>
  <h4>Calls</h4>${calls.map(([call, line], index) =>
    `<div class="app-call ${index === calls.length - 1 ? "is-current" : ""}">${call} · line ${line}</div>`).join("")}`;

const appConsoleError = (place, title, help) =>
  `<div class="app-message"><span class="app-place-link">${place}</span><span><b>${title}</b><br>${help}</span></div>`;
