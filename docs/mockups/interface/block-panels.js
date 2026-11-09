// `tab` is the tab shown, `content` its HTML, and `dot` the dot on Variables after an error. On a large
// window, `side` gives the HTML of both panels: { console, variables }, side by side without tabs.
const appPanels = ({ tab, content, dot = false, side }) => side
  ? `<div class="app-panels is-side">${[["Console", side.console], ["Variables", side.variables]].map(([name, html]) =>
    `<div class="app-side"><div class="app-side-title">${name}</div><div class="app-panel">${html}</div></div>`).join("")}</div>`
  : `<div class="app-panels">
  <div class="app-tabs">${["Console", "Variables"].map(name =>
    `<button type="button" class="app-tab ${name === tab ? "is-chosen" : ""} ${dot && name === "Variables" ? "has-dot" : ""}">${name}</button>`).join("")}</div>
  <div class="app-panel">${content}</div>
</div>`;

// `values` lists [name, value, changed by the last step]; `calls` lists [call, line], the last one
// running; `chosen` is the index of the call shown, by default the one running.
const appVariables = (where, values, calls, chosen = calls.length - 1) => `
  <div class="app-values"><h4>in ${where}</h4>${values.map(([name, value, changed]) =>
    `<div class="app-value ${changed ? "is-changed" : ""}"><span>${name}</span><span>${value}</span></div>`).join("")}</div>
  <h4>Calls</h4>${calls.map(([call, line], index) =>
    `<div class="app-call ${index === chosen ? "is-current" : ""}">${call} · Ln ${line}</div>`).join("")}`;

const appConsoleError = (place, title, help) =>
  `<div class="app-message"><span class="app-place-link">${place}</span><span><b>${title}</b><br>${help}</span></div>`;
