// The shortcuts of the controls, open until the owner picks a proposal: the menus, the hover of the
// controls and the panel of the shortcuts all read them here.
const KEY_PROPOSALS = {
  "enter-f10": { title: "1 · Ctrl+Enter and F10", run: "Ctrl+Enter", step: "F10", reset: "Ctrl+Shift+Enter", breakpoint: "F9" },
  "function-keys": { title: "2 · Function keys", run: "F5", step: "F10", reset: "Shift+F5", breakpoint: "F9" },
  "no-function-key": { title: "3 · No function key", run: "Ctrl+Enter", step: "Ctrl+Shift+Enter", reset: "Ctrl+Shift+Backspace", breakpoint: "Ctrl+B" },
};

// The shortcuts of the rest of the screen, already decided in specs.md.
const KEYS_EDIT = { undo: "Ctrl+Z", redo: "Ctrl+Y", comment: "Ctrl+:", lineUp: "Alt+↑", lineDown: "Alt+↓", commands: "Ctrl+Space", lookUp: "F1" };

const appKeys = () => KEY_PROPOSALS[chosen.keys];
