// The manual: its page (the sections on the left, the content, On this page on the right), its search,
// and its drawer over the level screen. Every window is live: the search filters as one types, the
// arrows and Enter move in its results, On this page follows the scroll, each example plays on its map.

// Cells of 48 pixels, as on the level screen: the 5 × 3 map is as high as the longest example, 6 lines.
const MANUAL_CELL = 48;
const MANUAL_CATEGORIES = MANUAL_COMMANDS.map(({ category }) => category);
const MANUAL_DIRECTIONS = ["north", "east", "south", "west"];
const MANUAL_STEPS = { north: [0, -1], east: [1, 0], south: [0, 1], west: [-1, 0] };

// The states of the map along an example: the start, then one per step of its trace.
function manualFrames({ map, trace }) {
  const rows = map.rows.map(row => [...row]);
  let links = [...(map.links ?? [])];
  let [x, y, facing] = map.robot;
  const items = new Set(map.items ?? []);
  let [brush, eraser, carrying] = [null, false, false];
  const out = [];
  const paint = () => {
    if (brush) rows[y][x] = brush;
    if (eraser) { rows[y][x] = "."; links = links.filter(link => !link.split(" ").includes(`${x},${y}`)); }
  };
  const frame = (line, pincer = false) => ({ line, pincer, rows: rows.map(row => row.join("")), links: [...links], robot: [x, y, facing], items: [...items], carrying, out: [...out] });
  const frames = [frame(null)];
  for (const [line, action] of trace) {
    // Only the first colon separates the action from its value: a printed text may hold others.
    const [verb, value] = (action ?? "").split(/:(.*)/);
    if (verb === "forward") {
      const from = `${x},${y}`;
      x += MANUAL_STEPS[facing][0];
      y += MANUAL_STEPS[facing][1];
      if (brush) links.push(`${from} ${x},${y}`);
      paint();
    }
    if (verb === "left" || verb === "right") facing = MANUAL_DIRECTIONS[(MANUAL_DIRECTIONS.indexOf(facing) + (verb === "left" ? 3 : 1)) % 4];
    if (verb === "paint") { [brush, eraser] = [value, false]; paint(); }
    if (verb === "stop") brush = null;
    if (verb === "erase") { [brush, eraser] = [null, true]; paint(); }
    if (verb === "unerase") eraser = false;
    if (verb === "pick") { items.delete(`${x},${y}`); carrying = true; }
    if (verb === "drop") { items.add(`${x},${y}`); carrying = false; }
    if (verb === "print") out.push(value);
    frames.push(frame(line, verb === "pick" || verb === "drop"));
  }
  return frames;
}

const MANUAL_ENTRIES = Object.fromEntries(MANUAL_COMMANDS.flatMap(({ entries }) => entries).map(entry => [entry.id, entry]));
const MANUAL_FRAMES = Object.fromEntries(Object.values(MANUAL_ENTRIES).map(entry => [entry.id, manualFrames(entry)]));

function manualMap(entry, frame) {
  const extras = appMapLife(frame.rows);
  const add = (cell, mark) => { extras[cell] = { ...extras[cell], ...mark }; };
  for (const cell of entry.map.slots ?? []) add(cell, { goalItem: true });
  for (const cell of frame.items) add(cell, { item: true });
  const [x, y, facing] = frame.robot;
  add(`${x},${y}`, { robot: facing, carrying: frame.carrying, pincer: frame.pincer });
  return appMap(frame.rows, extras, MANUAL_CELL, frame.links);
}

