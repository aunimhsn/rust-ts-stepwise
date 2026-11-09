// The right end of a row of a menu: nothing, a chevron, a check mark or a shortcut.
const appMenuEnd = end => !end ? "" : end === "›" ? `<i>${ICONS.chevron(12)}</i>` : end === "✓" ? `<i>${ICONS.check(12)}</i>` : `<kbd>${appKeyLabel(end)}</kbd>`;

// A menu, at `left` and `top` pixels: rows of [label, end, greyed], null for a line.
const appDropdown = (left, rows, hovered, top = 49) => `<div class="app-popup app-dropdown" style="left:${left}px;top:${top}px">${rows.map((row, index) => row
  ? `<div class="app-menu-row ${index === hovered ? "is-hovered" : ""} ${row[2] ? "is-off" : ""}"><span>${row[0]}</span>${appMenuEnd(row[1])}</div>`
  : '<div class="app-menu-line"></div>').join("")}</div>`;
