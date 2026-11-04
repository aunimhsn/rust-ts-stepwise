// Section: the logo, the head of the robot, at the sizes of an icon and beside the name.

const logoFile = () => element(`candidates/logo-${chosen.logo}-${chosen.logoEyes}`);
const LOGO_GROUNDS = [["On a light page", "#f4f6f8", "#222935"], ["On a dark page", "#1b1d20", "#e9ecef"]];

SECTIONS.push(["Logo", () => `
  <p>The head of the robot, as the logo of Stepwise: the tabs <b>Logo</b> and <b>Logo eyes</b> change it. Shown at 128, 64, 32 and 16 px (the icon of a browser tab), then beside the name, whose font comes with the design system (step 4).</p>
  <div class="page-row">${LOGO_GROUNDS.map(([title, ground, ink]) => `
    <div class="page-slot"><span>${title}</span>
      <div style="background:${ground};padding:20px;border-radius:8px">
        <div class="page-row" style="gap:16px;align-items:flex-end">${[128, 64, 32, 16].map(size => `<img src="${logoFile()}" width="${size}" height="${size}" alt="">`).join("")}</div>
        <div style="display:flex;align-items:center;gap:10px;margin-top:20px;color:${ink};font:700 28px/1 system-ui, sans-serif"><img src="${logoFile()}" width="40" height="40" alt="">Stepwise</div>
      </div></div>`).join("")}
  </div>`]);
