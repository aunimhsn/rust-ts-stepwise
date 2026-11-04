// The tabs of the page and its sections. The address keeps the choices (?vision=…).

// One group of tabs per open decision; `fallback` is the choice shown by default.
// A group with a `range` instead of `options` is a slider, for a value tuned by eye.
const VARIANTS = {
  theme: { label: "Style", options: Object.fromEntries(Object.entries(THEMES).map(([key, theme]) => [key, theme.title])), fallback: Object.keys(THEMES)[0] },
  // The cells of a goal not met, once the program has ended.
  goalMissed: { label: "Goal not met", options: { frame: "Frame", badge: "Badge !", question: "Badge ?", flag: "Flag" }, fallback: "badge" },
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
