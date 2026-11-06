// A window of the level screen: the header, the editor on the left, the map above the panels.
const appWindow = ({ header, editor, map, panels, overlay = "" }) => `
  <div class="app-window">
    ${appHeader(header)}
    <div class="app-body">
      <div class="app-block is-editor">${appEditor(editor)}</div>
      <div class="app-block is-map">${map}</div>
      <div class="app-block">${appPanels(panels)}</div>
    </div>
    ${overlay}
  </div>`;

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
