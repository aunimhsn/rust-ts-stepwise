// Section: the manual. Every window is live: type in the search, use the arrows and Enter, scroll the
// page or click On this page; each example plays on its small map.

// The drawer: Ctrl+click on forward(), on line 5 of the sample code; the hand of the mouse on the name.
const manualDrawerWindow = () => readyWindow({
  editor: { underlines: [{ line: 5, col: 9, length: 9, kind: "link" }] },
  overlay: `${manualDrawer()}<span class="manual-hand-place">${MANUAL_HAND}</span>`,
});

SECTIONS.push(["Manual", () => `
  <div class="page-row">
    <div class="page-slot"><span>1280 × 720, 2. Commands, opened from a level: scroll it, click On this page, type in the search</span>${manualWindow()}</div>
    <div class="page-slot"><span>1280 × 720, paint typed: the results by place, the arrows move in them, Enter opens the one chosen</span>${manualWindow({ query: "paint" })}</div>
    <div class="page-slot"><span>1280 × 720, a search that finds nothing</span>${manualWindow({ query: "pizza" })}</div>
    <div class="page-slot"><span>1280 × 720, Ctrl+click on forward(): the drawer over the map and the panels, the code still in view</span>${manualDrawerWindow()}</div>
  </div>
  <h3>Large screen</h3>
  <div class="page-slot"><span>1920 × 1080, opened from the home screen: no way back to a level; the column of text stays under about 760 pixels</span>${manualWindow({ size: [1920, 1080], back: null })}</div>`]);
