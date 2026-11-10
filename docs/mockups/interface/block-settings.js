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

// Clear all progress: red, alone, since it says what it does. It fills as a loading bar while the mouse
// rests on it (or while it has the focus of the keyboard), and a click only counts once it is full: a
// danger that a passing click can't set off. The confirmation follows.
const SETTINGS_CLEAR = '<button type="button" class="settings-clear"><span>Clear all progress…</span><span class="settings-clear-fill" aria-hidden="true">Clear all progress…</span></button>';

// `sounds` is the state of the setting Sounds, off by default.
function appSettings({ sounds = false } = {}) {
  const control = chosen.settingsControls;
  const row = (title, text, input, wide = false) => `<div class="settings-row ${wide ? "is-wide" : ""}"><div><b>${title}</b><p>${text}</p></div>${input}</div>`;
  return `<div class="app-veil"><div class="app-popup app-settings">
    <div class="app-shortcuts-head"><b>Settings</b><button type="button" title="Close">${ICONS.close(16)}</button></div>
    ${row("Theme", "Only the interface changes: the map keeps its look.", settingsTheme(control), control === "cards")}
    ${row("Sounds", "A soft sound at the end of a run and when the robot bumps. Off in class.", settingsSounds(control, sounds))}
    <div class="settings-row is-end">${SETTINGS_CLEAR}</div>
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

// The button of Clear all progress is ready once its bar is full; it empties at once when the mouse leaves.
document.addEventListener("transitionend", event => {
  if (event.target.classList?.contains("settings-clear-fill") && event.target.matches(".settings-clear:hover > *, .settings-clear:focus-visible > *")) event.target.parentElement.classList.add("is-ready");
});
const settingsClearReset = event => event.target.closest?.(".settings-clear")?.classList.remove("is-ready");
document.addEventListener("mouseout", event => { if (!event.relatedTarget?.closest?.(".settings-clear")) settingsClearReset(event); });
document.addEventListener("focusout", settingsClearReset);
// A click before it is full does nothing; once full, the confirmation takes the place of the settings.
document.addEventListener("click", event => {
  const button = event.target.closest?.(".settings-clear.is-ready");
  if (button) button.closest(".app-veil").outerHTML = appConfirm({ question: "Clear all your progress?", lost: "The levels done and the code of every level will be cleared for good. Your settings stay.", action: "Clear progress" });
});
// In the mock-up, the two buttons of a confirmation close it.
document.addEventListener("click", event => {
  if (event.target.closest?.(".app-confirm .confirm-buttons button")) event.target.closest(".app-veil").remove();
});
