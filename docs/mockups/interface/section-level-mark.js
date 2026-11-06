// Section: the two tests for the circle of the level to clear the title, side by side.
SECTIONS.push(["The circle of the level", () => `<div class="mark-tests">${Object.entries(VARIANTS.markFit.options).map(([markFit, title]) => `
  <div class="mark-test"><span>${title}</span><div class="mark-crop">${pausedWindow({ markFit })}</div></div>`).join("")}</div>`]);
