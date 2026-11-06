// The tabs of the page and its sections. The address keeps the choices (?theme=…).

// One group of tabs per open decision; `fallback` is the choice shown by default.
const VARIANTS = {
  theme: { label: "Style", options: Object.fromEntries(Object.entries(INTERFACE_STYLES).map(([key, style]) => [key, style.title])), fallback: Object.keys(INTERFACE_STYLES)[0] },
  // The place of the level: on one bar (1 to 8), on two bars (9 to 13), then the combinations of the
  // two lines (2) and the map tabs (11), on one bar (14 to 17) and on two bars (18 and 19).
  place: { label: "Place", options: { title: "1 Title", stacked: "2 Two lines", field: "3 Field", stone: "4 Stone", path: "5 Path", tag: "6 Tag", robot: "7 Robot", crumb: "8 Breadcrumb", "bar-plain": "9 Second bar", "bar-columns": "10 Over the columns", "bar-tabs": "11 Map tabs", "bar-goal": "12 With the goal", "bar-levels": "13 All the levels", "mix-right": "14 Tabs on the right", "mix-split": "15 Split at the column", "mix-left": "16 Title after the menus", "mix-block": "17 Tabs in the map block", "bar-mix-center": "18 Bar of tabs only", "bar-mix-left": "19 Second bar with both" }, fallback: "mix-block" },
  // The texts of the bar around their center: tuned by eye on the owner's screen.
  menuOffset: { label: "Menus", options: { up: "-0.5 px", center: "0", down: "+0.5 px", more: "+1 px" }, fallback: "center" },
  barOffset: { label: "Place and maps", options: { up: "-0.5 px", center: "0", down: "+0.5 px", more: "+1 px" }, fallback: "center" },
  mapLabel: { label: "Map label", options: { text: "Text font", name: "Font of the name", all: "With its numbers" }, fallback: "text" },
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
