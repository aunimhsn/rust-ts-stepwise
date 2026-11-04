// The tabs of the page and its sections. The address keeps the choices (?vision=…).

// One group of tabs per open decision; `fallback` is the choice shown by default.
// A group with a `range` instead of `options` is a slider, for a value tuned by eye.
const VARIANTS = {
  theme: { label: "Style", options: Object.fromEntries(Object.entries(THEMES).map(([key, theme]) => [key, theme.title])), fallback: Object.keys(THEMES)[0] },
  // The case of the name beside the logo.
  nameCase: { label: "Name", options: { title: "Stepwise", lower: "stepwise" }, fallback: "title" },
  // The name centered on the head of the robot: on the middle of its capitals, or of its lowercase
  // letters; the slider moves it by a fraction of a pixel, at the size of 44 px.
  nameAlign: { label: "Center on", options: { cap: "Capitals", lower: "Lowercase" }, fallback: "cap" },
  nameOffset: { label: "Offset", range: { min: -3, max: 3, step: 0.5 }, fallback: "0" },
  // The gap between the head of the robot and the name, in % of the font size.
  nameGap: { label: "Gap", range: { min: 0, max: 30, step: 1 }, fallback: "14" },
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
