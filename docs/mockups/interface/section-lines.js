// Section: the lines between the blocks, side by side, on the right of the paused window with the
// edge of the code. The tab Lines applies one to the windows below.
SECTIONS.push(["The lines between the blocks", () => `<div class="page-row">${Object.entries(VARIANTS.lines.options).map(([lines, title]) => `
  <div class="page-slot"><span>${title}</span><div class="lines-crop">${pausedWindow({ lines })}</div></div>`).join("")}</div>`]);
