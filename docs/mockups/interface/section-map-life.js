// Section: the lines every five cells and the shadows of the walls, on a map of the largest size (16 × 12),
// at the size of its cells on a 1280 × 720 screen, then larger. The sliders change them in every window.
const LIFE_ROWS = [
  "....##..........",
  "....##......#...",
  "............#...",
  "..###.......#...",
  "..#......####...",
  "..#.............",
  ".......##.......",
  ".......##.....#.",
  "..............#.",
  "####.....#....#.",
  "#.......###.....",
  "#........#......",
];
const lifeMap = size => appMap(LIFE_ROWS, { ...appMapLife(LIFE_ROWS), "5,8": { ...appMapLife(LIFE_ROWS)["5,8"], robot: "east" }, "11,2": { ...appMapLife(LIFE_ROWS)["11,2"], item: true } }, size);

SECTIONS.push(["The life of the map", () => `<div class="life-maps">
  <div class="life-map"><span>Cells of 32 px, the largest map on a 1280 × 720 screen</span>${lifeMap(32)}</div>
  <div class="life-map"><span>Cells of 48 px</span>${lifeMap(48)}</div></div>`]);
