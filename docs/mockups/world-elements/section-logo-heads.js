// Section: the head of the robot as a logo, with expressions and accessories, all shown at once:
// five close to the robot, five further away.

const LOGO_HEADS = [["Close to the robot", [["smile", "Smile"], ["cap", "Graduation cap"], ["level-up", "Level up"], ["wink", "Wink"], ["sparkles", "Sparkles"]]], ["Further away", [["pixel", "Pixel"], ["bubble", "Speech bubble"], ["xp-bar", "Progress bar"], ["growth", "Growing up"], ["bulb", "Bright idea"]]]];

SECTIONS.push(["Logo heads", () => `
  <p>The head of the robot again, with an expression, an accessory or a hint of progress: five close to the robot, five further away. Each one at 96, 32 and 16 px, on a light and a dark page.</p>
  ${LOGO_HEADS.map(([group, heads]) => `<h3>${group}</h3>${LOGO_GROUNDS.map(([, ground, ink]) => `
  <div class="page-row" style="background:${ground};padding:20px;border-radius:8px;gap:28px;margin-bottom:10px">${heads.map(([file, name]) => `
    <div style="display:flex;flex-direction:column;gap:8px;align-items:flex-start;color:${ink};font-size:12px">
      <div style="display:flex;gap:10px;align-items:flex-end">${[96, 32, 16].map(size => `<img src="${element(`candidates/logo-head-${file}`)}" width="${size}" height="${size}" alt="">`).join("")}</div>${name}</div>`).join("")}
  </div>`).join("")}`).join("")}`]);
