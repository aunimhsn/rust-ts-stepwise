// Section: the hover effects of the stones, side by side: two levels done, then the level to play.

const HOVER_STONES = [
  { number: 16, title: "Count the steps", maps: 2, x: 62, y: 86, direction: 1 },
  { number: 17, title: "Keep the score", maps: 2, x: 167, y: 86, direction: 1 },
  { number: 18, title: "Paint a square", maps: 3, x: 272, y: 86, direction: 1 },
];

const hoverCard = ([number, name, text]) => `<div>
  <span class="hover-name">${number} · ${name}</span>
  <span class="hover-text">${text}</span>
  <div class="app-window hover-card done-h${number} play-h${number}">
    <div class="home-path" style="width:340px;height:124px">
      <div class="home-band"></div>
      <svg class="home-line" aria-hidden="true">
        <path class="is-ghost" d="${homeStretch([HOVER_STONES[2], { x: 340, y: 86 }])}"/>
        <path d="${homeStretch([{ x: 0, y: 86 }, ...HOVER_STONES])}"/>
      </svg>
      ${HOVER_STONES.map(stone => homeStone(stone, 18)).join("")}
      <img class="home-robot" src="world-elements/daylight/robot.svg" alt="" style="${homeRobotPlace(HOVER_STONES[2])}">
    </div>
  </div>
</div>`;

SECTIONS.push(["Hover of the stones", () => HOME_HOVERS.map(([temperature, hovers]) => `
  <h3>${temperature}</h3>
  <div class="page-row">${hovers.map(hoverCard).join("")}</div>`).join("")]);
