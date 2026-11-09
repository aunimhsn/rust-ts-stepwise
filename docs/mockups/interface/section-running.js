// Section: the program running, in its third side: 37 steps from the start, two sides painted and two
// cells of the third.
const runningWindow = editor => appWindow({
  editor: { lines: SAMPLE_CODE(), next: 5, controls: { main: "pause", stepOff: true }, steps: 37, cursor: "Ln 5, Col 9", ...editor },
  map: appWorld({ painted: ["2,1", "3,1", "4,1", "5,1", "5,2", "5,3", "5,4", "4,4", "3,4"], robot: "3,4", facing: "west" }),
  panels: { tab: "Console", content: consolePrints("Sides done: 1", "Sides done: 2") },
});

SECTIONS.push(["Running", () => `<div class="page-row">
  <div class="page-slot"><span>1280 × 720, the next line highlighted, Pause first, Step greyed</span>${runningWindow({ state: "Running · Reset to edit" })}</div>
  <div class="page-slot"><span>1280 × 720, the learner tries to type: the code doesn't change, the bar lights up for a moment</span>${runningWindow({ state: "Reset the program to edit your code.", tone: "flash" })}</div>
</div>`]);
