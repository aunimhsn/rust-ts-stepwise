// Section: the panel of Help › Keyboard shortcuts in its window, then its four looks side by side.
SECTIONS.push(["Keyboard shortcuts panel", () => `
  <div class="page-row"><div class="page-slot"><span>1280 × 720, opened from Help › Keyboard shortcuts</span>${readyWindow({ overlay: appShortcutsPanel(chosen.panel) })}</div></div>
  <h3>Four looks</h3>
  <div class="shortcuts-boards">${Object.entries(VARIANTS.panel.options).map(([look, title]) => `<div><span class="controls-board-title">${title}</span><div class="app-window shortcuts-board">${appShortcutsPanel(look, true)}</div></div>`).join("")}</div>`]);