// An example: its code, to type again rather than copy, the small map where it plays, filling the rest
// of the example, and the console, named as on the level screen, when it prints.
function manualExample(entry, step = 0) {
  const frame = MANUAL_FRAMES[entry.id][step];
  const prints = entry.lines.some(line => line.startsWith("print"));
  return `<div class="manual-example ${prints ? "has-console" : ""}" data-example="${entry.id}" data-frame="${step}">
    <div class="manual-code"><pre>${entry.lines.map((line, index) => `<span class="manual-line ${frame.line === index ? "is-on" : ""}">${highlightCode(line) || " "}</span>`).join("")}</pre><button type="button" class="app-chip manual-replay">${ICONS.reset(13)}<span>Replay</span></button></div>
    <div class="manual-map">${manualMap(entry, frame)}</div>
    ${prints ? `<div class="manual-console"><b>Console</b><div class="manual-out">${frame.out.join("<br>")}</div></div>` : ""}
  </div>`;
}

const manualEntry = entry => `<section class="manual-entry" data-entry="${entry.id}">
  <h3 class="manual-sign">${entry.names.map(name => `<code>${name}</code>`).join("")}</h3>
  <div class="manual-about">
    <p>${entry.text}</p>
    ${manualExample(entry)}
  </div>
</section>`;

const manualCategory = ({ category, text, entries }) => `<h2 data-anchor="${category}">${category}</h2>${text ? `<p>${text}</p>` : ""}${entries.map(manualEntry).join("")}`;

const MANUAL_LEAD = "The built-in functions move the robot, paint, carry an item, look around the robot and write in the console. A call always has its parentheses, even without a value: <code>forward()</code>.";

// The search field: a flat field, as the one of the codes; × clears it.
const manualSearch = query => `<label class="manual-search"><span class="manual-search-icon">${ICONS.search(15)}</span><input type="text" placeholder="Search the manual" value="${query}" spellcheck="false" autocomplete="off"><button type="button" class="manual-clear" title="Clear the search (Esc)" ${query ? "" : "hidden"}>${ICONS.close(13)}</button></label>`;

// The results of a search, grouped by their place in the manual; the match is marked in each name.
function manualResults(query, chosen = 0) {
  const words = query.trim().toLowerCase();
  const found = MANUAL_INDEX.filter(item => (item.name + " " + item.keys).toLowerCase().includes(words));
  if (!found.length) return `<div class="manual-none"><img src="world-elements/daylight/logo-disappointed.svg" height="52" alt=""><p>Nothing in the manual for “${query.trim()}”.</p><span>Try the name of a command, such as <code>forward</code>, or a word such as <i>loop</i>.</span></div>`;
  const mark = name => {
    const at = name.toLowerCase().indexOf(words);
    return at < 0 ? name : `${name.slice(0, at)}<mark>${name.slice(at, at + words.length)}</mark>${name.slice(at + words.length)}`;
  };
  const places = [...new Set(found.map(item => item.place))];
  let index = -1;
  return `<div class="manual-count">${found.length} ${found.length === 1 ? "result" : "results"}</div>${places.map(place => `<div class="manual-place">${place.replace(" › ", "<i> › </i>")}</div>${found.filter(item => item.place === place).map(item => {
    index += 1;
    return `<a class="manual-result ${item.code ? "is-code" : ""} ${index === chosen ? "is-chosen" : ""}" ${item.target ? `data-target="${item.target}"` : ""}>${mark(item.name)}</a>`;
  }).join("")}`).join("")}`;
}

// The list on the left: the ten sections, the open one with its categories, or the results of a search.
const manualList = (query, here, chosen) => query.trim() ? manualResults(query, chosen) : `<nav class="manual-sections">${MANUAL_SECTIONS.map((title, index) => index === 1
  ? `<a class="manual-section is-open">${index + 1}. ${title}</a>${MANUAL_CATEGORIES.map(category => `<a class="manual-category ${category === here ? "is-here" : ""}" data-anchor="${category}">${category}</a>`).join("")}`
  : `<a class="manual-section">${index + 1}. ${title}</a>`).join("")}</nav>`;

const manualToc = here => `<nav class="manual-toc"><b>On this page</b>${MANUAL_CATEGORIES.map(category => `<a class="${category === here ? "is-here" : ""}" data-anchor="${category}">${category}</a>`).join("")}</nav>`;

