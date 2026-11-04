// The tabs of the page and its sections. The address keeps the choices (?paint=joined&vision=…).

// One group of tabs per open decision; `fallback` is the choice shown by default.
const VARIANTS = {
  theme: { label: "Style", options: Object.fromEntries(Object.entries(THEMES).map(([key, theme]) => [key, theme.title])), fallback: Object.keys(THEMES)[0] },
  paint: { label: "Paint", options: { cell: "Per cell", full: "Full cell", joined: "Joined", seams: "Joined + seams", stroke: "Stroke" }, fallback: "stroke" },
  // The width of the stroke, out of 64 for a cell; the robot is 42 wide with its tracks, 32 without.
  // How the color under the robot stays visible when the robot covers the stroke.
  onPaint: { label: "Robot on paint", options: { hidden: "Hidden", sensor: "Sensor", front: "Sensor, item in front", visor: "Visor", halo: "Halo" }, fallback: "visor" },
  width: { label: "Stroke width", options: { 30: "30", 34: "34", 38: "38" }, fallback: "34" },
  vision: { label: "Vision", options: { normal: "Normal", protanopia: "Protanopia", deuteranopia: "Deuteranopia", tritanopia: "Tritanopia" }, fallback: "normal" },
};

const params = new URLSearchParams(location.search);
const chosen = Object.fromEntries(Object.entries(VARIANTS).map(([key, variant]) =>
  [key, params.get(key) in variant.options ? params.get(key) : variant.fallback]));

// Each section script adds itself: [title, function returning its HTML]. The newest goes first.
const SECTIONS = [];

function render() {
  for (const key of Object.keys(VARIANTS)) document.documentElement.dataset[key] = chosen[key];
  // A group with a single option is decided: it isn't shown.
  document.getElementById("tabs").innerHTML = Object.entries(VARIANTS).filter(([, variant]) => Object.keys(variant.options).length > 1).map(([key, variant]) =>
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
