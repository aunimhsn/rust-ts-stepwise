// The panel of the settings, over a veil, from the gear of the level screen or the link of the home
// screen. A change applies at once and is saved: no Save button. Escape, × or a click on the veil closes it.
// The controls follow the tab Settings controls of the page; a click on a theme changes the whole page.

// The tones of each theme, for its small preview: the header, the editor, the map block and its accent.
const SETTINGS_PREVIEWS = {
  "day-desk": { title: "Day Desk", band: "#eceff3", surface: "#fafbfc", line: "#c3cad3", map: "#d3d8df", accent: "#ffb300" },
  "night-desk": { title: "Night Desk", band: "#171b21", surface: "#1e232b", line: "#434d5c", map: "#262c35", accent: "#ffb300" },
};
const SETTINGS_THEMES = ["day-desk", "night-desk"];

// A small window of the theme: its header, three lines of code, the map with a red stroke.
const settingsPreview = key => {
  const tone = SETTINGS_PREVIEWS[key];
  return `<svg class="settings-preview" viewBox="0 0 120 68" aria-hidden="true">
    <rect width="120" height="68" fill="${tone.surface}"/>
    <rect width="120" height="11" fill="${tone.band}"/><circle cx="7" cy="5.5" r="2.4" fill="${tone.accent}"/>
    <rect x="6" y="18" width="30" height="3" rx="1.5" fill="${tone.line}"/><rect x="12" y="25" width="34" height="3" rx="1.5" fill="${tone.line}"/><rect x="12" y="32" width="22" height="3" rx="1.5" fill="${tone.line}"/>
    <rect x="60" y="11" width="60" height="57" fill="${tone.map}"/><rect x="70" y="20" width="40" height="38" fill="${FLOOR}"/>
    <path d="M78 50V28h24" fill="none" stroke="${paintShade("red")}" stroke-width="5" stroke-linejoin="round"/>
  </svg>`;
};

const settingsTheme = control => {
  const theme = chosen.theme;
  if (control === "cards") return `<div class="settings-cards">${SETTINGS_THEMES.map(key =>
    `<button type="button" class="settings-card" data-settings-theme="${key}" aria-pressed="${key === theme}">${settingsPreview(key)}<span>${SETTINGS_PREVIEWS[key].title}</span></button>`).join("")}</div>`;
  if (control === "radios") return `<div class="settings-radios">${SETTINGS_THEMES.map(key =>
    `<button type="button" class="settings-radio" data-settings-theme="${key}" aria-pressed="${key === theme}"><i></i>${SETTINGS_PREVIEWS[key].title}<span class="settings-swatch" style="background:linear-gradient(135deg, ${SETTINGS_PREVIEWS[key].band} 50%, ${SETTINGS_PREVIEWS[key].map} 50%)"></span></button>`).join("")}</div>`;
  return `<div class="settings-segments">${SETTINGS_THEMES.map(key =>
    `<button type="button" data-settings-theme="${key}" aria-pressed="${key === theme}">${SETTINGS_PREVIEWS[key].title}</button>`).join("")}</div>`;
};

const settingsSounds = (control, on) => {
  if (control === "segments") return `<div class="settings-segments" data-settings-sounds>${["Off", "On"].map(label =>
    `<button type="button" aria-pressed="${(label === "On") === on}">${label}</button>`).join("")}</div>`;
  if (control === "radios") return `<button type="button" class="settings-check" data-settings-sounds aria-pressed="${on}"><i>${ICONS.check(12)}</i>Play the sounds</button>`;
  return `<button type="button" class="settings-switch" data-settings-sounds aria-pressed="${on}" title="Sounds"><i></i></button>`;
};

// `sounds` is the state of the setting Sounds, off by default.
function appSettings({ sounds = false } = {}) {
  const control = chosen.settingsControls;
  const row = (title, text, input, wide = false) => `<div class="settings-row ${wide ? "is-wide" : ""}"><div><b>${title}</b><p>${text}</p></div>${input}</div>`;
  return `<div class="app-veil"><div class="app-popup app-settings">
    <div class="app-shortcuts-head"><b>Settings</b><button type="button" title="Close">${ICONS.close(16)}</button></div>
    ${row("Theme", "Only the interface changes: the map keeps its look.", settingsTheme(control), control === "cards")}
    ${row("Sounds", "A soft sound at the end of a run and when the robot bumps. Off in class.", settingsSounds(control, sounds))}
    ${row("Progress", "Clears the levels done and the code of every level.", '<button type="button" class="app-chip">Clear all progress…</button>')}
  </div></div>`;
}

// A theme chosen changes the whole page, as it would change the whole application.
document.addEventListener("click", event => {
  const button = event.target.closest?.("[data-settings-theme]");
  if (!button) return;
  chosen.theme = button.dataset.settingsTheme;
  keepInAddress();
  render();
});
// Sounds: a switch, a box or two segments, toggled in its own window only.
document.addEventListener("click", event => {
  const control = event.target.closest?.("[data-settings-sounds]");
  if (!control) return;
  if (control.classList.contains("settings-segments")) {
    const clicked = event.target.closest("button");
    if (clicked) control.querySelectorAll("button").forEach(button => button.setAttribute("aria-pressed", button === clicked));
  } else control.setAttribute("aria-pressed", control.getAttribute("aria-pressed") !== "true");
});
