// The success panel, in the middle of the screen once the robot has hopped. The robot crosses its top
// from right to left, painting it, and leaves; it comes back from above and becomes the logo.

// The robot that becomes the logo: the shapes of robot.svg and eyes-lit.svg, and those of logo.svg, in
// the same grid of 64. The attributes are the logo, so that it stays when nothing moves; the animations
// of block-success.css start from the robot. The tracks end hidden under the head.
const SUCCESS_HEAD = `<svg class="success-head" viewBox="0 0 64 64" aria-hidden="true">
  <rect x="11" y="15" width="8" height="34" rx="3" fill="#222935"/>
  <rect x="45" y="15" width="8" height="34" rx="3" fill="#222935"/>
  <rect class="head-body" x="6" y="11" width="52" height="44" rx="14" fill="#fff" stroke="#222935" stroke-width="3"/>
  <rect class="head-bay" x="21" y="32" width="22" height="15" rx="3.5" fill="#d5dbe3"/>
  <rect class="head-visor" x="13" y="20" width="38" height="17" rx="8.5" fill="#222935"/>
  <circle class="head-dot" cx="26.5" cy="23.5" r="3.4" fill="#ffd23f"/>
  <circle class="head-dot" cx="37.5" cy="23.5" r="3.4" fill="#ffd23f"/>
  <path class="head-line" pathLength="1" d="M19.5 30.5 Q24 24 28.5 30.5" fill="none" stroke="#3fc3ff" stroke-width="3.2" stroke-linecap="round"/>
  <path class="head-line" pathLength="1" d="M35.5 30.5 Q40 24 44.5 30.5" fill="none" stroke="#3fc3ff" stroke-width="3.2" stroke-linecap="round"/>
  <path class="head-line" pathLength="1" d="M23 43 Q32 50 41 43" fill="none" stroke="#222935" stroke-width="3" stroke-linecap="round"/>
</svg>`;

const SUCCESS_KEYWORDS = ["function", "while", "if", "repeat", "break", "return", "let"];

// What the learner used: a comment and the words of the language, as in the editor.
const successUsed = used => `<div class="used-code"><span># You used</span><div>${used.map(word =>
  `<i class="${SUCCESS_KEYWORDS.includes(word) ? "is-keyword" : ""}">${word}</i>`).join("")}</div></div>`;

// `number` goes in the title; `level` and `maps` are the line under it, `used` the ideas the learner used, `lines` and `steps`
// its numbers, `next` the title of the next level, and nothing more of it: the learner discovers it by
// going on. The last level of the game has none, and the main button goes back to all the levels. The
// code of a level is never shown here: the codes are for the teacher.
function appSuccess({ number, level, maps, used, lines, steps, next }) {
  return `<div class="app-popup app-success">
    <div class="success-top">
      <div class="success-band"></div>
      <div class="success-crossing"><img src="world-elements/daylight/robot.svg" alt=""><img src="world-elements/daylight/eyes-lit.svg" alt=""></div>
      <div class="success-heading"><div class="success-band-title">Level ${number} done!</div>${SUCCESS_HEAD}</div>
    </div>
    <p class="success-level"><b>${level}</b> · <span>${maps}</span></p>
    <p class="success-numbers"><b>${lines}</b> lines · <b>${steps}</b> steps on the last map</p>
    ${successUsed(used)}
    <button type="button" class="goal-go"><span>${next ? `Next level: ${next}` : "All levels"}</span></button>
    <div class="success-others"><button type="button">Stay on this level</button>${next ? '<button type="button">All levels</button>' : ""}</div>
  </div>`;
}

// Plays again what moves as the panel appears, for every panel of the page.
const replaySuccess = () => document.querySelectorAll(".app-success").forEach(panel =>
  panel.getAnimations({ subtree: true }).forEach(animation => { animation.cancel(); animation.play(); }));
