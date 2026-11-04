// Headless screenshot of a mock-up page with Playwright and its own Chromium, called by
// screenshot.sh. Prints the errors and warnings of the browser console, and exits with an
// error when the page has one, so that a broken script never goes unnoticed.
//
//   node screenshot.cjs <url> <output.png> <width> <height | full>
//
// Playwright is installed once, outside the projects (about 150 MB for Chromium):
//   mkdir -p ~/.local/share/mockup-tools && cd ~/.local/share/mockup-tools
//   npm init -y && npm install playwright && npx playwright install chromium
// PLAYWRIGHT_HOME points to another installation folder.
const path = require("node:path");

const home = process.env.PLAYWRIGHT_HOME || path.join(process.env.HOME, ".local/share/mockup-tools");
const { chromium } = require(path.join(home, "node_modules/playwright"));

async function main() {
  const [url, output, width, height] = process.argv.slice(2);
  const fullPage = height === "full";
  const browser = await chromium.launch();
  let errors = 0;
  try {
    const page = await browser.newPage({ viewport: { width: Number(width), height: fullPage ? 900 : Number(height) } });
    page.on("console", message => {
      if (message.type() === "error" || message.type() === "warning") {
        console.error(`console ${message.type()}: ${message.text()}`);
        if (message.type() === "error") errors += 1;
      }
    });
    page.on("pageerror", error => {
      console.error(`page error: ${error.message}`);
      errors += 1;
    });
    page.on("requestfailed", request => {
      console.error(`request failed: ${request.url()}`);
      errors += 1;
    });
    await page.goto(url, { waitUntil: "networkidle" });
    await page.evaluate(() => document.fonts.ready);
    await page.screenshot({ path: output, fullPage });
  } finally {
    await browser.close();
  }
  console.log(output);
  if (errors > 0) process.exitCode = 2;
}

main().catch(error => {
  console.error(error.message);
  process.exitCode = 1;
});
