// The four menus of the header. Their rows follow the state of the run: "ready", "running" or "paused".
// The left end of each menu is measured under its button. Previous and Next level have no shortcut on
// purpose: a level never changes by mistake.
const MENU_LEFT = { Level: 152, Edit: 207, Run: 254, Help: 301 };

const SPEEDS = ["1 step per second", "2 steps per second", "5 steps per second", "20 steps per second", "Instant"];

function appMenuRows(name, run = "ready") {
  const locked = run !== "ready";
  const main = { ready: "Run", running: "Pause", paused: "Resume" }[run];
  return {
    Level: [["Goal"], null, ["Previous level"], ["Next level", "", true], null, ["All levels"], ["Sandbox"], null, ["Restore starting code…"]],
    Edit: [["Undo", KEYS.undo, locked], ["Redo", KEYS.redo, locked], null, ["Comment lines", KEYS.comment, locked], ["Move line up", KEYS.lineUp, locked], ["Move line down", KEYS.lineDown, locked], null, ["Commands", KEYS.commands, locked], null, ["Toggle breakpoint", KEYS.breakpoint], ["Remove all breakpoints"]],
    Run: [[main, KEYS.run], ["Step", KEYS.step, run === "running"], ["Reset", KEYS.reset], null, ["Speed", "›"]],
    Help: [["Manual"], ["Look up in the manual", KEYS.lookUp], null, ["Legend of the map"], ["Keyboard shortcuts"]],
  }[name];
}

const appMenu = (name, { run, hovered } = {}) => appDropdown(MENU_LEFT[name], appMenuRows(name, run), hovered);

// The speeds, beside the row Speed of the menu Run; `current` is the index of the speed in use.
const appSpeedMenu = (current = 2) => appDropdown(MENU_LEFT.Run + 272, SPEEDS.map((speed, index) => [speed, index === current ? "✓" : ""]), current, 147);
