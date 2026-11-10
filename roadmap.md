# Roadmap

Implementation steps of the features described in [specs.md](specs.md). Each step is delivered in its own pull request, which checks its box.

## Reference

Read only by the steps that need them:

- [Technical choices](docs/technical-choices.md): the libraries, the services and the main designs, with their reasons.
- [Roadmap archive](docs/roadmap-archive.md): the detail of the steps done, moved there once their pull request is merged.
- [Backlog](docs/backlog.md): the ideas, polish and edge cases found along the way, until the owner makes them a step.
- [GitHub Pages usage](docs/github-pages-usage.md): the limits and rules of the host of the website.

## Steps

Design of the screens first (phases 3 and 4 of [process.md](process.md), with a sheet of the world elements before them), then a workspace that tries the riskiest part early (WebAssembly stepped at each frame, deployed), then the core in Rust, then the animations, the design system and the interface, then the content. This order departs from process.md on purpose (decided on 2026-11-10): the core doesn't depend on the mock-up, so it is built and tried with real programs first, and the animations and tokens are tuned closer to the code that uses them. Each step is split into sub-steps when it starts, if that helps.

- [x] **0. Specifications** (#2)
- [x] **1. Sheet of the world elements** (#4, #5)
- [x] **2. Wireframes** (#6, #7, #8, #9)
- [ ] **3. Mock-up**: the HTML mock-up of every screen and state, with real sample code and errors, in `docs/mockups/interface.html`. The world keeps the elements of step 1. Done when every screen looks right at 1280 × 720 and on a large screen.
  - [x] **3a–3f**: style, states and sizes, keyboard, panels, home screen, manual (#11, #12, #13, #14, #15, #16)
  - [ ] **3g. Settings and messages**: the settings with the theme, the storage refused, the window too small and the phone, the browser too old, offline.
- [ ] **4. Workspace and probe**: the template adapted to a web application: a Cargo workspace (`crates/core`, a plain library with the lints that forbid panics; `crates/wasm`, its bridge), the SvelteKit application in `web/` with TypeScript strict, Fluent and `locales/en.ftl`, the WebAssembly build script, the CI (Rust and front-end checks, skipped for documentation only), the deployment to GitHub Pages instead of the release workflow, at `stepwise.sylent.art` (the path never hard-coded; the owner adds the DNS records, see [GitHub Pages usage](docs/github-pages-usage.md#address)), `CLAUDE.md` and the README updated (commands, `--workspace`), `main` protected by a ruleset requiring the checks. A probe from end to end, thrown away by the next steps: a fake program of a few steps in Rust, advanced by `step()` at each animation frame from Svelte, moving the robot on a small map at each of the five speeds, with the size of the WebAssembly module measured. Done when `https://stepwise.sylent.art/` shows the probe running.
- [ ] **5. Lexer**: the tokens of the language with their positions, the indentation turned into block starts and ends (four spaces, no tab), the texts and their escapes, the comments; every error as a code with its values and its position. Tests of every rule of [Lines and indentation](specs.md#lines-and-indentation) and [Names](specs.md#names).
- [ ] **6. Parser**: the syntax tree of every statement and expression, the priorities, the first syntax error with the helpful messages (`elif`, `=` for `==`, chained comparisons, a missing `:`).
- [ ] **7. Checker**: names and scopes, `let` before use, no hoisting, the number of values of a call, `break` and `return` in place, functions at the top level, reserved and `snake_case` names; every error found, not only the first.
- [ ] **8. Map and levels**: the map (cells, paints, painted passages, items, robot with its brush and eraser), the JSON level format with its version, its validation and its goals; a test that reads every level file of the repository.
- [ ] **9. Interpreter**: the step-by-step machine with its stack of frames, the values and their errors, the statements, the functions and the limit of 200 calls, the built-in functions acting on the map (the brush and the eraser as modes), the events of each step (robot action, printed line, current line), the goals checked at the end; tests running whole programs.
- [ ] **10. WebAssembly bridge**: the API used by the interface (check, start, step, state of the map, variables, events) with its TypeScript types, and tests from TypeScript.
- [ ] **11. Animations and sounds**: a section of the mock-up that decides the style of each action of the robot (`pick_up()` with its pincer, the bump into a wall, the end of a run that succeeds or fails), its duration at the normal speed, and the sounds, synthesized and CC0 compared by ear. The tuning at every speed is left to step 14, in the application.
- [ ] **12. Design system**: the tokens (colors of the two themes, spacing, radii, typography, motion) in `specs.md`, the fonts and their licenses, the tone of the messages; the mock-up updated to use exactly these tokens; an entry in `design-journal.md`.
- [ ] **13. Application shell**: the pages and their addresses, the header with its menus and the progress, the design tokens and the two themes, the texts from Fluent, the browser storage with its version and the banner when it is refused, the settings, the messages for a window too narrow and a browser too old.
- [ ] **14. Map view**: the elements of step 1 as Svelte components, the map of a level drawn from the core, the goal marks, the animations of every action, tuned at every speed with the owner.
- [ ] **15. Code editor**: CodeMirror with our theme, the highlighting, the indentation rules, the errors underlined as the learner types, the line at 80 characters, the code saved per level, **Restore starting code**.
- [ ] **16. Running**: **Run**, **Pause**, **Step**, **Reset**, the five speeds, the status bar, the current line, the step counter, the infinite loops that keep running without freezing the page; the console with the printed lines, the errors and their click to the editor.
- [ ] **17. Sounds**: the sounds of the end of a run (success, failure) and of the robot bumping into a wall, synthesized or CC0 as chosen by ear in step 11, played in time with the animations at every speed and never at each step; the setting **Sounds**, off by default.
- [ ] **18. Breakpoints and variables**: the breakpoints in the margin, saved with the code; the variables panel and the calls in progress.
- [ ] **19. Command menu**: Ctrl+Space, the categories, the filter, the descriptions and examples, the snippets of the blocks, Tab and Enter to insert.
- [ ] **20. Levels**: the home screen with the chapters, the levels opening in order and their codes (and the guide of the codes for the teachers, with its sentence for the learner who finds it), the goal and the hint, the variants and their marks, the goals not met shown on the map, the legend of the map with its new elements, the success panel, **Clear all progress**.
- [ ] **21. Manual**: its Markdown sections, its search, the section of the map with the drawing of each element, the addresses of its sections and the links from the levels, the command menu and the errors; its examples checked by the tests.
- [ ] **22. Levels of chapters 1 to 4**: First steps, Repeat, Paint, Decide; designed with the owner, each variant solved by a test.
- [ ] **23. Levels of chapters 5 to 8**: Loop until, Carry, Remember, Functions.
- [ ] **24. Sandbox**: its open map, its saved code.
- [ ] **25. First release**: a pass on every screen in Firefox, Chrome and Safari (or WebKit), the keyboard alone, the README (what Stepwise is, its address, how to develop it), version 0.1.0 and its tag.
