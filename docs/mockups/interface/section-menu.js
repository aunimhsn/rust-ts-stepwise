// Section: a menu open. Its shortcuts come from the wireframe and are decided in step 3c.
const RUN_MENU = [["Resume", "F5"], ["Step", "F11"], ["Reset", "Shift+F5"], null, ["Speed", "›"]];

SECTIONS.push(["Menu open", () => `<div class="page-row"><div class="page-slot"><span>1280 × 720, the menu Run</span>${pausedWindow({ header: { menu: "Run" }, overlay: appDropdown(255, RUN_MENU, 1) })}</div></div>`]);
