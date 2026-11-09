// The four menus of the header. Their rows follow the state of the run: "ready", "running" or "paused".
// The left end of each menu is measured under its button. Previous and Next level have no shortcut on
// purpose: a level never changes by mistake.
const MENU_LEFT = { Level: 152, Edit: 207, Run: 254, Help: 301 };

const SPEEDS = ["1 step per second", "2 steps per second", "5 steps per second", "20 steps per second", "Instant"];

function appMenuRows(name, run = "ready") {
  const keys = appKeys();
  const locked = run !== "ready";
  const main = { ready: "Run", running: "Pause", paused: "Resume" }[run];
  return {
    Level: [["Goal"], null, ["Previous level"], ["Next level", "", true], null, ["All levels"], ["Sandbox"], null, ["Restore starting code…"]],
    Edit: [["Undo", KEYS_EDIT.undo, locked], ["Redo", KEYS_EDIT.redo, locked], null, ["Comment lines", KEYS_EDIT.comment, locked], ["Move line up", KEYS_EDIT.lineUp, locked], ["Move line down", KEYS_EDIT.lineDown, locked], null, ["Commands", KEYS_EDIT.commands, locked], null, ["Toggle breakpoint", keys.breakpoint], ["Remove all breakpoints"]],
    Run: [[main, keys.run], ["Step", keys.step, run === "running"], ["Reset", keys.reset], null, ["Speed", "›"]],
    Help: [["Manual"], ["Look up in the manual", KEYS_EDIT.lookUp], null, ["Legend of the map"], ["Keyboard shortcuts"]],
  }[name];
}

const appMenu = (name, { run, hovered } = {}) => appDropdown(MENU_LEFT[name], appMenuRows(name, run), hovered);

// The speeds, beside the row Speed of the menu Run; `current` is the index of the speed in use.
const appSpeedMenu = (current = 2) => appDropdown(MENU_LEFT.Run + 272, SPEEDS.map((speed, index) => [speed, index === current ? "✓" : ""]), current, 147);
