// The tabs of the page and its sections. The address keeps the choices (?theme=…).

// One group of tabs per open decision; `fallback` is the choice shown by default.
const VARIANTS = {
  theme: { label: "Style", options: Object.fromEntries(Object.entries(INTERFACE_STYLES).map(([key, style]) => [key, style.title])), fallback: Object.keys(INTERFACE_STYLES)[0] },
  // The fonts to try: one for everything, then one per part, which follows it until it is changed.
  font: { label: "All text", select: true, options: Object.fromEntries(TEXT_FONTS.map(family => [family, family])), fallback: "Manrope" },
  code: { label: "Code", select: true, options: Object.fromEntries(CODE_FONTS.map(family => [family, family])), fallback: "JetBrains Mono" },
  ...Object.fromEntries(Object.entries(FONT_ROLES).map(([key, [label, , section]]) => [key, { label, section, select: true, options: { same: "As all text", ...Object.fromEntries(TEXT_FONTS.map(family => [family, family])) }, fallback: "same" }])),
};

const params = new URLSearchParams(location.search);
const chosen = Object.fromEntries(Object.entries(VARIANTS).map(([key, variant]) =>
  [key, params.get(key) in variant.options ? params.get(key) : variant.fallback]));

// Each section script adds itself: [title, function returning its HTML]. The newest goes first.
const SECTIONS = [];

// A change of font only changes variables: the page stays as it is, with the list that was used.
function applyFonts() {
  const root = document.documentElement.style;
  root.setProperty("--font-text", fontStack(chosen.font));
  root.setProperty("--font-title", fontStack(chosen.font));
  root.setProperty("--font-code", fontStack(chosen.code, "monospace"));
  for (const [key, [, variable]] of Object.entries(FONT_ROLES)) {
    root.setProperty(variable, fontStack(chosen[key] === "same" ? chosen.font : chosen[key]));
  }
}

const tabsOf = ([key, variant]) => `${variant.section ? `<i class="page-tabs-break"></i><strong class="page-tabs-section">${variant.section}</strong>` : ""}${variant.select
  ? `<label class="group"><span>${variant.label}</span><select data-variant="${key}">${Object.entries(variant.options).map(([value, label]) =>
    `<option value="${value}" ${chosen[key] === value ? "selected" : ""}>${label}</option>`).join("")}</select></label>`
  : `<div class="group"><span>${variant.label}</span>${Object.entries(variant.options).map(([value, label]) =>
    `<button type="button" data-variant="${key}" data-value="${value}" aria-pressed="${chosen[key] === value}">${label}</button>`).join("")}</div>`}`;

function render() {
  for (const key of Object.keys(VARIANTS)) document.documentElement.dataset[key] = chosen[key];
  applyFonts();
  document.getElementById("tabs").innerHTML = Object.entries(VARIANTS).map(tabsOf).join("");
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
document.getElementById("tabs").addEventListener("change", event => {
  const { variant } = event.target.dataset;
  if (!variant) return;
  chosen[variant] = event.target.value;
  history.replaceState(null, "", "?" + new URLSearchParams(chosen) + location.hash);
  applyFonts();
});
document.getElementById("file-warning").hidden = location.protocol !== "file:";
