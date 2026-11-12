# Backlog

The ideas, polish and edge cases found along the way that the current step doesn't need. They stay here, out of [roadmap.md](../roadmap.md), until the owner makes one of them a step.

## Language

- Lists, and a `for` loop over them.
- Decimal numbers.
- Two functions that call each other (needs a way to declare a function before its body, or hoisting).
- Messages of their own, rather than *unexpected character*, for `1.5` (no decimal numbers in version 1), `;` (one statement per line) and the typographic quotes `“ ”` pasted from a document (seen at step 5); the error already carries the character, so this may be done in the messages alone.

- A function written in another function that uses a variable or a parameter of the function around it: today an unknown name, while the same mistake in a block of the main program is a variable the function doesn't see. And a variable used after a block that held a function declaring the same name: the `let` given is the one of the block, not the closest above. Both already have the error of a function defined in a block, and go away with it (seen in the review of step 7b).
- A type `Checked` that only `check` gives and that `Machine::new` asks for, so that a program can't be run without its checks: to weigh at step 10, whose bridge is the one caller (seen in the review of step 9a).
- The result of a built-in function that gives back no value, found by the checks before running rather than while the program runs (`if forward():`, `let steps = turn_left()`): the checker knows these functions, and a beginner may well write the first one to ask whether the robot can move. Today the function runs, then the program stops (seen at step 9).
- The words of Python written with a capital, `True`, `False` and `None`: today names that aren't in `snake_case`, so the checker recalls the rule of the names instead of showing `true`, `false` or that there is no empty value (seen at step 6a).

## Interface

- The French translation of the interface, the levels and the manual; the comments of the starting code of the levels too, which stay in English in the level files for now (seen at step 8b).
- The code normalized to NFC in the editor (step 16): a name pasted with its accents decomposed (`côté` written `co` + U+0302 + …) is cut before the accent by the lexer, which reads the combining mark as an unexpected character (seen in the review of step 5).
- The description of a built-in function on hover in the editor.
- A phone or tablet layout.
- The home screen in the mock-up (step 3): the path painted up to the level to play and drawn as a ghost after it, as the paint and the goal marks of the map.
- The robot of the home screen walking along the path to the stone clicked, or another animation of the choice; to tune with the mock-up.
- The scene of the level to play on the home screen, without giving the solution: a teaser of its goals, the robot saying them, or a funny moment (the robot dozing off, then *"Ah, functions!"*); to discuss with the mock-up.
- The robot as a mascot in the interface, with expressions (a smile, a wink, a level up, a graduation cap) on the success panel, the home screen or the end of a chapter; drafts in the history of step 1b of `docs/mockups/world-elements/`.
- A setting that draws a pattern inside each paint stroke (dots, stripes…), for learners with a color blindness.
- A mouse cursor of our own on the map or the buttons, such as a small pincer of the robot, drawn by hand; never in the editor, whose I-beam stays the one of the system, and it may take away the larger or contrasted cursors that some learners set.
- A divider to drag between the code and the map, as in VS Code: interesting, but it needs a design of its own (limits, double-click to reset, the size saved) and its integration in every screen.
- A choice **Follow the system** for the theme of the settings: secondary for a website.
- **Ctrl+K Ctrl+C** and **Ctrl+K Ctrl+U** to comment and uncomment, as in VS Code, if learners on a QWERTY keyboard miss a shortcut without punctuation. Set aside for now: two sequences to learn instead of one toggle, and a pause inside a sequence lets Ctrl+U reach the browser, which opens the source of the page.
- Export and import of the progress in a file, so that a learner can go on from another computer without a teacher (the level codes are for the teacher alone). A file could circulate between classmates too, but far less easily than a word dictated.
- The console of step 17 keeping only its last lines (a few hundred): a `print` in a loop that never ends would otherwise fill the memory of the page, however fast the core is (seen at step 9).
- The running of step 17: a `step()` that throws (an error of the bridge, not of the program) must stop the loop of the animation frames and say it, rather than leave the program shown as running; the speed kept as its value in steps per second rather than its place in the list, so that the speed saved between sessions survives a change of the list (seen in the review of the probe of step 4c).

## Content

- Editing the map of the sandbox.
- A level editor, and sharing a level as a file.
- More kinds of items, and elements such as doors or switches.
- More paints than `red`, `yellow` and `blue`, if a level needs them.
- The same map solved twice, the second time with a constraint (e.g. first with `forward()` written out, then with a loop and at most a given number of lines), so that the learner feels what a control structure brings; close to the challenges of the shortest code.

## Tooling

- An `animation-reviewer` subagent, to decide at step 11 (animations and sounds): like `code-reviewer`, it recommends without changing anything, once per pull request that touches the animations. It checks what can be measured, the taste staying with the owner: the time an element is really visible for its size, a hover that never restarts by itself, the stacking order, the state at rest in both themes, a fallback for each animation when the learner asked for less motion, and the match with `specs.md`.
- A recommendation of the model and effort for each kind of step, to discuss at the retrospective (and to write in `process.md` and the template if it holds). Every step so far ran on Opus 5.5 at high effort; the steps of architecture (the workspace, the interpreter, the bridge), the audits of quality and the refactorings could run on Fable 5.2, if it brings much in good practices and in quality for its cost. Tried from step 4: the model of each step is marked in `roadmap.md`, and step 4a is reviewed by `code-reviewer` on both models; see the lessons of `process.md`.
- Measure the size and the speed of the WebAssembly module again once the interpreter exists (step 9 or 10): `opt-level` 3 was the smallest on the probe of step 4c, a code too small to be sure it stays so, and `wasm-opt` could matter for a larger module (see [Technical choices](technical-choices.md), WebAssembly size).
- The level files read in TypeScript (`smol-toml`) and handed to the core through the bridge, if the weight of the reader in the WebAssembly module becomes a problem: `serde` and `toml` add about 100 kB gzipped to it (measured at step 8b, see [Technical choices](technical-choices.md), Levels). The files would not change; the cost is a second reader of the format.
- A JSON Schema of the level files, so that VS Code (with Even Better TOML) completes and checks their fields while they are written.

## Distribution

- A paid version on Steam, once version 1 has been tried by real learners: 60 to 100 levels with optional challenges (fewest steps, shortest code), a desktop application built with Tauri around the same Rust core and Svelte interface, Steam achievements, the Steam Direct fee and a store page. The free website would stay as its demo; the levels, drawings and sounds of the paid version could take a license of their own, since MIT and Apache let anyone rebuild the game.
