// Section: the smiling head of the robot with different eyes, all shown at once.

const SMILE_EYES = [["arcs", "Happy arcs"], ["round", "Round"], ["shine", "Shining"], ["tall", "Tall"], ["wink", "Wink"], ["look-up", "Looking up"], ["hearts", "Hearts"], ["yellow-arcs", "Yellow arcs"]];

SECTIONS.push(["Smile, with other eyes", () => `
  <p>The smiling head, with eight kinds of eyes. Each one at 96, 32 and 16 px, on a light and a dark page.</p>
  ${LOGO_GROUNDS.map(([, ground, ink]) => `
  <div class="page-row" style="background:${ground};padding:20px;border-radius:8px;gap:28px;margin-bottom:10px">${SMILE_EYES.map(([file, name]) => `
    <div style="display:flex;flex-direction:column;gap:8px;align-items:flex-start;color:${ink};font-size:12px">
      <div style="display:flex;gap:10px;align-items:flex-end">${[96, 32, 16].map(size => `<img src="${element(`candidates/logo-smile-${file}`)}" width="${size}" height="${size}" alt="">`).join("")}</div>${name}</div>`).join("")}
  </div>`).join("")}`]);
