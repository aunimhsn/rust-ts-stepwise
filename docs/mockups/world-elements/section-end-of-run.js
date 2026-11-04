// Section: the robot at the end of a run, lit in yellow on success; on failure, the flags of the
// goals not met. The level is the one of section-goal-marks.js; the motion comes in step 3.

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
  <p>The robot lights up in yellow when every goal is met (it will hop too, in step 3); when a goal isn't met, it shakes its head and the flags appear. The tabs <b>Success</b> and <b>Failure</b> change its look at rest.</p>
  <div class="page-row">${endStages().map(({ title, painted, links, done, extras }) => `
    <div class="page-slot"><span>${title}</span>${appMap(paintedRows(painted), goalExtras(done, extras), 64, links)}<div style="height:12px"></div>${appMap(paintedRows(painted), goalExtras(done, extras), 32, links)}</div>`).join("")}
    <div class="page-slot"><span>Alone</span><div class="page-row" style="gap:8px">${appCell({ robot: "north", lit: "success" }, 96)}${appCell({ robot: "north", lit: "failure" }, 96)}</div></div>
  </div>`]);
