// The content of the manual drawn in the mock-up: its ten sections, and the section Commands in full,
// from the built-in functions of specs.md. Each entry has an example, played on a small map by
// block-manual.js: `trace` lists the steps of the example, [index of its line, action or null].
// The actions: forward, left, right, paint:<r|y|b>, stop, erase, unerase, pick, drop, print:<text>.

const MANUAL_SECTIONS = ["Your first program", "Commands", "Indentation and blocks", "Conditions", "Loops", "Variables", "Functions", "Conventions", "Errors", "The map"];

const MANUAL_FLOOR = [".....", ".....", "....."];
const MANUAL_RED_ROW = [".....", "rrrrr", "....."];
const MANUAL_RED_LINKS = pathLinks("0,1", "1,1", "2,1", "3,1", "4,1");
const MANUAL_REPEAT = (times, steps) => Array.from({ length: times }, () => steps).flat();

const MANUAL_COMMANDS = [
  { category: "Move", entries: [
    { id: "forward", names: ["forward()"], text: "Moves the robot one cell forward.",
      lines: ["forward()", "forward()", "turn_left()", "forward()"],
      map: { rows: MANUAL_FLOOR, robot: [0, 2, "east"] },
      trace: [[0, "forward"], [1, "forward"], [2, "left"], [3, "forward"]] },
    { id: "turn_left", names: ["turn_left()"], text: "Turns the robot a quarter turn to the left, on its cell.",
      lines: ["turn_left()", "forward()", "turn_left()", "forward()"],
      map: { rows: MANUAL_FLOOR, robot: [3, 2, "east"] },
      trace: [[0, "left"], [1, "forward"], [2, "left"], [3, "forward"]] },
    { id: "turn_right", names: ["turn_right()"], text: "Turns the robot a quarter turn to the right, on its cell.",
      lines: ["repeat 4:", "    forward()", "    turn_right()"],
      map: { rows: MANUAL_FLOOR, robot: [1, 0, "east"] },
      trace: MANUAL_REPEAT(4, [[0, null], [1, "forward"], [2, "right"]]) },
  ] },
  { category: "Paint", text: "The robot holds a brush and an eraser. Once down, a tool works on every cell the robot moves into, until it is lifted; putting one down lifts the other.", entries: [
    { id: "start_painting", names: ["start_painting(color)"], text: "Puts the brush down with the color: paints the cell of the robot at once, then every cell it moves into, over any paint already there. Called while painting, it changes the color.",
      lines: ["start_painting(blue)", "forward()", "forward()", "start_painting(red)", "forward()"],
      map: { rows: MANUAL_FLOOR, robot: [0, 1, "east"] },
      trace: [[0, "paint:b"], [1, "forward"], [2, "forward"], [3, "paint:r"], [4, "forward"]] },
    { id: "stop_painting", names: ["stop_painting()"], text: "Lifts the brush: the robot moves on without painting.",
      lines: ["start_painting(yellow)", "forward()", "stop_painting()", "forward()", "forward()"],
      map: { rows: MANUAL_FLOOR, robot: [0, 1, "east"] },
      trace: [[0, "paint:y"], [1, "forward"], [2, "stop"], [3, "forward"], [4, "forward"]] },
    { id: "start_erasing", names: ["start_erasing()"], text: "Puts the eraser down: removes the paint of the cell of the robot at once, then of every cell it moves into.",
      lines: ["forward()", "start_erasing()", "forward()", "forward()"],
      map: { rows: MANUAL_RED_ROW, links: MANUAL_RED_LINKS, robot: [0, 1, "east"] },
      trace: [[0, "forward"], [1, "erase"], [2, "forward"], [3, "forward"]] },
    { id: "stop_erasing", names: ["stop_erasing()"], text: "Lifts the eraser: the robot moves on without erasing.",
      lines: ["start_erasing()", "forward()", "stop_erasing()", "forward()"],
      map: { rows: MANUAL_RED_ROW, links: MANUAL_RED_LINKS, robot: [0, 1, "east"] },
      trace: [[0, "erase"], [1, "forward"], [2, "unerase"], [3, "forward"]] },
  ] },
  { category: "Carry", text: "The robot carries one item at a time, in the bay at its back.", entries: [
    { id: "pick_up", names: ["pick_up()"], text: "Picks up the item of the cell of the robot.",
      lines: ["forward()", "forward()", "pick_up()", "forward()"],
      map: { rows: MANUAL_FLOOR, robot: [0, 1, "east"], items: ["2,1"] },
      trace: [[0, "forward"], [1, "forward"], [2, "pick"], [3, "forward"]] },
    { id: "drop", names: ["drop()"], text: "Drops the item the robot carries on its cell.",
      lines: ["pick_up()", "forward()", "forward()", "forward()", "drop()", "forward()"],
      map: { rows: MANUAL_FLOOR, robot: [0, 1, "east"], items: ["0,1"], slots: ["3,1"] },
      trace: [[0, "pick"], [1, "forward"], [2, "forward"], [3, "forward"], [4, "drop"], [5, "forward"]] },
  ] },
  { category: "Sense", text: "The sensors look around the robot without moving it. Each one gives <code>true</code> or <code>false</code>, for an <code>if</code> or a <code>while</code>.", entries: [
    { id: "is_clear", names: ["front_is_clear()", "left_is_clear()", "right_is_clear()"], text: "<code>true</code> when the cell on that side of the robot is a floor cell; <code>false</code> for a wall or the edge of the map.",
      lines: ["while front_is_clear():", "    forward()"],
      map: { rows: [".....", "...#.", "....."], robot: [0, 1, "east"] },
      trace: [[0, null], [1, "forward"], [0, null], [1, "forward"], [0, null]] },
    { id: "here_is", names: ["here_is(color)"], text: "<code>true</code> when the cell of the robot has that paint.",
      lines: ["while not here_is(red):", "    forward()"],
      map: { rows: ["...r.", "...r.", "...r."], robot: [0, 1, "east"] },
      trace: [[0, null], [1, "forward"], [0, null], [1, "forward"], [0, null], [1, "forward"], [0, null]] },
    { id: "front_is", names: ["front_is(color)", "left_is(color)", "right_is(color)"], text: "<code>true</code> when the cell on that side of the robot has that paint; <code>false</code> for a wall.",
      lines: ["while not front_is(blue):", "    forward()", "turn_left()"],
      map: { rows: [".....", "....b", "....."], robot: [0, 1, "east"] },
      trace: [[0, null], [1, "forward"], [0, null], [1, "forward"], [0, null], [1, "forward"], [0, null], [2, "left"]] },
    { id: "item_here", names: ["item_here()", "item_in_front()"], text: "<code>true</code> when the cell of the robot, or the cell in front of it, has an item.",
      lines: ["while not item_in_front():", "    forward()", "forward()", "pick_up()"],
      map: { rows: MANUAL_FLOOR, robot: [0, 1, "east"], items: ["3,1"] },
      trace: [[0, null], [1, "forward"], [0, null], [1, "forward"], [0, null], [2, "forward"], [3, "pick"]] },
    { id: "is_carrying", names: ["is_carrying()"], text: "<code>true</code> when the robot carries an item.",
      lines: ["forward()", "if not is_carrying():", "    pick_up()", "forward()"],
      map: { rows: MANUAL_FLOOR, robot: [0, 1, "east"], items: ["1,1"] },
      trace: [[0, "forward"], [1, null], [2, "pick"], [3, "forward"]] },
    { id: "is_painting", names: ["is_painting()", "is_erasing()"], text: "<code>true</code> when the brush, or the eraser, is down.",
      lines: ["start_painting(red)", "forward()", "if is_painting():", "    stop_painting()", "forward()"],
      map: { rows: MANUAL_FLOOR, robot: [0, 1, "east"] },
      trace: [[0, "paint:r"], [1, "forward"], [2, null], [3, "stop"], [4, "forward"]] },
  ] },
  { category: "Write", entries: [
    { id: "print", names: ["print(value, …)"], text: "Writes its values in the console, separated by spaces, on one line.",
      lines: ["let steps = 0", "repeat 3:", "    forward()", "    steps = steps + 1", 'print("Steps:", steps)'],
      map: { rows: MANUAL_FLOOR, robot: [0, 1, "east"] },
      trace: [[0, null], ...MANUAL_REPEAT(3, [[1, null], [2, "forward"], [3, null]]), [4, "print:Steps: 3"]] },
  ] },
  { category: "Colors", text: "The three paints of the world.", entries: [
    { id: "colors", names: ["red", "yellow", "blue"], keys: "paint color", text: "A color is a value, written without quotes. Give it to <code>start_painting()</code>, <code>here_is()</code> and the other sensors of color.",
      lines: ["start_painting(red)", "forward()", "start_painting(yellow)", "forward()", "start_painting(blue)", "forward()"],
      map: { rows: MANUAL_FLOOR, robot: [0, 1, "east"] },
      trace: [[0, "paint:r"], [1, "forward"], [2, "paint:y"], [3, "forward"], [4, "paint:b"], [5, "forward"]] },
  ] },
];

