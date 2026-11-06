// The tabs of the page and its sections. The address keeps the choices (?theme=…).

// One group of tabs per open decision; `fallback` is the choice shown by default.
const VARIANTS = {
  theme: { label: "Style", options: Object.fromEntries(Object.entries(INTERFACE_STYLES).map(([key, style]) => [key, style.title])), fallback: Object.keys(INTERFACE_STYLES)[0] },
  // The ideas tried on Floor Plan and Visor, to compare one by one on the two styles (ideas.css).
  statusBar: { label: "Status bar", options: { plain: "Plain", state: "Color of the state" }, fallback: "plain" },
  paint: { label: "Paint in the code", options: { off: "Off", on: "Underlined" }, fallback: "off" },
  progress: { label: "Progress", options: { thin: "Thin", stroke: "Stroke of paint" }, fallback: "thin" },
  currentLine: { label: "Current line", options: { band: "Band", stroke: "Stroke" }, fallback: "band" },
  mark: { label: "Its mark", options: { arrow: "Arrow", visor: "Visor" }, fallback: "arrow" },
  controls: { label: "Controls", options: { square: "Square", round: "Round", hexagon: "Hexagon" }, fallback: "square" },
  map: { label: "Map", options: { block: "On a block", wall: "In the wall" }, fallback: "block" },
  accent: { label: "Accent", options: { amber: "Amber", cyan: "Cyan" }, fallback: "amber" },
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
