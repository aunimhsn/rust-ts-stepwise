// Section: the floor and the walls, on maps of different shapes: a maze with thin walls, and an
// open floor with pillars and a block, without a border (outside the grid counts as a wall).

const GROUND_MAPS = [
  {
    title: "A maze, with a border of walls",
    rows: [
      "##########",
      "#....#...#",
      "#.##.#.#.#",
      "#.#....#.#",
      "#...##...#",
      "##########",
    ],
    extras: { "1,1": { robot: "east" }, "8,4": { goalStand: true }, "6,3": { item: true } },
    links: pathLinks("1,1", "2,1", "3,1", "4,1", "4,2", "4,3"),
    paint: { "1,1": "y", "2,1": "y", "3,1": "y", "4,1": "y", "4,2": "y", "4,3": "y" },
  },
  {
    title: "An open floor, with pillars and a block",
    rows: [
      "..........",
      ".#....##..",
      "......##..",
      "..#.......",
      "........#.",
      "..........",
    ],
    extras: { "4,4": { robot: "north" }, "1,4": { goalItem: true }, "8,1": { goalPaint: "red" } },
    links: pathLinks("4,1", "4,2", "4,3", "4,4"),
    paint: { "4,1": "b", "4,2": "b", "4,3": "b", "4,4": "b" },
  },
];

const groundRows = ({ rows, paint }) => rows.map((row, y) => [...row].map((char, x) => paint[`${x},${y}`] ?? char).join(""));

SECTIONS.push(["Floor and walls", () => `
  <p>The tabs <b>Floor</b> and <b>Wall</b> change the ground. <b>Merged</b> and <b>Raised</b> join the walls into one mass, drawn by the map, with rounded outer corners; <b>Raised</b> adds a darker lip above the floor, and <b>Raised square</b> keeps it with square corners.</p>
  <div class="page-row">${GROUND_MAPS.map(map => `
    <div class="page-slot"><span>${map.title}</span>${appMap(groundRows(map), map.extras, 48, map.links)}
      <div style="height:12px"></div>${appMap(groundRows(map), map.extras, 32, map.links)}</div>`).join("")}
  </div>`]);
