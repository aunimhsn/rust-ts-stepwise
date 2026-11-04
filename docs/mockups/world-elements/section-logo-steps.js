// Section: the smiling head of the robot with the idea of steps, all shown at once.

const STEP_HEADS = [["stair-mouth", "A smile of steps"], ["trail", "Step by step"], ["on-stairs", "On the stairs"], ["path", "A painted path"], ["tracks", "Its tracks"]];

SECTIONS.push(["Smile and steps", () => `
  <p>The familiar head of the robot, with the idea of the name: step by step. Each one at 96, 32 and 16 px, on a light and a dark page.</p>
  ${LOGO_GROUNDS.map(([, ground, ink]) => `
  <div class="page-row" style="background:${ground};padding:20px;border-radius:8px;gap:28px;margin-bottom:10px">${STEP_HEADS.map(([file, name]) => `
    <div style="display:flex;flex-direction:column;gap:8px;align-items:flex-start;color:${ink};font-size:12px">
      <div style="display:flex;gap:10px;align-items:flex-end">${[96, 32, 16].map(size => `<img src="${element(`candidates/logo-step-${file}`)}" width="${size}" height="${size}" alt="">`).join("")}</div>${name}</div>`).join("")}
  </div>`).join("")}`]);
