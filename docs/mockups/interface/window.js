// A window of the level screen: the header, the editor on the left, the map above the panels.
// `maps` gives the tabs of the map: the map shown, the maps that have passed and the state of the legend. `size` is the window
// in pixels: from about 1600 wide, the editor stops at about 80 characters. `sandbox` removes what
// belongs to a goal.
const LARGE_FROM = 1600;
const EDITOR_LARGEST = 768;

const appWindow = ({ header, maps = {}, editor, map, panels, overlay = "", size = [1280, 720], sandbox = false }) => {
  const [width, height] = size;
  const large = width >= LARGE_FROM;
  const style = large ? `width:${width}px;height:${height}px;--crossing:${EDITOR_LARGEST}px` : "";
  return `
  <div class="app-window ${large ? "is-large" : ""}" style="${style}">
    ${appHeader({ ...header, sandbox })}
    <div class="app-body">
      <div class="app-block is-editor">${appEditor(editor)}</div>
      <div class="app-block is-map">${appMapRow(maps.shown ?? 1, maps.passed ?? [], sandbox, maps.legend)}${map}</div>
      <div class="app-block">${appPanels(panels)}</div>
    </div>
    ${sandbox ? "" : appLevelMark(header?.place?.number)}
    ${overlay}
  </div>`;
};

// The sample program of the level: the code of the wireframes, with its map below.
const SAMPLE_CODE = (side = 3) => [
  "# Paint a square of side 3",
  "function paint_side(length):",
  "    start_painting(red)",
  "    repeat length:",
  "        forward()",
  "    stop_painting()",
  "",
  "let sides = 0",
  "while sides < 4:",
  `    paint_side(${side})`,
  "    turn_right()",
  "    sides = sides + 1",
  "    print(\"Sides done:\", sides)",
];
