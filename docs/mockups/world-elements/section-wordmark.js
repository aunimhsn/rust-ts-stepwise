// Section: the name beside the logo, "stepwise" in Fredoka 600, centered on the head of the robot.

const NAME_FONT = "600 100px Fredoka, sans-serif";
// The head of the robot fills the height of logo.svg. Tuned by eye with the owner, in fractions of the
// font size: the middle of the head sits a little above the middle of the lowercase letters, and the
// name starts a little after the tip of the arrow.
const NAME_OFFSET = 0.5 / 44;
const NAME_GAP = 0.18;

// The height of a letter of the name, out of its font size.
function letterHeight(letter) {
  const context = document.createElement("canvas").getContext("2d");
  context.font = NAME_FONT;
  return context.measureText(letter).actualBoundingBoxAscent / 100;
}

// The name at a font size: the logo is sized so that its head is a quarter taller than the
// capitals, then raised so that the middle of its head meets the middle of the lowercase letters.
function appWordmark(fontSize, ink) {
  const height = letterHeight("S") * fontSize * 1.25;
  const lift = letterHeight("x") * fontSize / 2 - height / 2 + NAME_OFFSET * fontSize;
  return `<div style="color:${ink};font:600 ${fontSize}px/1.4 Fredoka, sans-serif;white-space:nowrap">
    <img src="${logoFile()}" height="${height.toFixed(1)}" alt="" style="vertical-align:${lift.toFixed(2)}px;margin-right:${(fontSize * NAME_GAP).toFixed(1)}px">stepwise</div>`;
}

SECTIONS.push(["Name", () => `
  <p>The name, in lowercase, in Fredoka 600 (SIL Open Font License), centered on the head of the robot: the middle of the head meets the middle of the lowercase letters, raised by 0.011 em; the name starts 0.18 em after the tip of the arrow. The head is a quarter taller than the capitals.</p>
  <div class="page-row">${LOGO_GROUNDS.map(([title, ground, ink]) => `
    <div class="page-slot"><span>${title}</span>
      <div style="background:${ground};padding:20px 24px;border-radius:8px;display:flex;flex-direction:column;gap:14px">
        ${[64, 44, 24, 16].map(size => appWordmark(size, ink)).join("")}
      </div></div>`).join("")}
  </div>`]);
