// The tabs of the page and its sections. The address keeps the choices (?theme=…).

// One group of tabs per open decision; `fallback` is the choice shown by default.
const VARIANTS = {
  theme: { label: "Style", options: Object.fromEntries(Object.entries(INTERFACE_STYLES).map(([key, style]) => [key, style.title])), fallback: Object.keys(INTERFACE_STYLES)[0] },
  mapWord: { label: "Word Map", options: { each: "Map 1, Map 2, Map 3", once: "Map, then 1 2 3", none: "1 2 3" }, fallback: "each" },
  mapShape: { label: "Maps as", options: { pills: "Pills", tabs: "Tabs" }, fallback: "pills" },
  // How a tab shows that its map has passed: a bar in three places, or a teal background (block-map.css).
  tabBar: { label: "Map passed", options: { teal: "Teal", "teal-soft": "Pale teal", "teal-check": "Teal with a check", "teal-soft-check": "Pale teal with a check", top: "Bar on top", under: "Bar under the text", edge: "Bar on the edge" }, fallback: "teal-soft" },
  settings: { label: "Settings", options: { gear: "Gear on the right", "gear-left": "Gear after the menus", name: "Menu on the name", view: "Menu View" }, fallback: "gear-left" },
};

const params = new URLSearchParams(location.search);
const chosen = Object.fromEntries(Object.entries(VARIANTS).map(([key, variant]) =>
  [key, params.get(key) in variant.options ? params.get(key) : variant.fallback]));

// Each section script adds itself: [title, function returning its HTML]. The newest goes first.
const SECTIONS = [];

function render() {
  for (const key of Object.keys(VARIANTS)) document.documentElement.dataset[key] = chosen[key];
  document.getElementById("tabs").innerHTML = Object.entries(VARIANTS).map(([key, variant]) =>
    `<div class="group"><span>${variant.label}</span>${Object.entries(variant.options).map(([value, label]) =>
      `<button type="button" data-variant="${key}" data-value="${value}" aria-pressed="${chosen[key] === value}">${label}</button>`).join("")}</div>`).join("");
  const style = INTERFACE_STYLES[chosen.theme];
  document.getElementById("sheet").innerHTML = `
    <header class="page-header"><h1>Interface · ${style.title}</h1><p>${style.text}</p></header>
    ${SECTIONS.length > 10 ? `<nav class="page-contents" aria-label="Sections">${SECTIONS.map(([title], index) =>
      `<a href="#section-${index + 1}">${index + 1}. ${title}</a>`).join("")}</nav>` : ""}
    ${SECTIONS.map(([title, content], index) => `<section class="page-section" id="section-${index + 1}"><h2>${index + 1}. ${title}</h2>${content()}</section>`).join("")}`;
}

document.getElementById("tabs").addEventListener("click", event => {
  const { variant, value } = event.target.dataset;
  if (!variant || value === undefined) return;
  chosen[variant] = value;
  history.replaceState(null, "", "?" + new URLSearchParams(chosen) + location.hash);
  render();
});
document.getElementById("file-warning").hidden = location.protocol !== "file:";
