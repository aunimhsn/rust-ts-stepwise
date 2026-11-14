# Roadmap archive

The detail of the steps done, moved here from [roadmap.md](../roadmap.md) once their pull request is merged.

## 1. Sheet of the world elements

### 1a. Styles compared (#4)

Ten styles tried; Daylight chosen (a light gray floor, slate walls, the clearest for learning and on a projector), the paint drawn as a stroke of 28 through the centers of the cells along the path of the robot, the paint under the robot left hidden.

### 1b. Daylight refined (#5)

The goal marks (a ghost of the stroke, slots, a red flag when not met), the floor and the slate walls with a rim and an edge, the paints scarlet, bright amber and cobalt, the teal hexagon in the bay of the robot, its pincer and its eyes at the end of a run, the logo and the name; the shades in `specs.md`.

## 2. Wireframes

### 2a. Level screen (#6)

Four layouts compared, layout B (code left, map right) refined: a menu bar, the goal in a bubble, three floating controls, a status bar, the maps in the header, the large screen.

### 2b. Other screens (#7)

The home screen (a path of stones in rows, the chapters as bands, the level chosen on the right with **Play**), the manual (sections, search, content, *On this page*, a drawer from Ctrl+click), the sandbox as the level screen without goal, and what opens over a screen: the success panel, the legend, the command menu in three columns, the settings, the confirmations.

### 2c. States of a run (#8)

In `docs/wireframes/states.excalidraw`, from the level screen: running (Pause first, Step greyed, the editor locked and the status bar saying so when the learner types), paused on a breakpoint (the tab Variables opened), the errors before running (underlined, listed in the console), an error while running (its own mark ✕ in the margin, the code editable at once), a goal not met (a flag on each cell missed).

### 2d. Messages (#9)

In `docs/wireframes/states.excalidraw`: the storage refused (a banner of one line under the header), the window too small and the phone, the browser too old, and the files that can't load when offline, each message replacing the screen under the logo.

## 3. Mock-up

### 3a. Style of the interface (#11)

The level screen (paused, an error while running, a menu, the goal) in 3 or 4 styles around the world of step 1; Day Desk and Night Desk kept as the two themes; the logo reworked so that the name lines up with the menus.

### 3b. States and sizes of the window (#12)

Every state of a run (ready, running, typing refused, paused on a breakpoint, errors before running, goal not met, a map and the level done), the three controls with their hover, the variables panel, the sandbox, the large screen and the largest map.

### 3c. Keyboard (#13)

