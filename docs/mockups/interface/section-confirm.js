// Section: the confirmation of Restore starting code. It opens only from the menu Level, and says what is lost.
// Clear all progress has none: its button must be held (block-settings.js).
SECTIONS.push(["Confirmation", () => `<div class="page-row">
  <div class="page-slot"><span>1280 × 720, Level › Restore starting code…</span>${readyWindow({ overlay: appConfirm({ question: "Restore the starting code<br>of 18. Paint a square?", lost: "Your code will be replaced for good by the starting code of the level, as the first time.", action: "Restore starting code" }) })}</div>
</div>`]);
