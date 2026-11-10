// The messages that replace the whole screen, under the head of the disappointed robot (as an empty search),
// without the name: the window too small, the phone, the browser
// too old, the files that can't load. With the tab Messages on 9, a small scene of the robot under the text.

// The scenes: the robot between two walls, the robot before a wall with its eyes dim, its stroke broken.
const MESSAGE_SCENES = {
  small: () => appMap(["#.#"], { "1,0": { robot: "south" } }, 44),
  old: () => appMap(["..#"], { "1,0": { robot: "east", lit: "failure" } }, 44),
  offline: () => appMap(["rr.rr"], { "1,0": { robot: "east" } }, 44, ["0,0 1,0", "3,0 4,0"]),
};

// `kind` is the message, `size` the window in pixels; the window too small says its own size.
function appMessage(kind, [width, height]) {
  const address = '<p class="message-address">stepwise.sylent.art</p>';
  const texts = {
    small: ["Stepwise needs a larger window", `<p>At least 1280 × 720 pixels. This one is <b class="message-size">${width} × ${height}</b>.</p><p>Widen the window, or zoom out with Ctrl and −.</p>`],
    phone: ["Stepwise needs a computer", "<p>It is made for a large window.</p>"],
    old: ["Stepwise needs a recent browser", `<p>This browser can't run it. Update it, or open Stepwise in a recent Firefox, Chrome, Edge or Safari.</p>${address}`],
    offline: ["Stepwise couldn't load", '<p>The connection seems to be lost. Check it, then try again.</p><button type="button" class="app-primary">Try again</button>'],
  };
  const [title, body] = texts[kind];
  const scene = chosen.messages === "scene" ? `<div class="message-scene">${MESSAGE_SCENES[kind === "phone" ? "small" : kind]()}</div>` : "";
  return `<div class="app-message ${kind === "phone" ? "is-phone" : ""}"><img class="message-head" src="world-elements/daylight/logo-disappointed.svg" height="${kind === "phone" ? 64 : 72}" alt=""><div class="message-title">${title}</div>${body}${scene}</div>`;
}

const messageWindow = (kind, size) => `<div class="app-window" style="width:${size[0]}px;height:${size[1]}px">${appMessage(kind, size)}</div>`;

// The window too small, live: drag its corner. Below 800 pixels wide it asks for a computer; from
// 1280 × 720 the level screen comes back as it was.
const messageLiveWindow = () => `<div class="message-live" style="width:1024px;height:640px"></div>`;
function messageFill(box) {
  const width = Math.round(box.clientWidth);
  const height = Math.round(box.clientHeight);
  const kind = width >= 1280 && height >= 720 ? "level" : width < 800 ? "phone" : "small";
  if (box.dataset.kind === kind && kind !== "small") return;
  if (box.dataset.kind === "small" && kind === "small") { box.querySelector(".message-size").textContent = `${width} × ${height}`; return; }
  box.dataset.kind = kind;
  box.innerHTML = kind === "level" ? readyWindow() : `<div class="app-window" style="width:100%;height:100%">${appMessage(kind, [width, height])}</div>`;
}
const messageWatch = new ResizeObserver(entries => entries.forEach(entry => messageFill(entry.target)));
new MutationObserver(() => document.querySelectorAll(".message-live:not([data-kind])").forEach(box => { messageFill(box); messageWatch.observe(box); }))
  .observe(document.getElementById("sheet"), { childList: true });
