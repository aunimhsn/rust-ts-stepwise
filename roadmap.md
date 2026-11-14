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

Model: the steps whose design lasts, marked *(Fable 5.2)*, run on Fable 5.2; the others on Opus 5.5 (decided on 2026-11-10, to check at the retrospective). At the start of a step whose model is not the one of the session, before anything else, ask the owner with a short form to switch with `/model`.

- [x] **0. Specifications** (#2)
- [x] **1. Sheet of the world elements** (#4, #5)
- [x] **2. Wireframes** (#6, #7, #8, #9)
- [x] **3. Mock-up** (#11, #12, #13, #14, #15, #16, #17)
- [x] **4. Workspace and probe** (#18, #19, #21, #22)
- [x] **5. Lexer** (#23)
- [x] **6. Parser** (#25, #26, #27)
- [x] **7. Checker** (#29, #30, #31)
- [x] **8. Map and levels** (#32, #33)
- [x] **9. Interpreter** (#34, #35, #36)
- [ ] **10. WebAssembly bridge** *(Fable 5.2)*: the API used by the interface (check, start, step, state of the map, variables, events) with its TypeScript types, and tests from TypeScript, one of which checks and runs a program at the maximum depth of the tree through the real module, so that the margin of the stack is tested where it matters, and another compares the lines highlighted and the place of an error for a program with accents and an emoji, the bridge being the one place where the positions change.
  - [x] **10a. Checks** (#37)
  - [ ] **10b. Levels and map**: a level file or the sandbox read through the bridge: the starting code and, for each map, the world as data (cells, paints, passages, items, the robot) and the goals to mark.
  - [ ] **10c. Running**: start, the machine asking for the checked program of 10a, step, the state, the events, the errors while running, the goals checked at the end, the calls in progress and their variables; the two tests through the real module (the maximum depth, the lines highlighted with accents and an emoji); the probe of step 4c replaced or removed; the size and the speed of the module measured again (see the [backlog](docs/backlog.md)).
- [ ] **11. Animations and sounds**: a section of the mock-up that decides the style of each action of the robot (`pick_up()` with its pincer, the bump into a wall, the end of a run that succeeds or fails), its duration at the normal speed, and the sounds, synthesized and CC0 compared by ear. The tuning at every speed is left to step 14, in the application.
- [ ] **12. Design system**: the tokens (colors of the two themes, spacing, radii, typography, motion) in `specs.md`, the fonts and their licenses, the tone of the messages; the mock-up updated to use exactly these tokens; an entry in `design-journal.md`.
- [ ] **13. Application shell**: the pages and their addresses, the header with its menus and the progress, the design tokens and the two themes, the texts from Fluent, the browser storage with its version and the banner when it is refused, the settings, the messages for a window too narrow and a browser too old.
- [ ] **14. Map view**: the elements of step 1 as Svelte components, the map of a level drawn from the core, the goal marks, the animations of every action, tuned at every speed with the owner.
- [ ] **15. Quality audit** *(Fable 5.2)*: an audit of the whole code, not of one branch, once the core is complete (steps 9 and 10) and the interface has its first bases (steps 13 and 14), before the steps of the interface copy its patterns: the responsibilities of the modules and the components and their dependencies, no type or component that does everything, SOLID and KISS (and the splits that cost more than they bring), the naming and the conventions of `CLAUDE.md`, what Rust and TypeScript repeat; a report in `outputs/`, the owner choosing what is kept, then those refactorings, one pull request per coherent theme, so that the refactoring before the first release stays small (decided on 2026-11-12).
- [ ] **16. Code editor**: CodeMirror with our theme, the highlighting, the indentation rules, the errors underlined as the learner types, the line at 80 characters, the code saved per level, **Restore starting code**.
- [ ] **17. Running**: **Run**, **Pause**, **Step**, **Reset**, the five speeds, the status bar, the current line, the step counter, the infinite loops that keep running without freezing the page; the console with the printed lines, the errors and their click to the editor; two or three tests in a real browser (Playwright, added to `web/` and to the CI) on the main path, which check the wiring and not the logic: **Run** and the highlight that follows, an error in the console and its click, **Pause** and **Step**.
- [ ] **18. Sounds**: the sounds of the end of a run (success, failure) and of the robot bumping into a wall, synthesized or CC0 as chosen by ear in step 11, played in time with the animations at every speed and never at each step; the setting **Sounds**, off by default.
- [ ] **19. Breakpoints and variables**: the breakpoints in the margin, saved with the code; the variables panel and the calls in progress.
- [ ] **20. Command menu**: Ctrl+Space, the categories, the filter, the descriptions and examples, the snippets of the blocks, Tab and Enter to insert.
- [ ] **21. Levels**: the home screen with the chapters, the levels opening in order and their codes (and the guide of the codes for the teachers, with its sentence for the learner who finds it), the goal and the hint, the variants and their marks, the goals not met shown on the map, the legend of the map with its new elements, the success panel, **Clear all progress**.
- [ ] **22. Manual**: its Markdown sections, its search, the section of the map with the drawing of each element, the addresses of its sections and the links from the levels, the command menu and the errors; its examples checked by the tests.
- [ ] **23. Levels of chapters 1 to 4**: First steps, Repeat, Paint, Decide; designed with the owner, each variant solved by a test.
- [ ] **24. Levels of chapters 5 to 8**: Loop until, Carry, Remember, Functions.
- [ ] **25. Sandbox**: its open map, its saved code.
- [ ] **26. Content license**: the code stays under MIT or Apache 2.0; the content (levels, drawings, sounds, texts of the manual) goes under CC BY-NC-ND 4.0, so that nobody sells it nor publishes a modified or extended copy of the game, a paid version on Steam being considered (see the [backlog](docs/backlog.md)). A license file for the content with the list of the folders it covers; a sentence in the README that grants more to teachers and trainers (using Stepwise in their lessons, paid ones included, and adapting the levels for their learners), since "non-commercial" is unclear for a paid training; the `license` field of `Cargo.toml` left to the code. What was published before under MIT or Apache 2.0 stays so for those versions. Not legal advice: to have read by a professional before a paid release (decided on 2026-11-10).
- [ ] **27. First release**: a pass on every screen in Firefox, Chrome and Safari (or WebKit), the keyboard alone, the README (what Stepwise is, its address, how to develop it), version 0.1.0 and its tag.
