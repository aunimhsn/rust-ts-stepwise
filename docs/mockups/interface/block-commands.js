// The command menu (Ctrl+Space), at the cursor of the editor: the categories, the commands, the details.
const COMMAND_CATEGORIES = ["Move", "Paint", "Carry", "Sense", "Write", "Colors", "Control", "Variables", "Functions", "Your names"];

// `category` is the category chosen; `counts` gives the results of a filter, the categories without
// result growing pale; `groups` are the rows of the middle column, [title or null, [commands]];
// `chosen` is the command chosen and `details` { name, text, example, more } what is shown beside it.
function appCommandMenu({ category, counts, groups, chosen: command, details, left = 133, top = 172, empty }) {
  const categories = COMMAND_CATEGORIES.map(name => {
    const count = counts?.[name];
    const pale = counts && !count;
    return `<div class="app-commands-category ${name === category ? "is-chosen" : ""} ${pale ? "is-pale" : ""}"><span>${name}</span>${count ? `<em>${count}</em>` : ""}</div>`;
  }).join("");
  const rows = groups.map(([title, commands]) => `${title ? `<div class="app-commands-title">${title}</div>` : ""}${commands.map(name =>
    `<div class="app-commands-row ${name === command ? "is-chosen" : ""}">${name}</div>`).join("")}`).join("");
  const side = details ? `<div class="app-commands-name">${details.name}</div>
    <p>${details.text}</p>
    <pre class="app-commands-example">${details.example.map(highlightCode).join("\n")}</pre>
    <button type="button" class="app-primary app-commands-more">Read the manual</button>` : "";
  return `<div class="app-popup app-commands" style="left:${left}px;top:${top}px">
    <div class="app-commands-columns">
      <div class="app-commands-categories">${categories}</div>
      <div class="app-commands-list">${empty ? `<div class="app-commands-empty">${empty}</div>` : rows}</div>
      <div class="app-commands-details">${side}</div>
    </div>
    <div class="app-commands-hints"><span><kbd>${appKeyLabel("↑ ↓")}</kbd> move</span><span><kbd>${appKeyLabel("← →")}</kbd> category</span><span><kbd>Tab</kbd> or <kbd>Enter</kbd> insert</span><span><kbd>Esc</kbd> close</span><span>type to search</span></div>
  </div>`;
}
