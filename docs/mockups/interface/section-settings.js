// Section: the settings. Every window is live: a theme clicked changes the whole page, Sounds toggles.
SECTIONS.push(["Settings", () => `<div class="page-row">
  <div class="page-slot"><span>1280 × 720, the gear of the level screen: the settings as they are at first, Sounds off</span>${readyWindow({ header: { settings: true }, overlay: appSettings() })}</div>
  <div class="page-slot"><span>1280 × 720, Settings on the home screen, Sounds on</span>${homeWindow({ settings: true, overlay: appSettings({ sounds: true }) })}</div>
</div>`]);