// The page of the manual, without a header: the column of the sections runs from top to bottom with
// the name at its top, the content up to the top of the window, the way back at the bottom of the
// column. `back` is the level it was opened from, in a tab of its own; null when it was opened from the
// home screen, in the same tab: the button then leads to it, named as the menu Level › All levels, and
// so does the logo.
// `query` is a search typed, `result` its result chosen with the arrows.
function manualWindow({ size = [1280, 720], back = "18. Paint a square", query = "", result = 0 } = {}) {
  return `<div class="app-window manual-window ${size[0] >= LARGE_FROM ? "is-large" : ""}" style="width:${size[0]}px;height:${size[1]}px">
    <div class="manual-body">
      <aside class="manual-side">${back ? `<div class="manual-name">${appWordmark(21)}</div>` : `<a class="manual-name is-home" title="All levels">${appWordmark(21)}</a>`}${manualSearch(query)}<div class="manual-list">${manualList(query, "Move", result)}</div><button type="button" class="manual-back"><span>${back ? `Back to ${back}` : "Back to all levels"}</span></button></aside>
      <div class="manual-content"><article class="manual-text"><h1>2. ${MANUAL_SECTIONS[1]}</h1><p class="manual-lead">${MANUAL_LEAD}</p>${MANUAL_COMMANDS.map(manualCategory).join("")}</article></div>
      ${manualToc("Move")}
      ${size[0] >= LARGE_FROM ? manualBuddy() : ""}
    </div>
  </div>`;
}

// On a large screen, the room left on the right: the head of the robot says one sentence per visit,
// drawn at random; a click on it makes it say another, an easter egg to find. Only to fill the room: it
// never moves by itself. Kind words, readable by a whole class; the sayings of the Zen of Python signed.
const MANUAL_SAYINGS = [
  ["Explicit is better than implicit.", "The Zen of Python"],
  ["You read the manual. Most people don't."],
  ["Four spaces. Not three. Not five. Four."],
  ["Three turn_left() make a turn_right(). Don't tell anyone."],
  ["Readability counts.", "The Zen of Python"],
  ["An error is just your program talking back."],
  ["No shame in pressing Step. I do it all the time."],
  ["Simple is better than complex.", "The Zen of Python"],
  ["I never bump into walls. Well, almost never."],
  ["Errors should never pass silently.", "The Zen of Python"],
];
let manualSaying = 0;
const manualSays = index => {
  const [text, source] = MANUAL_SAYINGS[index % MANUAL_SAYINGS.length];
  return `<p>${text}</p>${source ? `<small>${source}</small>` : ""}`;
};
const manualBuddy = () => `<aside class="manual-buddy"><div class="manual-bubble">${manualSays(manualSaying)}</div><button type="button" class="manual-buddy-head" title=""><img src="world-elements/daylight/logo.svg" alt=""></button></aside>`;

document.addEventListener("click", event => {
  const head = event.target.closest?.(".manual-buddy-head");
  if (!head) return;
  manualSaying += 1;
  const bubble = head.closest(".manual-buddy").querySelector(".manual-bubble");
  bubble.innerHTML = manualSays(manualSaying);
  bubble.classList.remove("is-new");
  void bubble.offsetWidth;
  bubble.classList.add("is-new");
});

// The drawer over the level screen, opened by Ctrl+click or F1 on `forward()`: the search, the place
// in the manual, the entry and the rest of its category, and Full page.
const manualDrawer = () => `<aside class="app-popup manual-drawer">
  <div class="manual-drawer-head"><b>Manual</b><button type="button" class="app-chip manual-full">Full page${ICONS.external(13)}</button><button type="button" class="manual-drawer-close" title="Close (Esc)">${ICONS.close(15)}</button></div>
  <div class="manual-drawer-search">${manualSearch("")}</div>
  <div class="manual-drawer-results" hidden></div>
  <div class="manual-drawer-scroll">
    <div class="manual-drawer-place">Manual<i> › </i>2. ${MANUAL_SECTIONS[1]}<i> › </i>Move</div>
    <article class="manual-text">${MANUAL_COMMANDS[0].entries.map(manualEntry).join("")}${manualCategory(MANUAL_COMMANDS[1])}</article>
  </div>
</aside>`;

