# Roadmap

Implementation steps of the features described in [specs.md](specs.md). Each step is delivered in its own pull request, which checks its box.

## Reference

Read only by the steps that need them:

- [Technical choices](docs/technical-choices.md): the libraries, the services and the main designs, with their reasons.
- `docs/roadmap-archive.md`: the detail of the steps done, moved there once their pull request is merged.
- [Backlog](docs/backlog.md): the ideas, polish and edge cases found along the way, until the owner makes them a step.
- [GitHub Pages usage](docs/github-pages-usage.md): the limits and rules of the host of the website.

## Steps

Design first (phases 3 to 5 of [process.md](process.md), with a sheet of the world elements before them), then the core in Rust, then the interface, then the content. Each step is split into sub-steps when it starts, if that helps.

- [x] **0. Specifications** (#2)
- [ ] **1. Sheet of the world elements**: in `docs/mockups/`, each element of the map drawn alone in SVG (floor, wall, the five paints, the item, the goal marks, the robot in its four directions and carrying an item), in 3 or 4 styles compared with tabs, inspired by the clarity of Shapez 2; then all of them on a small map; the robot and the logo (its head). Done when the owner has chosen a style and approved every element; the final names and shades of the colors written in `specs.md`.
- [ ] **2. Wireframes**: Excalidraw, in `docs/wireframes/`: the level screen (goal, map with its variant tabs, editor, controls, console, variables panel), the home screen, the manual, the sandbox, the success panel, the command menu, the settings; then the states (running, paused, error, goal not met, storage refused, window too narrow). Done when the owner has approved the layout, written in `specs.md`.
- [ ] **3. Mock-up**: the HTML mock-up of every screen and state, in the style of step 1, with real sample code and errors; the keyboard shortcuts of the controls decided. Done when every scenario looks right at 1280 × 720 and on a large screen.
- [ ] **4. Design system**: the tokens (colors, spacing, radii, typography, motion) in `specs.md`, the fonts and their licenses, the tone of the messages; the mock-up updated to use exactly these tokens; an entry in `design-journal.md`.
- [ ] **5. Workspace**: the template adapted to a web application: a Cargo workspace (`crates/core`, a plain library with the lints that forbid panics; `crates/wasm`, its bridge), the SvelteKit application in `web/` with TypeScript strict, Fluent and `locales/en.ftl`, the WebAssembly build script, the CI (Rust and front-end checks, skipped for documentation only), the deployment to GitHub Pages instead of the release workflow, at `stepwise.sylent.art` (the path never hard-coded; the owner adds the DNS records, see [GitHub Pages usage](docs/github-pages-usage.md#address)), `CLAUDE.md` and the README updated (commands, `--workspace`), `main` protected by a ruleset requiring the checks. Done when `https://stepwise.sylent.art/` shows a text computed by Rust.
- [ ] **6. Lexer**: the tokens of the language with their positions, the indentation turned into block starts and ends (four spaces, no tab), the texts and their escapes, the comments; every error as a code with its values and its position. Tests of every rule of [Lines and indentation](specs.md#lines-and-indentation) and [Names](specs.md#names).
- [ ] **7. Parser**: the syntax tree of every statement and expression, the priorities, the first syntax error with the helpful messages (`elif`, `=` for `==`, chained comparisons, a missing `:`).
- [ ] **8. Checker**: names and scopes, `let` before use, no hoisting, the number of values of a call, `break` and `return` in place, functions at the top level, reserved and `snake_case` names; every error found, not only the first.
- [ ] **9. Map and levels**: the map (cells, paints, items, robot), the JSON level format with its version, its validation and its goals; a test that reads every level file of the repository.
- [ ] **10. Interpreter**: the step-by-step machine with its stack of frames, the values and their errors, the statements, the functions and the limit of 200 calls, the built-in functions acting on the map, the events of each step (robot action, printed line, current line), the goals checked at the end; tests running whole programs.
- [ ] **11. WebAssembly bridge**: the API used by the interface (check, start, step, state of the map, variables, events) with its TypeScript types, and tests from TypeScript.
- [ ] **12. Application shell**: the pages and their addresses, the design tokens, the texts from Fluent, the browser storage with its version and the banner when it is refused, the settings, the messages for a window too narrow and a browser too old.
- [ ] **13. Map view**: the elements of step 1 as Svelte components, the map of a level drawn from the core, the goal marks, the animations of every action at every speed.
- [ ] **14. Code editor**: CodeMirror with our theme, the highlighting, the indentation rules, the errors underlined as the learner types, the code saved per level, **Reset code**.
- [ ] **15. Running**: **Run**, **Pause**, **Step**, **Stop**, **Reset**, the five speeds, the current line, the step counter, the infinite loops that keep running without freezing the page; the console with the printed lines, the errors and their click to the editor.
- [ ] **16. Breakpoints and variables**: the breakpoints in the margin, saved with the code; the variables panel and the calls in progress.
- [ ] **17. Command menu**: Ctrl+Space, the categories, the filter, the descriptions and examples, the snippets of the blocks, **Suggest while typing**.
- [ ] **18. Levels**: the home screen with the chapters, the levels opening in order and **Unlock all levels**, the goal and the hint, the variants and their marks, the goals not met shown on the map, the success panel, **Reset progress**.
- [ ] **19. Manual**: its Markdown sections, its search, the addresses of its sections and the links from the levels, the command menu and the errors; its examples checked by the tests.
- [ ] **20. Levels of chapters 1 to 4**: First steps, Repeat, Paint, Decide; designed with the owner, each variant solved by a test.
- [ ] **21. Levels of chapters 5 to 8**: Loop until, Carry, Remember, Functions.
- [ ] **22. Sandbox**: its open map, its saved code.
- [ ] **23. First release**: a pass on every screen in Firefox, Chrome and Safari (or WebKit), the keyboard alone, the README (what Stepwise is, its address, how to develop it), version 0.1.0 and its tag.
