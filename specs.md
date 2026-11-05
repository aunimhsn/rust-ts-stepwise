# Specifications

## Purpose

Stepwise teaches the first steps of programming. The learner writes a small program in a language of its own, close to Python, then watches a robot run it on a grid seen from above, one instruction at a time.

- **Learners**: beginners from high school on (French *première* and *terminale*) and beyond, in class with a teacher or on their own.
- **Goal**: the habits of a developer from the first line: indentation, clear names, small steps, reading an error message and fixing the program. The interface, the language and its messages are in English.
- **Where**: a website, on a computer (no phone or tablet layout). Everything runs in the browser: no account, no server, the progress stays in the learner's browser.
- **Content of version 1**: a series of about twenty levels, from the first `forward()` to functions, a sandbox to program freely, and the manual of the language.
- **Volume of data**: tiny. The levels ship with the website; the browser keeps the progress, the code of each level and a few settings.

## Design principles

- **Look**: flat, clean and calm, seen from above, inspired by the clarity of Shapez 2 (no 3D, no perspective). Every element of the map reads at a glance; color is kept for what carries meaning (paints, errors, success, the current line).
- **Identity**: our own robot, friendly and simple, whose smiling head is the logo of Stepwise. Its look comes from the sheet of the world elements (see [Look of the world](#look-of-the-world)).
- **The robot shows what the program does**: every action is animated, the line being run is highlighted, and nothing happens without the learner seeing it.
- **Tone of the messages**: short, friendly, in the second person, never blaming. An error says what happened, where (line and column), and how to fix it, with the words of the language: *"Expected ':' at the end of the `if` line."*
- **Strict where it builds a habit**: indentation of exactly four spaces, `snake_case` names, a function defined before it is called, a variable declared before it is used. Each rule has its own clear message.

## The language

A small language of our own. It reads like Python, but it isn't Python: `else if` instead of `elif`, `let` to declare a variable, `function` to define one.

```
# Paint a square of side 3
function paint_side(length):
    start_painting(red)
    repeat length:
        forward()
    stop_painting()

let sides = 0
while sides < 4:
    paint_side(3)
    turn_right()
    sides = sides + 1

if front_is_clear():
    forward()
else if left_is_clear():
    turn_left()
else:
    print("I am stuck")
```

### Lines and indentation

- One statement per line. A line ending with `:` opens a block: the lines under it, indented by **exactly four more spaces**, until a line goes back to the level of the opening line.
- Indentation uses spaces only. A tab, or an indentation that isn't four spaces more than the line that opens the block, is an error. The editor inserts four spaces when the Tab key is pressed.
- A block has at least one line.
- `#` starts a comment, up to the end of the line. Empty lines and comment lines are ignored, whatever their indentation.
- A line can't be split in two, even inside parentheses.

### Names

- Variables and functions are written in `snake_case`: lowercase letters, digits and underscores, starting with a letter (`sides`, `paint_side`, `count_2`). Any other name is an error that suggests the `snake_case` version (`paintSide` → `paint_side`).
- The keywords, the built-in functions and the colors are reserved: a variable, a function or a parameter can't take their names.
- Keywords: `let`, `if`, `else`, `while`, `repeat`, `break`, `function`, `return`, `and`, `or`, `not`, `true`, `false`.
- The words a learner may write from another language are reserved too, each with a message that shows the Stepwise way: `elif` (→ `else if`), `def` (→ `function`), `var` and `const` (→ `let`), `for`, `continue`, `pass`, `none`, `null`.

### Values

| Type | Written | Notes |
|---|---|---|
| Whole number | `0`, `42`, `-7` | 64-bit; a result beyond that range stops the program with an error. No decimal numbers in version 1. |
| Boolean | `true`, `false` | The only values a condition accepts: `if 3:` is an error, there is no "truthy" value. |
| Text | `"I am stuck"` | Double quotes, on one line; `\"`, `\\` and `\n` inside. Used by `print`. |
| Color | `red`, `yellow`, `blue` | The three paints of the world: enough for the sensors to tell three cases apart (four with an unpainted cell), few enough to read at a glance and to stay distinct with a red-green color blindness. Their shades are in [Look of the world](#look-of-the-world). |

Lists and decimal numbers come later (see the [backlog](docs/backlog.md)).

### Variables

- `let name = value` declares a variable; `name = value` changes it afterwards. Changing a variable that was never declared is an error that suggests `let`, which catches the typos (`sides = sidse + 1`).
- A variable exists from its `let` to the end of the block where it is declared. Declaring a name already declared in that block or in a block around it is an error.
- A function sees only its parameters and its own variables, never the variables of the main program.

### Expressions

- Arithmetic on whole numbers: `+`, `-`, `*`, `/` (whole division, `7 / 2` is `3`), `%` (remainder). Dividing by zero stops the program with an error.
- `+` also joins two texts. Adding a text and a number is an error: `print` takes several values instead (`print("Sides:", sides)`).
- Comparisons: `==`, `!=` (two values of the same type), `<`, `<=`, `>`, `>=` (whole numbers). Comparisons don't chain: `1 < x < 5` is an error that suggests `1 < x and x < 5`.
- Logic: `and`, `or`, `not`, on booleans, evaluated from left to right and stopping as soon as the result is known.
- Parentheses group as usual and are never required: `if front_is_clear():` and `if (front_is_clear()):` both work.
- Priority, from the strongest: parentheses and calls; `-` and `not` in front of a value; `*`, `/`, `%`; `+`, `-`; comparisons; `and`; `or`.
- `=` where `==` is expected (`if x = 3:`) is an error that suggests `==`.

### Statements

| Statement | Meaning |
|---|---|
| `let name = expression` | Declares a variable. |
| `name = expression` | Changes a declared variable. |
| `if condition:` then `else if condition:` (any number) then `else:` (optional) | Runs the block of the first true condition. |
| `while condition:` | Runs its block as long as the condition is true. |
| `repeat count:` | Runs its block `count` times; `count` is a whole number evaluated once, `0` runs nothing, a negative number is an error. |
| `break` | Leaves the innermost `while` or `repeat`. Outside a loop, an error. |
| `function name(parameter, …):` | Defines a function, at the top level of the program only (not in a block, not in another function). |
| `return` or `return expression` | Leaves the function, with a value or without. Outside a function, an error. |
| A call alone on its line | `forward()`, `paint_side(3)`. |

### Functions

- **A function is defined before it is called**, in the reading order of the program: calling it on a line above its definition is an error found before the program runs (*"`paint_side` is called on line 3 but defined on line 12. Move its definition above."*). There is no hoisting.
- A function can call itself (recursion), since it is defined when its body runs. Two functions can't call each other.
- A call gives exactly as many values as the function has parameters; another number is an error found before the program runs.
- A function that ends without `return expression` gives back no value: using its result in an expression stops the program with an error.
- Calls nest up to 200 deep; beyond, the program stops with an error (a recursion that never ends).
- A function named without parentheses (`forward`), or a variable called like a function (`sides()`), is an error that shows the right way.

### Built-in functions

The robot stands on one cell and faces one of the four directions. It can carry one item at a time. It holds a brush and an eraser, each up or down, as modes: once down, the tool works on every cell the robot moves into, until it is lifted. At most one of them is down: putting one down lifts the other. Turning on its cell paints or erases nothing more.

| Category | Function | What it does | Error when |
|---|---|---|---|
| Move | `forward()` | Moves one cell in front. | A wall or the edge of the map is in front: the robot bumps and the program stops. |
| Move | `turn_left()`, `turn_right()` | Turns a quarter turn on its cell. | Never. |
| Paint | `start_painting(color)` | Puts the brush down with the color: paints its cell at once, then every cell it moves into and the passage it crosses, over any paint already there. Called while painting, it changes the color. | Never. |
| Paint | `stop_painting()` | Lifts the brush: the robot moves on without painting. | Never (nothing happens when the brush is already up). |
| Paint | `start_erasing()` | Puts the eraser down: removes the paint of its cell at once, then of every cell it moves into, with the painted passages of these cells. | Never (an unpainted cell stays unpainted). |
| Paint | `stop_erasing()` | Lifts the eraser. | Never (nothing happens when the eraser is already up). |
| Carry | `pick_up()` | Picks up the item of its cell. | No item on the cell, or the robot already carries one. |
| Carry | `drop()` | Drops the item it carries on its cell. | The robot carries nothing, or the cell already has an item. |
| Sense | `front_is_clear()`, `left_is_clear()`, `right_is_clear()` | `true` when the cell in that direction is a floor cell of the map. | Never. |
| Sense | `here_is(color)` | `true` when its cell has that paint. | Never. |
| Sense | `front_is(color)`, `left_is(color)`, `right_is(color)` | `true` when the cell in that direction has that paint (`false` for a wall). | Never. |
| Sense | `item_here()`, `item_in_front()` | `true` when its cell, or the cell in front, has an item. | Never. |
| Sense | `is_carrying()` | `true` when the robot carries an item. | Never. |
| Sense | `is_painting()`, `is_erasing()` | `true` when the brush, or the eraser, is down. | Never. |
| Write | `print(value, …)` | Writes its values to the console, separated by spaces, on one line. | Never. |

### Checks before running

The program is checked as the learner types (a short moment after the last key) and again when **Run** is pressed. The checks find:

- the syntax errors: the first one only, since the rest of the program can't be read reliably after it;
- once the syntax is right, every error of names and structure: unknown names, variables used before their `let`, functions called above their definition, a wrong number of values in a call, `break` or `return` out of place, a function defined in a block, a reserved name, a name not in `snake_case`.

Every error found is underlined in the editor, with its message on hover, and listed in the console. **Run** doesn't start a program that has errors: it shows them in the console.

The errors that depend on the values (a wall in front, a division by zero, a condition that isn't a boolean) can only be found while the program runs: they stop it and are shown the same way.

## Features

### Screens

| Screen | Content |
|---|---|
| Home | The path through the levels, grouped by chapter, each locked, open or done; the level chosen; the sandbox, the manual and the settings (see [Home screen](#home-screen)). |
| Level | The code editor with its controls, the map, the console and the variables panel; the goal opens from the header, the legend from a button on the map (see [Level screen](#level-screen)). |
| Sandbox | The level screen without goal, on an open map. |
| Manual | The manual of the language. |

The layout of each screen is decided with the wireframes, then the mock-up. The wireframes, in [`docs/wireframes/`](docs/wireframes/), only fixed the first layout: once the mock-up is approved, it becomes the reference.

### Home screen

Wireframe: [`docs/wireframes/home-screen.excalidraw`](docs/wireframes/home-screen.excalidraw). A path through every level on the left, the level chosen on the right, under a header of one line:

- **Header**: the logo and the name; in the middle, *"17 of 20 levels done"*; on the right, **Sandbox**, **Manual** and **Settings**. The same thin bar as on the level screen fills along its bottom with the levels done.
- **Path**, on the left: one stone per level, numbered, linked by a path that runs in rows, left to right then back, with a rounded half-turn at the end of each row. The path is painted up to the level to play and drawn as a ghost after it. A stone is filled when its level is done, bold when it is the level to play, dashed when it is locked. The robot stands on the level chosen, astride the top of its stone, and moves to the stone clicked. Hovering a stone shows its title; a click chooses its level and shows it on the right, and only **Play** opens it, so that the learner reads the goal first.
- **Chapters**: a light band behind the stones of each chapter, with its name and its idea (*"Loop until · while, not, break"*). A chapter never wraps around a turn of the path.
- **Maps of a level**: under each stone, one small square per map, astride the bottom of the stone, green once the map has passed, grey until then, with no mark for a map that failed. A done level keeps its squares green, even when its code has changed since.
- **Level chosen**, on the right: its chapter, its number and title, a scene of its map, its goal, its maps with their state, and one button. The screen opens with the level to play chosen.
  - **Level to play**: the starting map with its goal marks and the robot waiting (it blinks, looks around), never the solution; **Play**.
  - **Level done**: the learner's own program replayed on its first map; **Play again**.
  - **Level locked**: the starting map, still, and *"Finish 18. Paint a square to open this level."*; under it, *"Have a code?"*, a field and **Open** (see [Level codes](#levels-and-progression)); the button is greyed.
  - The panel of a level open or done shows its code (*"Code of this level: BLUE-FOX"*).
- **Large screen**: the path and the panel keep their size, centered in the window; the room left serves the levels added later before the path scrolls.
- **More levels**: the path is laid out from the list of levels, so that adding levels or chapters needs no new design; when they no longer fit, the path scrolls.

### Level screen

Wireframe: [`docs/wireframes/level-screen.excalidraw`](docs/wireframes/level-screen.excalidraw). The code on the left, the world on the right, under a header of one line:

- **Header**: the logo and the name; the menus **Level**, **Edit**, **Run**, **Help**; in the middle, the place of the level (`◎ Functions › 18. Paint a square`), a button that opens its goal; on the right, the maps of the level (see [Variants](#levels-and-progression)). A thin bar along the bottom of the header fills with the levels done; on hover, a discreet *"17 of 20 levels done"*.
- **Editor**, on the left, on the whole height: the code with its line numbers and breakpoints; three square buttons fixed in its bottom right corner, which don't scroll with the code: **Run** / **Pause** / **Resume** (one button, ▶ or ⏸), **Step** (an arrow down onto a dot, as the *step into* of the developer tools) and **Reset** (↻). They have no text: each one names itself, with its shortcut, on hover. They grow pale while the learner types and come back as soon as the mouse moves; the code can scroll past its last line so that no line stays under them.
- **Status bar**, under the code, as in VS Code: the state of the program (`Ready`, `Running`, `Paused at line 5 · Reset to edit`, `Error at line 3`), the number of errors when there are some (a click shows the first one in the console), the steps run (`47 steps`), the speed (five small squares to click), the position of the cursor (`Ln 7, Col 12`).
- **Map**, on the right, on top: the whole map, always; its cells grow or shrink to fill the block, up to a maximum size (about 64 pixels, tuned with the mock-up), the map centered in its block. No zoom, no scrolling. The button **Legend** sits in its top right corner (see [Map](#map)).
- **Console** and **Variables**, under the map, about 250 pixels high: two tabs; from about 1600 pixels wide, side by side without tabs.
- **Widths**: the editor takes half the window, and stops growing at about 80 characters (about two fifths of a 1920-pixel screen); the map and the panels take the rest. The text keeps its size on a large screen, where more lines show; the zoom of the browser enlarges everything for a projector.

The menus give every command of the screen, with its shortcut, so that the learner meets the shortcuts of a code editor:

| Menu | Commands |
|---|---|
| **Level** | Goal, Previous level, Next level (greyed until the level is done), All levels (the home screen), Sandbox, Restore starting code…, Settings. |
| **Edit** | Undo, Redo, Commands (the command menu), Toggle breakpoint, Remove all breakpoints. |
| **Run** | Run / Pause / Resume, Step, Reset, Speed. |
| **Help** | Manual, Look up in the manual (F1, the word at the cursor), Legend of the map, Keyboard shortcuts. |

The previous and the next level are only in the menu, so that a click by mistake never changes the level.

**Goal**: a bubble under the place of the level, which doesn't block the screen: the learner can keep it open while typing, and Escape or a click elsewhere closes it. It shows the title, the goal, the elements new on the map, **Show hint**, the links to the manual, and **Let's go**, which closes it. It opens by itself the first time the learner opens the level.

The window needs at least 1280 × 720 pixels: below, a message replaces the screen (*"Stepwise needs a window at least 1280 pixels wide."*) and goes away once the window is wide enough again, without losing anything.

### Levels and progression

- The levels are grouped in chapters, each introducing one idea. The plan below is a starting point; the levels are designed in their own roadmap steps. About 20 levels in 8 chapters for the first version: a range rather than a fixed number, which follows the ideas found while designing the levels.

  | Chapter | Idea | Levels (about) |
  |---|---|---|
  | First steps | `forward()`, `turn_left()`, `turn_right()`, the order of the instructions | 3 |
  | Repeat | `repeat` and its block, the indentation | 3 |
  | Paint | the brush and the eraser as modes (`start_painting()`, `stop_painting()`, `start_erasing()`, `stop_erasing()`), the colors | 2 |
  | Decide | `if`, `else if`, `else`, the sensors | 3 |
  | Loop until | `while`, `not`, `break` | 2 |
  | Carry | `pick_up()`, `drop()`, `is_carrying()` | 2 |
  | Remember | `let`, changing a variable, `print` | 2 |
  | Functions | `function`, parameters, `return` | 3 |

- A level has a title, a goal written in a few sentences, its map, the code it starts with, and an optional hint, hidden until the learner asks for it. Each level links to the sections of the manual it needs.
- **Variants**: a level may have up to three maps, the same task on different ground (a wall further away, another number of items), so that a program written for one map only doesn't pass. The learner switches between them with the buttons **1**, **2**, **3** on the right of the header, above the map, hidden when the level has one map; the shown map has a full background. **Run** runs the shown variant. A bar under each number turns green once its variant has passed, and stays grey until then; the level is done once every variant has passed with the same code. Changing the code turns the bars grey again.
- The levels are numbered from the first to the last, across the chapters (`18. Paint a square`). The number is computed from their order, never saved: the progress is saved under a stable name of each level, so that a level added later doesn't mix it up.
- The levels open in order: a level done opens the next one. A done level stays playable. Each level must be earned: there is no setting that unlocks every level; a learner who is stuck has the hint, then the teacher.
- **Level codes**, as the passwords of the old console games: each level has a code of two English words (`GREEN-OWL`), easy to write and to dictate, unrelated to its title so that it can't be guessed, and tied to the stable name of the level, not to its number, so that a level added later changes no code. Entering the code of a level opens it and every level before it, without marking them done. The success panel gives the code of the next level, so that the learner can go on from another computer (the progress lives in one browser); the home screen shows the code of every level open or done. The teacher gives a code to open a level for the whole class, or the code of the last level for a demonstration; the list of the codes is in a guide for the teachers in `docs/`, readable in the repository: the codes add friction, not security.
- **Goals** of a level, all checked when the program ends without error:
  - the robot stands on a given cell;
  - given cells have given paints (and, when the level says so, no other cell is painted);
  - items lie on given cells;
  - the robot carries an item, or carries none.
- **New elements**: each element of the map (see [Map](#map)) is introduced by the first level that uses it: the goal of this level names it, links to the section **The map** of the manual, and the legend of the level marks it as new. There is no tutorial level showing every element at once: a chapter brings one idea, and an element waits for the chapter that needs it.
- When the program ends and every goal is met, the variant passes; once every variant has, a success panel offers the next level. When a goal isn't met, the console says which one, and the cells concerned are marked on the map.
- **Success panel** (wireframe: [`docs/wireframes/success-panel.excalidraw`](docs/wireframes/success-panel.excalidraw)): in the middle of the screen, over a veil, once the robot has hopped (about one second). The robot, *"Level done!"*, *"18. Paint a square · its 3 maps passed"*; what the learner used (*"function, parameters, while"*), the number of lines and of steps, never a score; the code of the next level (*"Code of the next level: GREEN-OWL · Write it down to go on from another computer."*); **Next level: 19. Two squares** (Enter), **Stay on this level** (Escape: back to the code, which can be changed) and **All levels**. When a map passes and others remain, there is no panel: the console says *"Map 1 passed. Try map 2."* and the bar of that map turns green.

### Code editor

- The code of each level is saved in the browser as the learner types, and comes back when they return to the level. **Level › Restore starting code…** puts the level back in its first state, after a confirmation: the starting code, and no breakpoint (a breakpoint belongs to a line of the code, which no longer exists). The confirmations of **Restore starting code** and **Reset progress** are small panels in the middle of the screen, over a veil (wireframe: [`docs/wireframes/overlays.excalidraw`](docs/wireframes/overlays.excalidraw)): the question, what is lost, *"This can't be undone."*, then **Cancel** and the action, which is the main button: Enter confirms, Escape cancels, as everywhere. Its message doesn't name the breakpoints: *"Your code will be replaced by the starting code of the level, as the first time. This can't be undone."*
- Syntax highlighting of the keywords, the names, the numbers, the texts, the colors and the comments.
- Tab inserts four spaces; a new line after a line ending with `:` is indented four spaces more; Backspace at the start of an indentation removes four spaces.
- Errors underlined, with their message on hover (see [Checks before running](#checks-before-running)).
- **Ctrl+click** (Cmd+click on a Mac), as in a code editor: holding Ctrl underlines the name under the mouse; a click on a built-in function or a keyword opens its entry of the manual in a drawer (see [Manual](#manual)), and a click on a function of the program puts the cursor on its definition. **F1** does the same for the word at the cursor, for the keyboard. **Help › Keyboard shortcuts** and the section **Your first program** of the manual mention it.
- A thin vertical line at the 80th column. A longer line is underlined with dots, as advice and not as an error, with *"Lines are easier to read under 80 characters."* on hover; the program still runs. The manual explains it in **Conventions**.
- **Breakpoints**: a click in the margin, left of a line number, sets or removes a breakpoint on that line. They are saved with the code of the level.
- While a program runs or is paused, the editor can't be changed: the status bar says so, and when the learner tries to type, it lights up briefly with *"Reset the program to edit your code."* A program stopped by an error has ended: the editor can be changed at once, without **Reset**, so that the learner fixes the line of the error; the map and the variables panel keep the state at the error until the next **Run** or **Reset**.

### Command menu

**Ctrl+Space** in the editor opens a menu at the cursor, to find a command without knowing its name by heart. It opens only on Ctrl+Space, never by itself while typing, so that the learner takes the habit of asking for it. Wireframe: [`docs/wireframes/command-menu.excalidraw`](docs/wireframes/command-menu.excalidraw): one fixed panel of three columns, as the cascade of RoboMind but without submenus: the categories, always in view; the commands of the chosen category; the details of the chosen command.

- the commands grouped by category: **Move**, **Paint**, **Carry**, **Sense**, **Write** (the built-in functions), **Colors**, **Control** (`if`, `else if`, `else`, `while`, `repeat`, `break`), **Variables** (`let`), **Functions** (`function`, `return`), and **Your names** (the variables and functions of the program that can be used at the cursor);
- typing filters the commands (`paint` keeps `start_painting()`, `stop_painting()` and `is_painting()`): the categories without result grow pale, the others show their number of results, and the middle column lists every result under the name of its category; ↑ ↓ move in the commands, ← → change the category, Tab, Enter or a click inserts the command and closes the menu, Escape closes it (Tab inserts four spaces only when the menu is closed);
- next to the chosen command, its one-line description, an example from the manual and **Read more**, which opens its entry of the manual;
- a statement that opens a block (`if`, `while`, `repeat`, `function`) is inserted with its `:` and its first indented line, the cursor on what to fill in (`if |:`);

### Running a program

| Control | What it does |
|---|---|
| **Run** | Checks the program, puts the map back to its starting state, then runs the program at the chosen speed. |
| **Pause** / **Resume** | Pauses the program before its next step; resumes it. |
| **Step** | Runs one step, then pauses: it goes into the functions called (the *step into* of the debuggers; there is no *step over* or *step out*). Available when paused or stopped (from stopped, it starts the program paused on its first step). |
| **Reset** | Ends the program if it runs or is paused, and puts the map back to its starting state. The editor can be changed again. The code stays as it is: its hover says *"Puts the map back. Your code stays."* It is the only command named **Reset** near the code, so that it is never confused with **Restore starting code…**, which replaces the code. |
| **Speed** | In the status bar. Five speeds, from one step per second to **Instant**, which runs as many steps as the browser allows without animation. Kept between sessions. |

- A **step** is one line run: a statement, the condition of a loop checked again, a call entered. The line of the next step is highlighted in the editor.
- A program that never ends (an infinite loop) keeps running, visibly, until **Pause** or **Reset**: the robot keeps moving or turning, the highlighted line keeps changing, and the step counter of the status bar keeps rising, even for a loop that doesn't move the robot. The page never freezes, even at **Instant**.
- A breakpoint pauses the program before it runs that line.
- The robot's actions are animated at the chosen speed: moving, turning, painting, picking up, dropping, and a bump against a wall that stops the program.
- The robot picks up and drops an item with a pincer that comes out of its front, then goes back in; the item it carries lies in the bay at its back.
- At the end of a run, the robot shows the result: when every goal is met, its eyes light up in yellow and it hops; when a goal isn't met, its eyes dim, it shakes its head, and the cells of the goals not met get a red flag (see [Levels and progression](#levels-and-progression)).
- **Sounds**, when the setting is on: a short, soft sound when a run succeeds, another when it fails (never scolding), and one when the robot bumps into a wall; none at each step. The browser plays them only after a click, such as **Run**.
- There is no **Stop**: **Pause** freezes the program to look at the map and the variables, **Reset** ends it.
- The keyboard shortcuts of the controls are decided with the mock-up.

### Console

- The lines written by `print`, in order.
- The errors, before running or while running, with their line and column: a click on an error puts the cursor on its position in the editor.
- The messages of the level: the goals not met, the success.
- Cleared at each **Run**.
- **Run** shows the console tab.

### Variables panel

Its tab opens by itself when the program pauses (by **Pause**, **Step** or a breakpoint). When the program stops on an error, the console stays shown, with the error, and a dot on the tab **Variables** says that it holds the state at that moment. Otherwise, the tab says *"Pause the program to see its variables."* It shows:

- the variables of the function being run (or of the main program), with their values; the ones changed by the last step stand out;
- the calls in progress, from the main program to the current function, each with its line; a click on one shows its variables.

### Manual

The manual of the Stepwise language, on its own page, reachable from every screen. Its sections, each with short examples the learner can copy:

1. **Your first program**: one instruction per line, run from top to bottom.
2. **Commands**: every built-in function, by category (Move, Paint, Carry, Sense, Write), with what it does, its errors and an example; and the colors.
3. **Indentation and blocks**: the `:` and the four spaces.
4. **Conditions**: `if`, `else if`, `else`, the comparisons, `and`, `or`, `not`.
5. **Loops**: `repeat`, `while`, `break`, and how to stop a program that never ends.
6. **Variables**: `let`, changing a value, where a variable exists.
7. **Functions**: `function`, the parameters, `return`, why a function is defined before it is called.
8. **Conventions**: the `snake_case` names, choosing a clear name, the four spaces, one idea per function, the comments; what Stepwise checks and what is only good practice.
9. **Errors**: how to read an error message, with the most common ones and how to fix them.
10. **The map**: every element of the world with its drawing and one sentence: the floor, the wall, the paints, the item, the goal marks, the flag of a goal not met, the robot.

The manual is searchable (a search field filters its sections and commands), and each section has its own address, so that a level, the command menu and an error message can link to it.

Wireframe: [`docs/wireframes/manual.excalidraw`](docs/wireframes/manual.excalidraw). A page of documentation, laid out as the usual ones of programming languages, so that the learner also learns to read them:

- **Header**: the logo and the name, then *"› Manual"*; on the right, when the manual was opened from a level, **Back to 18. Paint a square**. There is no menu bar.
- **Sections**, on the left: the search field, then the ten sections; the open one is marked, with its categories under it (*Move*, *Paint*…).
- **Search**: typing filters the list on the left into results grouped by section and category (`paint` keeps *Commands › Paint* with `start_painting(color)`, `stop_painting()` and `is_painting()`, *Commands › Colors* and *The map › Paints*), with their number; the arrow keys move in them, Enter opens the chosen one, Escape clears the search.
- **Content**, in the middle: the section, then each command with what it does, an example with **Copy**, and a small map where the example plays.
- **On this page**, on the right: the categories or the headings of the section; a click scrolls smoothly to its anchor, and the one in view is marked while scrolling.
- **Large screen**: the column of text stays under about 760 pixels wide, to stay readable; the sections on the left and **On this page** on the right stay beside it, the whole centered in the window.
- **From a level**: every link to the manual (the goal, **Help › Manual**, **Read more** in the command menu, an error) opens it in a tab of its own, always the same one, at the right section, so that the code and the map stay open in the tab of the level. **Back to 18. Paint a square** closes that tab and brings the level back.
- **Drawer**: in the editor, Ctrl+click (Cmd+click on a Mac) on a built-in function or a keyword, or F1 on the word at the cursor, opens the entry of the manual in a drawer on the right of the level screen, over the map and the panels, the code still in view: the search, the place in the manual (*Manual › 2. Commands › Move*), the entry, and **Full page**, which opens the page above in its tab. Escape, **×** or **Run** closes it, so that the map shows while the program runs.

### Sandbox

An open map with a few walls, paints and items, without goal. The code is saved like a level's; **Run**, the console and the variables panel work the same way. Wireframe: the frame *Sandbox* of [`docs/wireframes/level-screen.excalidraw`](docs/wireframes/level-screen.excalidraw). It is the level screen without what belongs to a goal: in the middle of the header, *"◇ Sandbox"*, which isn't a button; no maps on the right of the header, no progress bar; no goal marks on the map; the menu **Level** keeps **All levels**, **Sandbox** (the current screen), **Restore starting code…** and **Settings**. Editing the map of the sandbox, and a level editor, come later (see the [backlog](docs/backlog.md)).

### Settings

A panel in the middle of the screen, over a veil, opened from the header of the home screen or **Level › Settings** (wireframe: [`docs/wireframes/overlays.excalidraw`](docs/wireframes/overlays.excalidraw)). A change applies at once and is saved, without a **Save** button; Escape, **×** or a click on the veil closes it.

- **Sounds**: on or off (off by default, so that a classroom stays quiet).
- **Reset progress**: puts every level back in its first state (as **Restore starting code**, its breakpoints with its code) and clears the levels done and the levels opened by a code, after a confirmation; the settings stay. The messages only say *"the levels done and the code of every level"*: the breakpoints go with the code, without being named.

### Interface language

The interface is in English. Every text of the interface, the error messages and the manual included, comes from translation files (English is the reference), so that a French translation can come later without changing the code.

## Map

- A grid of cells, at most 16 columns and 12 rows. The coordinates start at the top left cell, `x` growing to the right and `y` downward.
- A cell is a **wall** or a **floor**. A floor cell may hold one paint (one of the three colors) and one item.
- The paint also lies on the **passages** between two cells: a passage is painted when the robot moves from one cell to the other with its brush down, and loses its paint when the eraser passes on one of its cells. The map draws the paint as a stroke through the centers of the cells, joining two cells only along a painted passage, so that the stroke follows the path of the robot; it stops, rounded, in the middle of the cells at its ends. The sensors and the goals only look at the cells.
- One kind of item in version 1 (its look comes from the sheet of the world elements).
- The goal cells of a level are marked on the map (where to stand, which color where, where to put the items), so that the learner sees the target before writing code.
- Outside the grid counts as a wall.
- **Legend**: the button **Legend**, in the top right corner of the map, or **Help › Legend of the map** opens the legend of the level in a bubble under the button (wireframe: [`docs/wireframes/overlays.excalidraw`](docs/wireframes/overlays.excalidraw)), which blocks nothing, like the goal: Escape, **×** or a click elsewhere closes it. The button carries a dot while the map has new elements not yet seen in the legend. The legend shows only the elements present on its map, each with its drawing and one sentence, the new ones marked as new, and a link to the section **The map** of the manual.

### Look of the world

The sheet of the world elements, [`docs/mockups/world-elements.html`](docs/mockups/world-elements.html), is the reference of every element: flat, seen from above, in the Daylight style. Each element is an SVG file of `docs/mockups/world-elements/daylight/`, reused as it is by the application, which stacks them in a cell; only the paint strokes are drawn by the map.

| Element | Look | Shades |
|---|---|---|
| Floor | A light gray cell with a thin line around it, so that the cells can be counted. | `#e1e5ea`, line `#cad1da` |
| Wall | A plain slate cell; the walls meet as one mass. A wall below a floor cell has a light rim at its top, a wall above a floor cell a darker edge at its bottom: a flat hint of height. | `#5d687a`, rim `#7d899c`, edge `#3f4857` |
| Paints | A stroke 28/64 of a cell wide through the centers of the cells, along the painted passages (see [Map](#map)), rounded at its ends. The red is darker and the yellow lighter so that they stay apart in a red-green color blindness. | `red` scarlet `#dc2f34`, `yellow` bright amber `#ffb300`, `blue` cobalt `#2459d6` |
| Item | A teal hexagon with a dark center. | `#5fd0b0`, ink `#222935` |
| Goal: paint | The stroke to paint, in its color at 35 % opacity; a cell painted with the right color hides it. | the paint |
| Goal: item | A slot of the shape of the item, a recess in the floor. | `#c3cad4`, rim `#a9b3c0` |
| Goal: where to stand | A large square slot, in which the robot fits. | as the item slot |
| Goal not met | A small red flag planted in the corner of the cell, once a run ends without meeting it. | `#c2255c` (may follow the error color of the design system) |
| Robot | White, with two tracks, a dark visor with two cyan eyes at its front (its direction), and a bay at its back that holds the item it carries. Its pincer comes out of its front only while it picks up or drops an item. At the end of a run, its eyes light up in yellow when every goal is met, and dim when not. | body `#ffffff`, ink `#222935`, eyes `#3fc3ff`, bay `#d5dbe3`, lit `#ffd23f`, dim `#7d8798` |
| Logo | The head of the robot with happy eyes and a smile, carried up by a stroke of yellow paint that rises above it as a chevron. | the robot, the yellow paint |
| Name | `stepwise` in lowercase, in Fredoka 600 (SIL Open Font License), beside the logo: the middle of the head of the robot meets the middle of the lowercase letters, raised by 0.011 em; the name starts 0.18 em after the head, whose height is 1.25 times the capitals. | the ink of the page |

## Local files

Stepwise has no files of its own on the learner's computer: everything is kept in the browser's `localStorage`, under keys starting with `stepwise.`, with a version number.

| Data | Content |
|---|---|
| Progress | The levels done, the levels opened by a code, and the variants passed with their code. |
| Code | The code and the breakpoints of each level and of the sandbox. |
| Settings | The speed, **Sounds**. |

The levels and the manual are files of the website, written in the repository.

## Error handling

| Situation | Behavior |
|---|---|
| A syntax or check error in the program | Underlined in the editor, listed in the console with its line and column; **Run** doesn't start. |
| An error while the program runs | The program stops; the error is shown in the console with its line, the line is marked in the editor, the variables panel shows the state at that moment. The code can be changed at once, without **Reset**. |
| An infinite loop | The program keeps running visibly until **Pause** or **Reset**; the page stays responsive. |
| A recursion that never ends | Stopped at 200 nested calls with an error. |
| The browser refuses `localStorage` (private browsing, storage full or blocked) | Stepwise works; a banner says *"Your progress can't be saved in this browser."* |
| Saved data that can't be read (damaged, or of an unknown version) | Ignored: the defaults are used, without error; the bad data stays until the next save replaces it. |
| A level file that isn't valid | The level is shown as unavailable in its chapter; the other levels work. The tests check every level file, so this shouldn't reach the website. |
| The WebAssembly module can't load (very old browser) | A message replaces the screen: *"Stepwise needs a recent browser."* |
| A window narrower than 1280 pixels | A message replaces the screen until the window is wide enough. |