// The hand of the mouse, Ctrl held over a name of the code.
const MANUAL_HAND = '<svg class="manual-hand" width="17" height="21" viewBox="0 0 17 21" aria-hidden="true"><path d="M5.6 1.2c.9 0 1.6.7 1.6 1.6v5.7l.5-.1c.8-.2 1.6.2 1.9.9l.5-.1c.8-.2 1.6.2 1.9.9l.4-.1c.9-.1 1.8.5 1.9 1.4l.3 3.6c.2 2.9-2 5.4-4.9 5.4H8.6c-1.3 0-2.6-.6-3.4-1.7L1.6 13.6c-.5-.7-.4-1.7.3-2.2.7-.5 1.6-.4 2.2.2L4 11.4V2.8c0-.9.7-1.6 1.6-1.6Z" fill="#ffffff" stroke="#222935" stroke-width="1.2" stroke-linejoin="round"/></svg>';

// ——— The life of the windows ———

// An example plays once, one step every 0.7 s, when it comes into view; then it stays on its last step
// and Replay plays it again. Seventeen maps moving at once would disturb the reading.
function manualDraw(example, step) {
  const entry = MANUAL_ENTRIES[example.dataset.example];
  const frame = MANUAL_FRAMES[entry.id][step];
  example.dataset.frame = step;
  example.querySelector(".manual-map").innerHTML = manualMap(entry, frame);
  example.querySelectorAll(".manual-line").forEach((line, index) => line.classList.toggle("is-on", index === frame.line));
  const out = example.querySelector(".manual-out");
  if (out) out.innerHTML = frame.out.join("<br>");
}

function manualPlay(example) {
  example.classList.remove("is-done");
  example.dataset.playing = "1";
  example.dataset.wait = 1;
  manualDraw(example, 0);
}

const manualWatch = new IntersectionObserver(seen => {
  for (const { target, isIntersecting } of seen) {
    if (isIntersecting && !target.dataset.played) { target.dataset.played = "1"; manualPlay(target); }
  }
}, { threshold: 0.7 });

setInterval(() => {
  // The page is drawn again at each change of a control: watch the new examples.
  for (const example of document.querySelectorAll(".manual-example:not([data-watched])")) {
    example.dataset.watched = "1";
    manualWatch.observe(example);
  }
  for (const example of document.querySelectorAll('.manual-example[data-playing="1"]')) {
    const wait = Number(example.dataset.wait ?? 0);
    if (wait > 0) { example.dataset.wait = wait - 1; continue; }
    const last = MANUAL_FRAMES[example.dataset.example].length - 1;
    const step = Number(example.dataset.frame) + 1;
    manualDraw(example, step);
    if (step === last) {
      example.dataset.playing = "0";
      example.classList.add("is-done");
      example.querySelectorAll(".manual-line").forEach(line => line.classList.remove("is-on"));
    }
  }
}, 700);

document.addEventListener("click", event => {
  const replay = event.target.closest?.(".manual-replay");
  if (replay) manualPlay(replay.closest(".manual-example"));
});

// The scroll of a page or of the drawer, and the place of a heading or an entry in it.
const manualScroller = node => node.closest(".manual-window")?.querySelector(".manual-content") ?? node.closest(".manual-drawer")?.querySelector(".manual-drawer-scroll");
const manualTop = (scroller, target) => target.getBoundingClientRect().top - scroller.getBoundingClientRect().top + scroller.scrollTop;

