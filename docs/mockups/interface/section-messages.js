// Section: the messages. The banner of the storage refused (× hides it), then what replaces the screen.
SECTIONS.push(["Messages", () => `
  <h3>Storage refused</h3>
  <div class="page-row">
    <div class="page-slot"><span>1280 × 720, the home screen: the banner under the header, × hides it until the next visit</span>${homeWindow({ banner: appStorageBanner() })}</div>
    <div class="page-slot"><span>1280 × 720, the level screen: the map keeps its size, the editor and the console give their height</span>${readyWindow({ banner: appStorageBanner() })}</div>
  </div>
  <h3>In place of the screen</h3>
  <div class="page-row">
    <div class="page-slot"><span>1024 × 640, live: drag the bottom right corner. The size follows; under 800 wide it asks for a computer; from 1280 × 720 the screen comes back</span>${messageLiveWindow()}</div>
    <div class="page-slot"><span>390 × 844, a phone</span>${messageWindow("phone", [390, 844])}</div>
    <div class="page-slot"><span>1280 × 720, a browser too old: written in the page itself, shown even without JavaScript</span>${messageWindow("old", [1280, 720])}</div>
    <div class="page-slot"><span>1280 × 720, the files can't load (offline): Try again reloads the page</span>${messageWindow("offline", [1280, 720])}</div>
  </div>`]);
