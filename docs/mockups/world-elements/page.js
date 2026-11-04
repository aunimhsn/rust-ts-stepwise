// The tabs of the page and its sections. The address keeps the choices (?theme=toy&vision=…).

// One group of tabs per open decision; `fallback` is the choice shown by default.
const VARIANTS = {
  theme: { label: "Style", options: Object.fromEntries(Object.entries(THEMES).map(([key, theme]) => [key, theme.title])), fallback: "blueprint" },
  vision: { label: "Vision", options: { normal: "Normal", protanopia: "Protanopia", deuteranopia: "Deuteranopia", tritanopia: "Tritanopia" }, fallback: "normal" },
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
  const theme = THEMES[chosen.theme];
  document.getElementById("sheet").innerHTML = `
    <header class="page-header"><h1>Style ${theme.title}</h1><p>${theme.text}</p></header>
    ${SECTIONS.map(([title, content], index) => `<section class="page-section" id="section-${index + 1}"><h2>${title}</h2>${content()}</section>`).join("")}`;
}

document.getElementById("tabs").addEventListener("click", event => {
  const { variant, value } = event.target.dataset;
  if (!variant) return;
  chosen[variant] = value;
  history.replaceState(null, "", "?" + new URLSearchParams(chosen) + location.hash);
  render();
});
document.getElementById("file-warning").hidden = location.protocol !== "file:";
