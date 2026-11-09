// A confirmation, over a veil: the question, what is lost for good, then Cancel and the action, the main
// button, red because it can't be undone. Enter confirms, Escape cancels.
const appConfirm = ({ question, lost, action }) => `<div class="app-veil"><div class="app-popup app-confirm">
  <div class="confirm-question">${question}</div>
  <p>${lost}</p>
  <div class="confirm-buttons"><button type="button" class="app-chip">Cancel</button><button type="button" class="app-primary">${action}</button></div>
</div></div>`;
