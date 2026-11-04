// The tabs of the page and its sections. The address keeps the choices (?paint=stroke&width=34…).

// One group of tabs per open decision; `fallback` is the choice shown by default.
// A group with a `range` instead of `options` is a slider, for a value tuned by eye.
const VARIANTS = {
  theme: { label: "Style", options: Object.fromEntries(Object.entries(THEMES).map(([key, theme]) => [key, theme.title])), fallback: Object.keys(THEMES)[0] },
  paint: { label: "Paint", options: { cell: "Per cell", full: "Full cell", joined: "Joined", seams: "Joined + seams", stroke: "Stroke" }, fallback: "stroke" },
  // How the color under the robot stays visible when the robot covers the stroke.
  onPaint: { label: "Robot on paint", options: { hidden: "Hidden", sensor: "Sensor", front: "Sensor, item in front", visor: "Visor", halo: "Halo" }, fallback: "visor" },
  // The width of the stroke, out of 64 for a cell; the robot is 42 wide with its tracks, 32 without.
  width: { label: "Stroke width", range: { min: 16, max: 64, step: 1 }, fallback: "34" },
  vision: { label: "Vision", options: { normal: "Normal", protanopia: "Protanopia", deuteranopia: "Deuteranopia", tritanopia: "Tritanopia" }, fallback: "normal" },
};

const params = new URLSearchParams(location.search);
const accepts = (variant, value) => variant.range
  ? value !== null && Number(value) >= variant.range.min && Number(value) <= variant.range.max
  : value in variant.options;
const chosen = Object.fromEntries(Object.entries(VARIANTS).map(([key, variant]) =>
  [key, accepts(variant, params.get(key)) ? params.get(key) : variant.fallback]));

// Each section script adds itself: [title, function returning its HTML]. The newest goes first.
const SECTIONS = [];

function renderTabs() {
  // A group with a single option is decided: it isn't shown.
  document.getElementById("tabs").innerHTML = Object.entries(VARIANTS)
    .filter(([, variant]) => variant.range || Object.keys(variant.options).length > 1)
    .map(([key, variant]) => `<div class="group"><span>${variant.label}</span>${variant.range
      ? `<input type="range" data-variant="${key}" min="${variant.range.min}" max="${variant.range.max}" step="${variant.range.step}" value="${chosen[key]}"><output>${chosen[key]}</output>`
      : Object.entries(variant.options).map(([value, label]) =>
        `<button type="button" data-variant="${key}" data-value="${value}" aria-pressed="${chosen[key] === value}">${label}</button>`).join("")}</div>`).join("");
}

function renderSheet() {
  for (const key of Object.keys(VARIANTS)) document.documentElement.dataset[key] = chosen[key];
  const theme = THEMES[chosen.theme];
  document.getElementById("sheet").innerHTML = `
    <header class="page-header"><h1>Style ${theme.title}</h1><p>${theme.text}</p></header>
    ${SECTIONS.map(([title, content], index) => `<section class="page-section" id="section-${index + 1}"><h2>${title}</h2>${content()}</section>`).join("")}`;
}

function render() {
  renderTabs();
  renderSheet();
}

const keepInAddress = () => history.replaceState(null, "", "?" + new URLSearchParams(chosen) + location.hash);

document.getElementById("tabs").addEventListener("click", event => {
  const { variant, value } = event.target.dataset;
  if (!variant || value === undefined) return;
  chosen[variant] = value;
  keepInAddress();
  render();
});
// A slider redraws only the sheet while it moves, so that it keeps the pointer.
document.getElementById("tabs").addEventListener("input", event => {
  const { variant } = event.target.dataset;
  if (!variant) return;
  chosen[variant] = event.target.value;
  event.target.nextElementSibling.value = event.target.value;
  keepInAddress();
  renderSheet();
});
document.getElementById("file-warning").hidden = location.protocol !== "file:";
