// The panel of the settings, over a veil, from the gear of the level screen or the link of the home
// screen. A change applies at once and is saved: no Save button. Escape, × or a click on the veil closes it.
// A click on a theme changes the whole page, as it would change the whole application.

const SETTINGS_THEMES = { "day-desk": "Day Desk", "night-desk": "Night Desk" };

// Two segments side by side, the one chosen dark (or light at night). `data` marks what they set.
const settingsSegments = (options, current, data) => `<div class="settings-segments" ${data}>${Object.entries(options).map(([key, label]) =>
  `<button type="button" data-value="${key}" aria-pressed="${key === current}">${label}</button>`).join("")}</div>`;

// Clear all progress: alone, gray at rest, since it says what it does and shouldn't call for a click. It
// turns red from the left as a loading bar while the mouse rests on it (or while it has the focus of the
// keyboard), and a click only counts once it is full: a danger that a passing click can't set off. That
// click clears at once, without a confirmation, so its name has no ellipsis.
const SETTINGS_CLEAR = '<button type="button" class="settings-clear"><span>Clear all progress</span><span class="settings-clear-fill" aria-hidden="true">Clear all progress</span></button>';

// `sounds` is the state of the setting Sounds, off by default.
function appSettings({ sounds = false } = {}) {
  return `<div class="app-veil"><div class="app-popup app-settings">
    <div class="app-shortcuts-head"><b>Settings</b><button type="button" title="Close">${ICONS.close(16)}</button></div>
    <div class="settings-row"><div><b>Theme</b><p>Only the interface changes: the map keeps its look.</p></div>${settingsSegments(SETTINGS_THEMES, chosen.theme, "data-settings-theme")}</div>
    <div class="settings-row"><b>Sounds</b>${settingsSegments({ off: "Off", on: "On" }, sounds ? "on" : "off", "data-settings-sounds")}</div>
    <div class="settings-row is-end">${SETTINGS_CLEAR}</div>
  </div></div>`;
}

// A theme chosen changes the whole page; Sounds changes in its own window only.
document.addEventListener("click", event => {
  const button = event.target.closest?.(".settings-segments button");
  if (!button) return;
  const group = button.parentElement;
  if ("settingsTheme" in group.dataset) {
    chosen.theme = button.dataset.value;
    keepInAddress();
    render();
  } else group.querySelectorAll("button").forEach(other => other.setAttribute("aria-pressed", other === button));
});

// The button of Clear all progress is ready once its bar is full; it empties at once when the mouse leaves.
document.addEventListener("transitionend", event => {
  if (event.target.classList?.contains("settings-clear-fill") && event.target.matches(".settings-clear:hover > *, .settings-clear:focus-visible > *")) event.target.parentElement.classList.add("is-ready");
});
const settingsClearReset = event => event.target.closest?.(".settings-clear:not(.is-done)")?.classList.remove("is-ready");
document.addEventListener("mouseout", event => { if (!event.relatedTarget?.closest?.(".settings-clear")) settingsClearReset(event); });
document.addEventListener("focusout", settingsClearReset);
// A click before it is full does nothing; once full, the progress is cleared and the button says so.
document.addEventListener("click", event => {
  const button = event.target.closest?.(".settings-clear.is-ready:not(.is-done)");
  if (!button) return;
  button.classList.add("is-done");
  button.innerHTML = `<span>${ICONS.check(14)} Progress cleared</span>`;
});
