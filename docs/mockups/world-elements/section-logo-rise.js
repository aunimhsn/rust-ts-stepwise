// Section: the smiling head with happy eyes, a yellow stroke of paint passing behind it and
// rising above it as an arrow, all shown at once, with the name beside them.

const RISE_LOGOS = [["up", "Straight up"], ["up-chevron", "Straight up, chevron"], ["diagonal", "Diagonal"], ["steps", "In steps"], ["swoosh", "Swoosh"]];

SECTIONS.push(["Smile and a rising stroke", () => `
  <p>The head with happy eyes; behind it a stroke of yellow paint rises above it as an arrow, as if it progressed. Each one at 96, 32 and 16 px, then beside the name, on a light and a dark page.</p>
  ${LOGO_GROUNDS.map(([, ground, ink]) => `
  <div class="page-row" style="background:${ground};padding:20px;border-radius:8px;gap:28px;margin-bottom:10px">${RISE_LOGOS.map(([file, name]) => `
    <div style="display:flex;flex-direction:column;gap:10px;align-items:flex-start;color:${ink};font-size:12px">
      <div style="display:flex;gap:10px;align-items:flex-end">${[96, 32, 16].map(size => `<img src="${element(`candidates/logo-rise-${file}`)}" width="${size}" height="${size}" alt="">`).join("")}</div>
      <div style="display:flex;align-items:center;gap:8px;font:700 22px/1 system-ui, sans-serif"><img src="${element(`candidates/logo-rise-${file}`)}" width="34" height="34" alt="">Stepwise</div>${name}</div>`).join("")}
  </div>`).join("")}`]);