// On this page and the categories on the left: the last heading in the top third of the view is marked
// while scrolling.
document.addEventListener("scroll", event => {
  const scroller = event.target;
  if (!scroller.classList?.contains("manual-content")) return;
  const headings = [...scroller.querySelectorAll("h2[data-anchor]")];
  const atEnd = scroller.scrollTop + scroller.clientHeight >= scroller.scrollHeight - 2;
  const here = atEnd ? headings.at(-1) : headings.filter(heading => manualTop(scroller, heading) - scroller.scrollTop <= scroller.clientHeight / 3).at(-1) ?? headings[0];
  scroller.closest(".manual-window").querySelectorAll(".manual-toc a, .manual-category").forEach(link => link.classList.toggle("is-here", link.dataset.anchor === here.dataset.anchor));
}, true);

// A click on a category scrolls smoothly to its heading.
document.addEventListener("click", event => {
  const link = event.target.closest?.(".manual-toc a, .manual-category");
  if (!link) return;
  const scroller = manualScroller(link);
  scroller.scrollTo({ top: manualTop(scroller, scroller.querySelector(`h2[data-anchor="${link.dataset.anchor}"]`)) - 18, behavior: "smooth" });
});

// The search: the list follows what is typed, the arrows move in its results, Enter opens the one
// chosen, Escape or × clears it. In the drawer, the results take the place of the entry.
function manualShowSearch(field, chosen = 0) {
  const query = field.querySelector("input").value;
  field.querySelector(".manual-clear").hidden = !query;
  const drawer = field.closest(".manual-drawer");
  if (drawer) {
    drawer.querySelector(".manual-drawer-results").hidden = !query.trim();
    drawer.querySelector(".manual-drawer-scroll").hidden = Boolean(query.trim());
    drawer.querySelector(".manual-drawer-results").innerHTML = query.trim() ? manualResults(query, chosen) : "";
  } else {
    field.closest(".manual-side").querySelector(".manual-list").innerHTML = manualList(query, field.closest(".manual-window").querySelector(".manual-toc .is-here")?.dataset.anchor, chosen);
  }
}

function manualOpen(result) {
  const box = result.closest(".manual-window, .manual-drawer");
  const target = result.dataset.target && box.querySelector(`.manual-entry[data-entry="${result.dataset.target}"]`);
  if (!target) return;
  if (box.classList.contains("manual-drawer")) {
    box.querySelector(".manual-drawer-results").hidden = true;
    box.querySelector(".manual-drawer-scroll").hidden = false;
  }
  const scroller = manualScroller(target);
  scroller.scrollTo({ top: manualTop(scroller, target) - 18, behavior: "smooth" });
  target.classList.remove("is-flash");
  void target.offsetWidth;
  target.classList.add("is-flash");
}

document.addEventListener("input", event => {
  const field = event.target.closest?.(".manual-search");
  if (field) manualShowSearch(field);
});
document.addEventListener("click", event => {
  const clear = event.target.closest?.(".manual-clear");
  if (clear) {
    const field = clear.closest(".manual-search");
    field.querySelector("input").value = "";
    manualShowSearch(field);
    field.querySelector("input").focus();
    return;
  }
  const result = event.target.closest?.(".manual-result");
  if (result) manualOpen(result);
});
document.addEventListener("keydown", event => {
  const field = event.target.closest?.(".manual-search");
  if (!field) return;
  const box = field.closest(".manual-window, .manual-drawer");
  const results = [...box.querySelectorAll(".manual-result")];
  const chosen = results.findIndex(result => result.classList.contains("is-chosen"));
  if (event.key === "Escape") { field.querySelector("input").value = ""; manualShowSearch(field); }
  if ((event.key === "ArrowDown" || event.key === "ArrowUp") && results.length) {
    event.preventDefault();
    const next = (chosen + (event.key === "ArrowDown" ? 1 : results.length - 1)) % results.length;
    results.forEach((result, index) => result.classList.toggle("is-chosen", index === next));
    results[next].scrollIntoView({ block: "nearest" });
  }
  if (event.key === "Enter" && results[chosen]) manualOpen(results[chosen]);
});
