// Section: the name beside the logo, in Fredoka 600, the head of the robot lined up with the letters.

const NAME_FONT = "600 100px Fredoka, sans-serif";
// The head of the robot in logo.svg, out of 64: from 24 to 56.2 (its outline included).
const HEAD_TOP = 23.4 / 64;
const HEAD_BOTTOM = 56.8 / 64;
// The empty space on the right of the head, in logo.svg: from 51.3 to 64.
const HEAD_RIGHT_MARGIN = 12.7 / 64;

// The height of a letter of the name, out of its font size: an "S" for the capitals, an "x" for
// the lowercase letters.
function letterHeight(letter) {
  const context = document.createElement("canvas").getContext("2d");
  context.font = NAME_FONT;
  return context.measureText(letter).actualBoundingBoxAscent / 100;
}

// The name at a font size, centered on the head of the robot: the logo is sized so that its head is
// a little taller than the capitals, then raised so that the middle of its head meets the middle of
// the capitals or of the lowercase letters, as the tab "Center on" says.
function appWordmark(fontSize, ink) {
  const cap = letterHeight("S") * fontSize;
  const middleOfText = (chosen.nameAlign === "cap" ? cap : letterHeight("x") * fontSize) / 2;
  const size = cap / (HEAD_BOTTOM - HEAD_TOP) * 1.25;
  // Distance from the bottom of the image to the middle of the head.
  const middleOfHead = (1 - (HEAD_TOP + HEAD_BOTTOM) / 2) * size;
  const lift = middleOfText - middleOfHead + Number(chosen.nameOffset) * fontSize / 44;
  const name = chosen.nameCase === "lower" ? "stepwise" : "Stepwise";
  return `<div style="color:${ink};font:600 ${fontSize}px/1.4 Fredoka, sans-serif;white-space:nowrap">
    <img src="${element("logo")}" width="${size.toFixed(1)}" height="${size.toFixed(1)}" alt="" style="vertical-align:${lift.toFixed(2)}px;margin-right:${(fontSize * chosen.nameGap / 100 - HEAD_RIGHT_MARGIN * size).toFixed(1)}px">${name}</div>`;
}

SECTIONS.push(["Name", () => `
  <p>The name in Fredoka 600, under the SIL Open Font License. The name is centered on the head of the robot: on the middle of its capitals, or of its lowercase letters (tab <b>Center on</b>); the slider <b>Offset</b> tunes it by eye, and <b>Gap</b> sets the space between the head and the name. The tab <b>Name</b> changes its case.</p>
  <div class="page-row">${LOGO_GROUNDS.map(([title, ground, ink]) => `
    <div class="page-slot"><span>${title}</span>
      <div style="background:${ground};padding:20px 24px;border-radius:8px;display:flex;flex-direction:column;gap:14px">
        ${[64, 44, 24, 16].map(size => appWordmark(size, ink)).join("")}
      </div></div>`).join("")}
  </div>`]);
