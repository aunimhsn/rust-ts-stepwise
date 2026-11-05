# Backlog

The ideas, polish and edge cases found along the way that the current step doesn't need. They stay here, out of [roadmap.md](../roadmap.md), until the owner makes one of them a step.

## Language

- Lists, and a `for` loop over them.
- Decimal numbers.
- Two functions that call each other (needs a way to declare a function before its body, or hoisting).

## Interface

- The French translation of the interface, the levels and the manual.
- The description of a built-in function on hover in the editor.
- A phone or tablet layout.
- The home screen in the mock-up (step 3): the path painted up to the level to play and drawn as a ghost after it, as the paint and the goal marks of the map.
- The robot of the home screen walking along the path to the stone clicked, or another animation of the choice; to tune with the mock-up.
- The scene of the level to play on the home screen, without giving the solution: a teaser of its goals, the robot saying them, or a funny moment (the robot dozing off, then *"Ah, functions!"*); to discuss with the mock-up.
- The robot as a mascot in the interface, with expressions (a smile, a wink, a level up, a graduation cap) on the success panel, the home screen or the end of a chapter; drafts in the history of step 1b of `docs/mockups/world-elements/`. First, with the mock-up of the success panel (step 3): a smile, or a small animation of the head of the robot as the panel appears.
- A setting that draws a pattern inside each paint stroke (dots, stripes…), for learners with a color blindness.
- A divider to drag between the code and the map, as in VS Code: interesting, but it needs a design of its own (limits, double-click to reset, the size saved) and its integration in every screen.
- A dark mode: an idea to discuss, not a plan. A learner used to it would read the map projected in class differently, since in the dark palette tried on the sheet of the world elements (Ink, in the history of `docs/mockups/world-elements/`) the walls are lighter than the floor, the opposite of the light one.

## Content

- Editing the map of the sandbox.
- A level editor, and sharing a level as a file.
- More kinds of items, and elements such as doors or switches.
- More paints than `red`, `yellow` and `blue`, if a level needs them.
- The same map solved twice, the second time with a constraint (e.g. first with `forward()` written out, then with a loop and at most a given number of lines), so that the learner feels what a control structure brings; close to the challenges of the shortest code.

## Distribution

- A paid version on Steam, once version 1 has been tried by real learners: 60 to 100 levels with optional challenges (fewest steps, shortest code), a desktop application built with Tauri around the same Rust core and Svelte interface, Steam achievements, the Steam Direct fee and a store page. The free website would stay as its demo; the levels, drawings and sounds of the paid version could take a license of their own, since MIT and Apache let anyone rebuild the game.