// What the search looks into: the commands, and the headings of the other sections (not drawn in the
// mock-up). `target` is the entry a result opens, when it is drawn.
const MANUAL_HEADINGS = {
  1: ["One instruction per line", "From top to bottom"],
  3: ["The colon", "Four spaces"],
  4: ["if", "else if", "else", "Comparisons", "and, or, not"],
  5: ["repeat", "while", "break", "A program that never ends"],
  6: ["let", "Changing a value", "Where a variable exists"],
  7: ["function", "Parameters", "return", "Define before calling"],
  8: ["snake_case names", "Clear names", "One idea per function", "Comments", "Checked, or good practice"],
  9: ["Reading an error", "The most common errors"],
  10: ["Floor", "Wall", "Paints", "Painted passages", "Item", "Goal marks", "Goal not met", "Robot"],
};
const MANUAL_CODE_HEADINGS = new Set(["if", "else if", "else", "repeat", "while", "break", "let", "function", "return"]);

const MANUAL_INDEX = [
  ...MANUAL_COMMANDS.flatMap(({ category, entries }) => entries.flatMap(entry =>
    (entry.id === "colors" ? [entry.names.join(", ")] : entry.names).map(name =>
      ({ place: `2. ${MANUAL_SECTIONS[1]} › ${category}`, name, code: true, keys: entry.keys ?? "", target: entry.id })))),
  ...Object.entries(MANUAL_HEADINGS).flatMap(([section, headings]) => headings.map(name =>
    ({ place: `${section}. ${MANUAL_SECTIONS[section - 1]}`, name, code: MANUAL_CODE_HEADINGS.has(name), keys: "" }))),
];
