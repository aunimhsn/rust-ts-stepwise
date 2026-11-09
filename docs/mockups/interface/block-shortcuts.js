// The panel of Help › Keyboard shortcuts, over a veil. Escape or × closes it.
function appShortcutsPanel() {
  const keys = appKeys();
  const group = (title, rows) => `<div class="app-shortcuts-group"><h4>${title}</h4>${rows.map(([label, key]) => `<div class="app-shortcuts-row"><span>${label}</span><kbd>${key}</kbd></div>`).join("")}</div>`;
  return `<div class="app-veil"><div class="app-popup app-shortcuts">
    <div class="app-shortcuts-head"><b>Keyboard shortcuts</b><button type="button" title="Close">${ICONS.close(16)}</button></div>
    <div class="app-shortcuts-columns">
      <div>
        ${group("Run", [["Run, Pause, Resume", keys.run], ["Step", keys.step], ["Reset", keys.reset], ["Toggle breakpoint", keys.breakpoint]])}
        ${group("Manual", [["Look up the word at the cursor", KEYS_EDIT.lookUp], ["Look up the word under the mouse", "Ctrl+click"]])}
      </div>
      ${group("Edit", [["Undo", KEYS_EDIT.undo], ["Redo", KEYS_EDIT.redo], ["Indent the lines", "Tab"], ["Unindent the lines", "Shift+Tab"], ["Comment the lines", KEYS_EDIT.comment], ["Move the line up", KEYS_EDIT.lineUp], ["Move the line down", KEYS_EDIT.lineDown], ["Commands", KEYS_EDIT.commands]])}
    </div>
  </div></div>`;
}
