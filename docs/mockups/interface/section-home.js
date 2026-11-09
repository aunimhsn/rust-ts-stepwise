// Section: the home screen. Every window is live: a click on a stone chooses its level.
// The scene and the goal of a level are those of the three samples, 18, 13 and 19.

SECTIONS.push(["Home screen", () => `
  <div class="page-row">
    <div class="page-slot"><span>1280 × 720, the level to play chosen, as the screen opens: click a stone</span>${homeWindow()}</div>
    <div class="page-slot"><span>1280 × 720, a level done chosen, the title of a stone hovered</span>${homeWindow({ chosen: 13, tip: 9 })}</div>
    <div class="page-slot"><span>1280 × 720, a level locked chosen: type GREEN-OWL in the field</span>${homeWindow({ chosen: 19 })}</div>
    <div class="page-slot"><span>1280 × 720, a code that opens no level</span>${homeWindow({ chosen: 19, code: "wrong" })}</div>
    <div class="page-slot"><span>1280 × 720, a right code typed: the button lights up, click it</span>${homeWindow({ chosen: 19, code: "right" })}</div>
    <div class="page-slot"><span>1280 × 720, a learner at level 3 after the code of level 17: every level up to it is open, none is marked done</span>${homeWindow({ chosen: 3, done: 2, open: 17 })}</div>
    <div class="page-slot"><span>1280 × 720, more levels than fit: the path scrolls</span>${homeWindow({ long: true })}</div>
  </div>
  <h3>Large screen</h3>
  <div class="page-slot"><span>1920 × 1080: the path and the panel keep their size, centered in the window</span>${homeWindow({ size: [1920, 1080] })}</div>`]);