The four menus, the keyboard shortcuts of the controls decided (checked against those of the browsers, such as F5 and F11, on an AZERTY keyboard), the shortcuts of comfort of the editor (see [Code editor](../specs.md#code-editor)), the command menu.

### 3d. Panels (#14)

The success panel, with the robot that paints its top and turns into the logo; the legend of the map; the two confirmations. The level codes became the teacher's alone: the interface never shows them.

### 3e. Home screen (#15)

The path of the levels with its chapters, the stones and the squares of their maps, the robot on the level chosen; the panel of a level to play, done and locked with its code; the large screen, and more levels than fit.

### 3f. Manual (#16)

A section with its commands and examples, the search and its results, **On this page**, the drawer over the level screen, the large screen.

### 3g. Settings and messages (#17)

The settings with the theme, the storage refused, the window too small and the phone, the browser too old, offline.

## 4. Workspace and probe

### 4a. Cargo workspace (#18)

`crates/core` (a plain library with the lints that forbid panics), `crates/wasm` (its bridge, built for `wasm32-unknown-unknown`), `src/main.rs` and the release workflow removed, the Rust CI on the whole workspace (with the WebAssembly target and the minimum version), `CLAUDE.md` and the README updated (commands, `--workspace`). Reviewed twice, by `code-reviewer` on Opus 5.5 and on Fable 5.2, to compare the two models.

### 4b. SvelteKit application (#19)

`web/` with TypeScript strict, `adapter-static`, Fluent and `locales/en.ftl`, ESLint, Prettier, Vitest and `svelte-check`; `web/scripts/build-wasm.ts` (`cargo`, then `wasm-bindgen` at the exact version of the crate, checked before building); the front-end checks in the CI, skipped for documentation only behind the single required job `CI`; a page showing a Fluent message and the version of the core computed in Rust.

### 4c. Probe (#21)

A fake program of a few steps in Rust, advanced by `step()` at each animation frame from Svelte, moving the robot on a small map at each of the five speeds; the size of the WebAssembly module measured, with `opt-level` 3, `"s"` and `"z"` compared and the choice written in `docs/technical-choices.md`.

### 4d. Deployment (#22)

The website deployed to GitHub Pages by a workflow at `stepwise.sylent.tools` (the path never hard-coded; the owner adds the DNS records, see [GitHub Pages usage](github-pages-usage.md#address)); `main` protected by a ruleset requiring the check `CI` (the last job of `.github/workflows/ci.yml`).

## 5. Lexer (#23)

The tokens of the language with their positions, the indentation turned into block starts and ends (four spaces, no tab), the texts and their escapes, the comments; every error as a code with its values and its position. Tests of every rule of [Lines and indentation](../specs.md#lines-and-indentation) and [Names](../specs.md#names).

## 6. Parser

### 6a. Expressions and simple statements (#25)

The syntax tree and `parse(source)`; the expressions with their priorities and the calls; `let`, the assignment, `break`, `return`, an expression alone on its line; the errors of the expressions (`=` for `==`, chained comparisons, a parenthesis not closed, a value missing, a comma at the end) and the words of other languages (`elif`, `def`, `for`…), wherever they are; a maximum depth of 200 for the tree, long chains of operators included, an error rather than an overflow of the stack of the browser, in the parser or later in the checker and the interpreter. A line that opens a block is still an error.

### 6b. Blocks (#26)

`if`, `else if`, `else`, `while`, `repeat`; the errors of the blocks (a missing `:`, something after the `:`, an empty block, a block that holds only comments, which are the comments indented further than the line of the `:`, the lexer giving the lines of the comments it drops, an indentation that follows no `:`, an `else` without its `if`, an `else` after the `else`); each block counts for one level of the maximum depth, and the blocks alone nest up to 50 deep.

### 6c. Functions (#27)

`function` with its parameters and the errors of its line (a missing name, missing parentheses, a parameter that isn't a name alone, a comma after the last parameter); the stack the parser needs at the maximum depth, measured again in WebAssembly with 50 blocks around the deepest expression, the worst case (see [Technical choices](technical-choices.md), Parser).

## 7. Checker

### 7a. Structure and form of the names (#29)

`check(&Program)` and its errors as codes; `break` and `return` in place, functions at the top level, `snake_case` names, the names reserved by the built-in functions and the colors (their lists, shared with the interpreter), two parameters of the same name.

### 7b. Variables (#30)

The scopes of the blocks and of the functions; a variable used before its `let`, after the end of its block, or from a function that doesn't see it (three errors of their own, for a variable read or changed); a name declared twice; a variable changed without `let`; unknown names; the colors as values. The checker then walks the expressions, and checks the form of a new name apart from declaring it, since `let x = x + 1` reads its value in between; a reserved name that is given a value keeps its one error. The messages state the error and recall the rule, without suggesting a fix (decided on 2026-11-11, see [Design principles](../specs.md#design-principles)). Until 7c, a function of the program named without parentheses is an unknown name, and a built-in one no error.

### 7c. Functions (#31)

A function called above its definition, two functions of the same name, a variable or a parameter named as a function, the number of values of a call (built-in functions included, `print` taking any number), a function named without parentheses, a variable called like a function, a call of a name that is no function, an expression alone on its line that isn't a call. A first pass collects the functions of the program: a name is refused as soon as a function has it, wherever it is defined. Eight gaps of the specifications decided on 2026-11-11, written in [Names](../specs.md#names), [Statements](../specs.md#statements) and [Functions](../specs.md#functions).

## 8. Map and levels

### 8a. Map (#32)

The world in Rust alone, without `serde`: its cells (floor or wall, at most 16 × 12), paints, painted passages (a passage has no color: each cell draws its half in its own), items, and the robot with its direction, the item it carries, and its brush or its eraser; every action of a built-in function and every sensor as a method of the world, returning its error (the bump, the item missing or already carried, the robot's state checked before the cell), so that step 9 only calls them. Decided on 2026-11-12: [Map](../specs.md#map), [Built-in functions](../specs.md#built-in-functions).

### 8b. Levels (#33)

The TOML format with its version (decided on 2026-11-12 rather than JSON, see [Technical choices](technical-choices.md), Levels), one file per level in `web/levels/` named after its stable name, its validation (a map of a level has goals, not all met at the start, and enough items), the goals checked on a world with the cell of each goal not met, the file of the sandbox, and a test that reads every level file of the repository and checks its starting code.

## 9. Interpreter

### 9a. Machine and statements (#34)

The values, the expressions and their errors (types, division by zero, overflow), the variables, `if`, `while`, `repeat`, `break`, the steps and the current line; `print` alone among the built-in functions, so that the tests see what a program does. Until 9b and 9c, calling a function of the program or another built-in function stops the program with the error kept for what the checks rule out.

### 9b. Functions (#35)

The calls, the stack of frames, the parameters, `return`, the value missing, the limit of 200 calls; the calls in progress and their variables, readable for the variables panel.

### 9c. Robot (#36)

The built-in functions acting on the map and the sensors, their errors, a value that is no color given to a function that takes one (decided on 2026-11-14), what each step reports (the action of the robot by its name, the printed line; an action that fails is told by its error alone), and whole programs run on the maps of three trial levels kept with the tests, their goals checked at the end. The probe of step 4c stayed: the page still ran it through the bridge.

## 10. WebAssembly bridge

### 10a. Checks (#37)

How the data crosses the bridge (see [Technical choices](technical-choices.md), Bridge), the places of the source as the editor counts them (a line, and offsets in UTF-16 code units), `check(source)` and every error of syntax, of names and of structure with its TypeScript type; a function of the core that checks a source and gives either its errors or a checked program, the only one the machine will run (decided on 2026-11-14); tests from TypeScript on the place of an error after accents and an emoji.
