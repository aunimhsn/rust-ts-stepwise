// The panel of Help › Keyboard shortcuts, over a veil, or alone on a board. `look` is how its parts are
// told apart. Escape or × closes it.
function appShortcutsPanel(look, alone = false) {
  const group = (title, rows) => `<div class="app-shortcuts-group"><h4>${title}</h4>${rows.map(([label, key]) => `<div class="app-shortcuts-row"><span>${label}</span><kbd>${appKeyLabel(key)}</kbd></div>`).join("")}</div>`;
  const panel = `<div class="app-popup app-shortcuts is-look-${look}">
    <div class="app-shortcuts-head"><b>Keyboard shortcuts</b><button type="button" title="Close">${ICONS.close(16)}</button></div>
    <div class="app-shortcuts-columns">
      <div class="app-shortcuts-column">
        ${group("Run", [["Run, Pause, Resume", KEYS.run], ["Step", KEYS.step], ["Reset", KEYS.reset], ["Toggle breakpoint", KEYS.breakpoint]])}
        ${group("Manual", [["Look up the word at the cursor", KEYS.lookUp], ["Look up the word under the mouse", "Ctrl+click"]])}
      </div>
      ${group("Edit", [["Undo", KEYS.undo], ["Redo", KEYS.redo], ["Indent the lines", "Tab"], ["Unindent the lines", "Shift+Tab"], ["Comment the lines", KEYS.comment], ["Move the line up", KEYS.lineUp], ["Move the line down", KEYS.lineDown], ["Commands", KEYS.commands]])}
    </div>
  </div>`;
  return alone ? panel : `<div class="app-veil">${panel}</div>`;
}
