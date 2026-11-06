// Section: the logo, the smiling head of the robot, at several sizes.

// The logo.
const logoFile = () => element("logo");

const LOGO_GROUNDS = [["On a light page", "#f4f6f8", "#222935"], ["On a dark page", "#1b1d20", "#e9ecef"]];

SECTIONS.push(["Logo", () => `
  <p>The head of the robot with happy eyes and a smile, alone: simple, and no taller than itself, so that the name stays on the line of the text beside it (decided in step 3a). It is also the icon of a browser tab. Shown 96, 48, 32 and 16 px high.</p>
  <div class="page-row">${LOGO_GROUNDS.map(([title, ground]) => `
    <div class="page-slot"><span>${title}</span>
      <div style="background:${ground};padding:20px;border-radius:8px">
        <div class="page-row" style="gap:16px;align-items:flex-end">${[96, 48, 32, 16].map(size => `<img src="${logoFile()}" height="${size}" alt="">`).join("")}</div>
      </div></div>`).join("")}
  </div>`]);
