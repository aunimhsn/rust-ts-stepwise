// The shortcut of Step is open again: F10 opens the File menu of Firefox. The numbers follow the three
// first proposals of the shortcuts, so that a number always names one proposal.
const STEP_PROPOSALS = {
  f8: { title: "4 · F8", step: "F8", reset: "Ctrl+Shift+Enter" },
  enter: { title: "5 · Ctrl+Shift+Enter", step: "Ctrl+Shift+Enter", reset: "Ctrl+Shift+Backspace" },
};

// The keyboard shortcuts of the level screen: the menus, the hover of the controls and the panel of the
// shortcuts read them here.
const KEYS = {
  run: "Ctrl+Enter", breakpoint: "F9",
  get step() { return STEP_PROPOSALS[chosen.step].step; },
  get reset() { return STEP_PROPOSALS[chosen.step].reset; },
  undo: "Ctrl+Z", redo: "Ctrl+Y", comment: "Ctrl+:", lineUp: "Alt+↑", lineDown: "Alt+↓", commands: "Ctrl+Space", lookUp: "F1",
};
