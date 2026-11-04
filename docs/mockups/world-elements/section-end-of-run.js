// Section: the robot at the end of a run, its eyes lit in yellow on success, dimmed on failure with
// the flags of the goals not met. The level is the one of section-goal-marks.js; the motion comes in step 3.

const endStages = () => [
  {
    title: "Every goal met",
    painted: Object.fromEntries(GOAL_BLUE.map(cell => [cell, "b"])),
    links: pathLinks(...GOAL_BLUE),
    done: { item: true },
    extras: { "1,3": { robot: "west", goalStand: true, lit: "success" } },
  },
  {
    title: "A goal not met",
    painted: Object.fromEntries(GOAL_BLUE.map(cell => [cell, "b"])),
    links: pathLinks(...GOAL_BLUE),
    done: { item: true },
    extras: { "2,3": { robot: "west", lit: "failure" }, "1,3": { goalStand: true, missed: true } },
  },
];

SECTIONS.push(["End of a run", () => `
  <p>When every goal is met, the eyes of the robot light up in yellow (it will hop too, in step 3); when a goal isn't met, they dim, it shakes its head and the flags appear.</p>
  <div class="page-row">${endStages().map(({ title, painted, links, done, extras }) => `
    <div class="page-slot"><span>${title}</span>${appMap(paintedRows(painted), goalExtras(done, extras), 64, links)}<div style="height:12px"></div>${appMap(paintedRows(painted), goalExtras(done, extras), 32, links)}</div>`).join("")}
    <div class="page-slot"><span>Alone</span><div class="page-row" style="gap:8px">${appCell({ robot: "north", lit: "success" }, 96)}${appCell({ robot: "north", lit: "failure" }, 96)}</div></div>
  </div>`]);
