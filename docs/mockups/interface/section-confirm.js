// Section: the two confirmations. They open only from a menu or the settings, and say what is lost.
SECTIONS.push(["Confirmations", () => `<div class="page-row">
  <div class="page-slot"><span>1280 × 720, Level › Restore starting code…</span>${readyWindow({ overlay: appConfirm({ question: "Restore the starting code<br>of 18. Paint a square?", lost: "Your code will be replaced for good by the starting code of the level, as the first time.", action: "Restore starting code" }) })}</div>
  <div class="page-slot"><span>1280 × 720, Settings › Clear all progress…</span>${readyWindow({ overlay: appConfirm({ question: "Clear all your progress?", lost: "The levels done and the code of every level will be cleared for good. Your settings stay.", action: "Clear progress" }) })}</div>
</div>`]);
