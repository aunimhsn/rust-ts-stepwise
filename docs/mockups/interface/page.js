// The tabs of the page and its sections. The address keeps the choices (?theme=…).

// One group of tabs per open decision; `fallback` is the choice shown by default.
const VARIANTS = {
  theme: { label: "Style", options: Object.fromEntries(Object.entries(INTERFACE_STYLES).map(([key, style]) => [key, style.title])), fallback: Object.keys(INTERFACE_STYLES)[0] },
  step: { label: "Step", options: Object.fromEntries(Object.entries(STEP_PROPOSALS).map(([key, proposal]) => [key, proposal.title])), fallback: "f8" },
  more: { label: "Manual button", options: { "read-more": "6 · Read more", "read-manual": "7 · Read the manual", "see-manual": "8 · See it in the manual" }, fallback: "read-manual" },
  hints: { label: "Hints lower by", options: { "0.0": "0 px", "0.5": "0.5 px", "1.0": "1 px", "1.5": "1.5 px" }, fallback: "1.0" },
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
