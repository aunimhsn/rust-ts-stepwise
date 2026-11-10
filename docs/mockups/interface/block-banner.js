// The banner of the storage refused: one line across the whole width, under the header of every screen.
// × hides it until the next visit. Its color follows the tab Banner of the page.
const appStorageBanner = () => `<div class="app-banner" role="status">
  <i>${ICONS.warning(16)}</i><span>Your progress can't be saved in this browser and will be lost when this tab closes.</span>
  <button type="button" title="Hide until the next visit">${ICONS.close(14)}</button>
</div>`;

// A click on × hides the banner of its window, the screen taking back its place.
document.addEventListener("click", event => {
  const close = event.target.closest?.(".app-banner button");
  if (!close) return;
  const screen = close.closest(".app-window");
  close.closest(".app-banner").remove();
  screen.classList.remove("has-banner");
});
