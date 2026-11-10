// The icons, as SVG lines in the color of the text.
const appIcon = (size, body, fill = "none") => `<svg width="${size}" height="${size}" viewBox="0 0 16 16" fill="${fill}" stroke="currentColor" stroke-width="1.7" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">${body}</svg>`;

const ICONS = {
  play: size => appIcon(size, '<path d="M5 3.2 12.6 8 5 12.8Z"/>', "currentColor"),
  stop: size => appIcon(size, '<rect x="3.5" y="3.5" width="9" height="9" rx="1"/>', "currentColor"),
  pause: size => appIcon(size, '<path d="M5.5 3.4v9.2M10.5 3.4v9.2" stroke-width="2.4"/>'),
  diamond: size => appIcon(size, '<path d="M8 2.2 13.8 8 8 13.8 2.2 8Z"/>'),
  // An arrow down onto a dot, as the "step into" of the developer tools.
  step: size => appIcon(size, '<path d="M8 1.8v6.6M5 5.6l3 3 3-3"/><circle cx="8" cy="12.9" r="1.5" fill="currentColor" stroke="none"/>'),
  reset: size => appIcon(size, '<path d="M3.2 8a4.8 4.8 0 1 0 1.5-3.5"/><path d="M4.4 1.9v2.8h2.8"/>'),
  target: size => appIcon(size, '<circle cx="8" cy="8" r="5.6"/><circle cx="8" cy="8" r="1.8" fill="currentColor" stroke="none"/>'),
  gear: size => appIcon(size, '<path d="M6.24 3.32 L6.63 1.24 L9.37 1.24 L9.76 3.32 L10.06 3.45 L11.81 2.25 L13.75 4.19 L12.55 5.94 L12.68 6.24 L14.76 6.63 L14.76 9.37 L12.68 9.76 L12.55 10.06 L13.75 11.81 L11.81 13.75 L10.06 12.55 L9.76 12.68 L9.37 14.76 L6.63 14.76 L6.24 12.68 L5.94 12.55 L4.19 13.75 L2.25 11.81 L3.45 10.06 L3.32 9.76 L1.24 9.37 L1.24 6.63 L3.32 6.24 L3.45 5.94 L2.25 4.19 L4.19 2.25 L5.94 3.45 Z" stroke-width="1.4"/><circle cx="8" cy="8" r="2" stroke-width="1.4"/>'),
  // A warning: a triangle with its mark, for what works but will be lost.
  warning: size => appIcon(size, '<path d="M8 2.2 14.2 13H1.8Z"/><path d="M8 6.6v2.9"/><circle cx="8" cy="11.3" r=".5" fill="currentColor"/>'),
  check: size => appIcon(size, '<path d="m3 8.5 3.2 3.2L13 4.8"/>'),
  flag: size => appIcon(size, '<path d="M4.5 14V2.5"/><path d="M4.5 3h8l-2.4 3 2.4 3h-8Z" fill="currentColor"/>'),
  chevron: size => appIcon(size, '<path d="m6 3.5 4.5 4.5L6 12.5"/>'),
  close: size => appIcon(size, '<path d="m4 4 8 8M12 4l-8 8"/>'),
  next: size => appIcon(size, '<path d="M4.5 3.5 11 8l-6.5 4.5Z"/>', "currentColor"),
  search: size => appIcon(size, '<circle cx="7" cy="7" r="4.6"/><path d="m10.4 10.4 3.6 3.6"/>'),
  // A page opened elsewhere: an arrow out of its box.
  external: size => appIcon(size, '<path d="M9.5 2.5h4v4M13.5 2.5 7.5 8.5"/><path d="M11.5 9.5v3a1 1 0 0 1-1 1h-7a1 1 0 0 1-1-1v-7a1 1 0 0 1 1-1h3"/>'),
  // The arrows of the keyboard, as printed on its keys.
  arrowUp: size => appIcon(size, '<path d="M8 13.2V2.8M3.8 7 8 2.8 12.2 7"/>'),
  arrowDown: size => appIcon(size, '<path d="M8 2.8v10.4M3.8 9 8 13.2 12.2 9"/>'),
  arrowLeft: size => appIcon(size, '<path d="M13.2 8H2.8M7 3.8 2.8 8 7 12.2"/>'),
  arrowRight: size => appIcon(size, '<path d="M2.8 8h10.4M9 3.8 13.2 8 9 12.2"/>'),
};

// A shortcut as text, its arrow keys drawn as the arrows printed on the keyboard: a chevron could be
// read as a circumflex accent or the letter V.
const KEY_ARROWS = { "↑": "arrowUp", "↓": "arrowDown", "←": "arrowLeft", "→": "arrowRight" };
const appKeyLabel = keys => keys.replace(/[↑↓←→]/g, mark => `<span class="app-key-arrow">${ICONS[KEY_ARROWS[mark]](13)}</span>`);
