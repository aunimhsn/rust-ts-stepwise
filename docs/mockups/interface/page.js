// The tabs of the page and its sections. The address keeps the choices (?theme=…).

// One group of tabs per open decision; `fallback` is the choice shown by default.
const VARIANTS = {
  theme: { label: "Style", options: Object.fromEntries(Object.entries(INTERFACE_STYLES).map(([key, style]) => [key, style.title])), fallback: Object.keys(INTERFACE_STYLES)[0] },
  // The life of the map, tuned by eye with sliders: the line every five cells, then the sun and the shadows.
  lineWidth: { label: "Line every 5 cells: width", range: { min: 1, max: 10, step: 0.5 }, fallback: "3" },
  lineStrength: { label: "darkness", range: { min: 5, max: 70, step: 5 }, unit: "%", fallback: "25" },
  sunAngle: { label: "Sun: direction", range: { min: 0, max: 345, step: 15 }, unit: "°", fallback: "315" },
  shadowLength: { label: "shadow length", range: { min: 0, max: 40, step: 1 }, fallback: "20" },
  shadowStrength: { label: "darkness", range: { min: 0, max: 60, step: 5 }, unit: "%", fallback: "20" },
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
  document.getElementById("tabs").innerHTML = Object.entries(VARIANTS).map(([key, variant]) => variant.range
    ? `<label class="group"><span>${variant.label}</span><input type="range" data-variant="${key}" min="${variant.range.min}" max="${variant.range.max}" step="${variant.range.step}" value="${chosen[key]}"><output>${chosen[key]}${variant.unit ?? ""}</output></label>`
    : `<div class="group"><span>${variant.label}</span>${Object.entries(variant.options).map(([value, label]) =>
      `<button type="button" data-variant="${key}" data-value="${value}" aria-pressed="${chosen[key] === value}">${label}</button>`).join("")}</div>`).join("");
}

function renderSheet() {
  for (const key of Object.keys(VARIANTS)) document.documentElement.dataset[key] = chosen[key];
  const style = INTERFACE_STYLES[chosen.theme];
  document.getElementById("sheet").innerHTML = `
    <header class="page-header"><h1>Interface · ${style.title}</h1><p>${style.text}</p></header>
    ${SECTIONS.length > 10 ? `<nav class="page-contents" aria-label="Sections">${SECTIONS.map(([title], index) =>
      `<a href="#section-${index + 1}">${index + 1}. ${title}</a>`).join("")}</nav>` : ""}
    ${SECTIONS.map(([title, content], index) => `<section class="page-section" id="section-${index + 1}"><h2>${index + 1}. ${title}</h2>${content()}</section>`).join("")}`;
}

function render() {
  renderTabs();
  renderSheet();
}

document.getElementById("tabs").addEventListener("click", event => {
  const { variant, value } = event.target.dataset;
  if (!variant || value === undefined) return;
  chosen[variant] = value;
  history.replaceState(null, "", "?" + new URLSearchParams(chosen) + location.hash);
  render();
});
// A slider redraws only the sheet while it moves, so that it keeps the pointer.
document.getElementById("tabs").addEventListener("input", event => {
  const { variant } = event.target.dataset;
  if (!variant) return;
  chosen[variant] = event.target.value;
  event.target.nextElementSibling.value = `${event.target.value}${VARIANTS[variant].unit ?? ""}`;
  history.replaceState(null, "", "?" + new URLSearchParams(chosen) + location.hash);
  renderSheet();
});
document.getElementById("file-warning").hidden = location.protocol !== "file:";
