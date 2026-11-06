// A menu of the header, at `left` pixels: rows of [label, shortcut or "›"], null for a line.
const appDropdown = (left, rows, hovered) => `<div class="app-popup app-dropdown" style="left:${left}px">${rows.map((row, index) => row
  ? `<div class="app-menu-row ${index === hovered ? "is-hovered" : ""}"><span>${row[0]}</span>${row[1] === "›" ? `<i>${ICONS.chevron(12)}</i>` : `<kbd>${row[1]}</kbd>`}</div>`
  : '<div class="app-menu-line"></div>').join("")}</div>`;
