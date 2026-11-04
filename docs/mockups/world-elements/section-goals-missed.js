// Section: the goals not met, marked on the map once the program has ended without meeting them.
// The map is the level of the section "Goal marks in a level" (section-goal-marks.js).

// A function, since the level comes from a script loaded after this one.
const missedStages = () => [
  {
    title: "The program stopped early",
    text: "The console says which goals aren't met: the red cell, the cells left unpainted, the item and where to stand.",
    painted: { "1,1": "b", "2,1": "b", "3,1": "b", "4,1": "r" },
    links: pathLinks("1,1", "2,1", "3,1"),
    done: {},
    extras: {
      "5,1": { robot: "south", goalPaint: "blue", missed: true },
      "4,1": { goalPaint: "blue", missed: true },
      "5,2": { goalPaint: "blue", missed: true },
      "5,3": { goalPaint: "blue", missed: true },
      "2,3": { item: true },
      "3,3": { goalItem: true, missed: true },
      "1,3": { goalStand: true, missed: true },
    },
  },
  {
    title: "Only the place where to stand is missing",
    text: "Everything is done, but the robot stopped one cell too early.",
    painted: Object.fromEntries(GOAL_BLUE.map(cell => [cell, "b"])),
    links: pathLinks(...GOAL_BLUE),
    done: { item: true },
    extras: { "2,3": { robot: "west" }, "1,3": { goalStand: true, missed: true } },
  },
];

SECTIONS.push(["Goals not met", () => `
  <p>When the program ends without meeting every goal, the cells of the goals not met are marked. The tab <b>Goal not met</b> changes the mark. Its color is a stand-in: the error color comes with the design system (step 4).</p>
  <div class="page-row">${missedStages().map(({ title, text, painted, links, done, extras }) => `
    <div class="page-slot"><span>${title}</span>${appMap(paintedRows(painted), goalExtras(done, extras), 48, links)}<span style="margin-top:6px;max-width:336px">${text}</span></div>`).join("")}
  </div>`]);
