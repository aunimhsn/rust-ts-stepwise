// The tabs of the page and its sections. The address keeps the choices (?theme=…).

// One control per open decision; `fallback` is the choice shown by default. Tabs by default; `select`
// is a list and `range` a slider, both setting the CSS `variable` at once, without redrawing the page.
const VARIANTS = {
  theme: { label: "Style", options: Object.fromEntries(Object.entries(INTERFACE_STYLES).map(([key, style]) => [key, style.title])), fallback: Object.keys(INTERFACE_STYLES)[0] },
};

const params = new URLSearchParams(location.search);
const accepts = (variant, value) => variant.range
  ? value !== null && Number(value) >= variant.range.min && Number(value) <= variant.range.max
  : value in variant.options;
const chosen = Object.fromEntries(Object.entries(VARIANTS).map(([key, variant]) =>
  [key, accepts(variant, params.get(key)) ? params.get(key) : variant.fallback]));

const keepInAddress = () => history.replaceState(null, "", "?" + new URLSearchParams(chosen) + location.hash);
const applyVariables = () => {
  for (const [key, variant] of Object.entries(VARIANTS)) {
    if (variant.variable) document.documentElement.style.setProperty(variant.variable, chosen[key] + (variant.unit ?? ""));
  }
};

// Each section script adds itself: [title, function returning its HTML]. The newest goes first.
const SECTIONS = [];

function render() {
  for (const key of Object.keys(VARIANTS)) document.documentElement.dataset[key] = chosen[key];
  applyVariables();
  document.getElementById("tabs").innerHTML = Object.entries(VARIANTS).map(([key, variant]) => variant.range
    ? `<label class="group"><span>${variant.label}</span><input type="range" data-variant="${key}" min="${variant.range.min}" max="${variant.range.max}" step="${variant.range.step}" value="${chosen[key]}"><output>${chosen[key]}${variant.unit ?? ""}</output></label>`
    : variant.select
    ? `<label class="group"><span>${variant.label}</span><select data-variant="${key}">${Object.entries(variant.options).map(([value, label]) =>
      `<option value="${value}" ${chosen[key] === value ? "selected" : ""}>${label}</option>`).join("")}</select></label>`
    : `<div class="group"><span>${variant.label}</span>${Object.entries(variant.options).map(([value, label]) =>
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
  keepInAddress();
  render();
});
// A list fires "input" too; so does a slider, at every step of its move.
document.getElementById("tabs").addEventListener("input", event => {
  const { variant } = event.target.dataset;
  if (!variant) return;
  chosen[variant] = event.target.value;
  if (VARIANTS[variant].range) event.target.nextElementSibling.value = chosen[variant] + (VARIANTS[variant].unit ?? "");
  keepInAddress();
  applyVariables();
});
document.getElementById("file-warning").hidden = location.protocol !== "file:";
