// Section: the logos compared, on the headers of the two themes, at the size of the header and larger.
const LOGO_GROUNDS = [["Day Desk", "#eceff3", "#1b2028"], ["Night Desk", "#171b21", "#e8ecf1"]];

SECTIONS.push(["Logo", () => `
  <p>Each variant keeps the height of the head of the robot, so that the name stays on the line of the menus. The tab <b>Logo</b> also changes it in the windows below.</p>
  <div class="page-row">${LOGO_GROUNDS.map(([title, ground, ink]) => `
    <div class="page-slot"><span>${title}</span>
      <div class="logo-sheet" style="background:${ground};color:${ink}">${Object.entries(LOGOS).map(([key, logo]) => `
        <div class="logo-row">
          <small>${logo.title}</small>
          ${appWordmark(21, key)}
          ${appWordmark(44, key)}
          <img src="interface/logo/${key}.svg" height="16" alt="">
        </div>`).join("")}
      </div></div>`).join("")}
  </div>`]);
