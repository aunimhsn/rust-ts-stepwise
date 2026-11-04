// Section: ten ideas of logo, away from the head of the robot, all shown at once.

const LOGO_IDEAS = [["stroke-s", "Painted S"], ["stairs", "Stairs"], ["footsteps", "Footsteps"], ["caret", "Blinking eye"], ["prompt", "Prompt"], ["trio", "Three steps of paint"], ["hexbot", "Item with eyes"], ["braces", "Braces"], ["grid-path", "A level"], ["turn", "turn_right()"]];

SECTIONS.push(["Logo ideas", () => `
  <p>Ten ideas, to look for directions rather than to choose one as it is: the steps, the painted stroke, the code, the three paints, the item. Each one at 96, 32 and 16 px, on a light and a dark page.</p>
  ${LOGO_GROUNDS.map(([title, ground, ink]) => `<h3>${title}</h3>
  <div class="page-row" style="background:${ground};padding:20px;border-radius:8px;gap:28px">${LOGO_IDEAS.map(([file, name]) => `
    <div style="display:flex;flex-direction:column;gap:8px;align-items:flex-start;color:${ink};font-size:12px">
      <div style="display:flex;gap:10px;align-items:flex-end">${[96, 32, 16].map(size => `<img src="${element(`candidates/logo-idea-${file}`)}" width="${size}" height="${size}" alt="">`).join("")}</div>${name}</div>`).join("")}
  </div>`).join("")}`]);
