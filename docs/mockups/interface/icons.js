// The icons, as SVG lines in the color of the text.
const appIcon = (size, body, fill = "none") => `<svg width="${size}" height="${size}" viewBox="0 0 16 16" fill="${fill}" stroke="currentColor" stroke-width="1.7" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">${body}</svg>`;

const ICONS = {
  play: size => appIcon(size, '<path d="M5 3.2 12.6 8 5 12.8Z"/>', "currentColor"),
  // An arrow down onto a dot, as the "step into" of the developer tools.
  step: size => appIcon(size, '<path d="M8 1.8v6.6M5 5.6l3 3 3-3"/><circle cx="8" cy="12.9" r="1.5" fill="currentColor" stroke="none"/>'),
  reset: size => appIcon(size, '<path d="M3.2 8a4.8 4.8 0 1 0 1.5-3.5"/><path d="M4.4 1.9v2.8h2.8"/>'),
  target: size => appIcon(size, '<circle cx="8" cy="8" r="5.6"/><circle cx="8" cy="8" r="1.8" fill="currentColor" stroke="none"/>'),
  chevron: size => appIcon(size, '<path d="m6 3.5 4.5 4.5L6 12.5"/>'),
  close: size => appIcon(size, '<path d="m4 4 8 8M12 4l-8 8"/>'),
  next: size => appIcon(size, '<path d="M4.5 3.5 11 8l-6.5 4.5Z"/>', "currentColor"),
  external: size => appIcon(size, '<path d="M5.5 3.5h7v7M12.5 3.5l-9 9"/>'),
};
