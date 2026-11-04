// Section: the font of the name beside the logo, six candidates under the SIL Open Font License.

const WORDMARK_FONTS = [
  ["Fredoka", 600, "Rounded and playful, like the robot."],
  ["Nunito", 800, "Rounded, calm, very readable."],
  ["Baloo 2", 700, "Chunky and friendly, a game title."],
  ["Lexend", 600, "Designed to make reading easier; clean."],
  ["Outfit", 700, "Geometric and clean, close to Shapez 2."],
  ["Space Grotesk", 700, "Technical, a nod to code."],
];

SECTIONS.push(["Font of the name", () => `
  <p>The name beside the logo in six fonts, all under the SIL Open Font License, so that the application can embed them. The tab <b>Name</b> changes its case. The fonts of the interface come with the design system (step 4).</p>
  ${LOGO_GROUNDS.map(([title, ground, ink]) => `<h3>${title}</h3>
  <div class="page-row" style="background:${ground};padding:20px;border-radius:8px;gap:36px 48px">${WORDMARK_FONTS.map(([font, weight, note]) => `
    <div style="display:flex;flex-direction:column;gap:12px;color:${ink}">
      <div style="display:flex;align-items:center;gap:12px;font:${weight} 44px/1 '${font}', sans-serif"><img src="${element("logo")}" width="60" height="60" alt="">${chosen.nameCase === "lower" ? "stepwise" : "Stepwise"}</div>
      <div style="display:flex;align-items:center;gap:7px;font:${weight} 19px/1 '${font}', sans-serif"><img src="${element("logo")}" width="26" height="26" alt="">${chosen.nameCase === "lower" ? "stepwise" : "Stepwise"}</div>
      <span style="font-size:12px;opacity:.75">${font} ${weight} · ${note}</span></div>`).join("")}
  </div>`).join("")}`]);
