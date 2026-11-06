// The tabs of the page and its sections. The address keeps the choices (?theme=…).

// One group of tabs per open decision; `fallback` is the choice shown by default.
const VARIANTS = {
  theme: { label: "Style", options: Object.fromEntries(Object.entries(INTERFACE_STYLES).map(([key, style]) => [key, style.title])), fallback: Object.keys(INTERFACE_STYLES)[0] },
  // The font of the console and the variables: a list, tried at once.
  consoleFont: { label: "Console font", select: true, options: Object.fromEntries(CONSOLE_FONTS.map(family => [family, family])), fallback: CONSOLE_FONTS[0] },
};

const params = new URLSearchParams(location.search);
const chosen = Object.fromEntries(Object.entries(VARIANTS).map(([key, variant]) =>
  [key, params.get(key) in variant.options ? params.get(key) : variant.fallback]));

// Each section script adds itself: [title, function returning its HTML]. The newest goes first.
const SECTIONS = [];

// A list only changes a variable: the page stays as it is.
const applyConsoleFont = () => document.documentElement.style.setProperty("--font-console", `'${chosen.consoleFont}', monospace`);

function render() {
  for (const key of Object.keys(VARIANTS)) document.documentElement.dataset[key] = chosen[key];
  applyConsoleFont();
  document.getElementById("tabs").innerHTML = Object.entries(VARIANTS).map(([key, variant]) => variant.select
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
  history.replaceState(null, "", "?" + new URLSearchParams(chosen) + location.hash);
  render();
});
document.getElementById("tabs").addEventListener("change", event => {
  const { variant } = event.target.dataset;
  if (!variant) return;
  chosen[variant] = event.target.value;
  history.replaceState(null, "", "?" + new URLSearchParams(chosen) + location.hash);
  applyConsoleFont();
});
document.getElementById("file-warning").hidden = location.protocol !== "file:";
