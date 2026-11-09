# Roadmap

Implementation steps of the features described in [specs.md](specs.md). Each step is delivered in its own pull request, which checks its box.

## Reference

Read only by the steps that need them:

- [Technical choices](docs/technical-choices.md): the libraries, the services and the main designs, with their reasons.
- [Roadmap archive](docs/roadmap-archive.md): the detail of the steps done, moved there once their pull request is merged.
- [Backlog](docs/backlog.md): the ideas, polish and edge cases found along the way, until the owner makes them a step.
- [GitHub Pages usage](docs/github-pages-usage.md): the limits and rules of the host of the website.

## Steps

Design first (phases 3 to 5 of [process.md](process.md), with a sheet of the world elements before them), then the core in Rust, then the interface, then the content. Each step is split into sub-steps when it starts, if that helps.

- [x] **0. Specifications** (#2)
- [x] **1. Sheet of the world elements** (#4, #5)
- [x] **2. Wireframes** (#6, #7, #8, #9)
- [ ] **3. Mock-up**: the HTML mock-up of every screen and state, with real sample code and errors, in `docs/mockups/interface.html`. The world keeps the elements of step 1. Done when every scenario looks right at 1280 × 720 and on a large screen.
  - [x] **3a. Style of the interface** (#11)
  - [ ] **3b. States and sizes of the window**: every state of a run (ready, running, typing refused, paused on a breakpoint, errors before running, goal not met, a map and the level done), the three controls with their hover, the variables panel, the sandbox, the large screen and the largest map.
  - [ ] **3c. Keyboard**: the four menus, the keyboard shortcuts of the controls decided, the command menu.
  - [ ] **3d. Panels**: the legend, the success panel, the confirmations.
  - [ ] **3e. Other screens**: the home screen and its path, the manual, the settings with the theme, the messages (the storage refused, the window too small, the browser too old, offline).
  - [ ] **3f. Animations and sounds**: the animations of the map as mini-scenes to replay and tune, at every speed (each action of the robot, `pick_up()` with its pincer, the end of a run that succeeds or fails), with their sounds, synthesized and CC0 compared by ear.
- [ ] **4. Design system**: the tokens (colors of the two themes, spacing, radii, typography, motion) in `specs.md`, the fonts and their licenses, the tone of the messages; the mock-up updated to use exactly these tokens; an entry in `design-journal.md`.
- [ ] **5. Workspace**: the template adapted to a web application: a Cargo workspace (`crates/core`, a plain library with the lints that forbid panics; `crates/wasm`, its bridge), the SvelteKit application in `web/` with TypeScript strict, Fluent and `locales/en.ftl`, the WebAssembly build script, the CI (Rust and front-end checks, skipped for documentation only), the deployment to GitHub Pages instead of the release workflow, at `stepwise.sylent.art` (the path never hard-coded; the owner adds the DNS records, see [GitHub Pages usage](docs/github-pages-usage.md#address)), `CLAUDE.md` and the README updated (commands, `--workspace`), `main` protected by a ruleset requiring the checks. Done when `https://stepwise.sylent.art/` shows a text computed by Rust.
- [ ] **6. Lexer**: the tokens of the language with their positions, the indentation turned into block starts and ends (four spaces, no tab), the texts and their escapes, the comments; every error as a code with its values and its position. Tests of every rule of [Lines and indentation](specs.md#lines-and-indentation) and [Names](specs.md#names).
- [ ] **7. Parser**: the syntax tree of every statement and expression, the priorities, the first syntax error with the helpful messages (`elif`, `=` for `==`, chained comparisons, a missing `:`).
- [ ] **8. Checker**: names and scopes, `let` before use, no hoisting, the number of values of a call, `break` and `return` in place, functions at the top level, reserved and `snake_case` names; every error found, not only the first.
- [ ] **9. Map and levels**: the map (cells, paints, painted passages, items, robot with its brush and eraser), the JSON level format with its version, its validation and its goals; a test that reads every level file of the repository.
- [ ] **10. Interpreter**: the step-by-step machine with its stack of frames, the values and their errors, the statements, the functions and the limit of 200 calls, the built-in functions acting on the map (the brush and the eraser as modes), the events of each step (robot action, printed line, current line), the goals checked at the end; tests running whole programs.
- [ ] **11. WebAssembly bridge**: the API used by the interface (check, start, step, state of the map, variables, events) with its TypeScript types, and tests from TypeScript.
- [ ] **12. Application shell**: the pages and their addresses, the header with its menus and the progress, the design tokens and the two themes, the texts from Fluent, the browser storage with its version and the banner when it is refused, the settings, the messages for a window too narrow and a browser too old.
- [ ] **13. Map view**: the elements of step 1 as Svelte components, the map of a level drawn from the core, the goal marks, the animations of every action at every speed.
- [ ] **14. Code editor**: CodeMirror with our theme, the highlighting, the indentation rules, the errors underlined as the learner types, the line at 80 characters, the code saved per level, **Restore starting code**.
- [ ] **15. Running**: **Run**, **Pause**, **Step**, **Reset**, the five speeds, the status bar, the current line, the step counter, the infinite loops that keep running without freezing the page; the console with the printed lines, the errors and their click to the editor.
- [ ] **16. Sounds**: the sounds of the end of a run (success, failure) and of the robot bumping into a wall, synthesized or CC0 as chosen by ear in step 3, played in time with the animations at every speed and never at each step; the setting **Sounds**, off by default.
- [ ] **17. Breakpoints and variables**: the breakpoints in the margin, saved with the code; the variables panel and the calls in progress.
- [ ] **18. Command menu**: Ctrl+Space, the categories, the filter, the descriptions and examples, the snippets of the blocks, Tab and Enter to insert.
- [ ] **19. Levels**: the home screen with the chapters, the levels opening in order and their codes (and the guide of the codes for the teachers), the goal and the hint, the variants and their marks, the goals not met shown on the map, the legend of the map with its new elements, the success panel, **Clear all progress**.
- [ ] **20. Manual**: its Markdown sections, its search, the section of the map with the drawing of each element, the addresses of its sections and the links from the levels, the command menu and the errors; its examples checked by the tests.
- [ ] **21. Levels of chapters 1 to 4**: First steps, Repeat, Paint, Decide; designed with the owner, each variant solved by a test.
- [ ] **22. Levels of chapters 5 to 8**: Loop until, Carry, Remember, Functions.
- [ ] **23. Sandbox**: its open map, its saved code.
- [ ] **24. First release**: a pass on every screen in Firefox, Chrome and Safari (or WebKit), the keyboard alone, the README (what Stepwise is, its address, how to develop it), version 0.1.0 and its tag.
