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
- **Identity**: our own robot, friendly and simple, whose head is the logo of Stepwise. Its look comes from the sheet of the world elements.
- **The robot shows what the program does**: every action is animated, the line being run is highlighted, and nothing happens without the learner seeing it.
- **Tone of the messages**: short, friendly, in the second person, never blaming. An error says what happened, where (line and column), and how to fix it, with the words of the language: *"Expected ':' at the end of the `if` line."*
- **Strict where it builds a habit**: indentation of exactly four spaces, `snake_case` names, a function defined before it is called, a variable declared before it is used. Each rule has its own clear message.

## The language

A small language of our own. It reads like Python, but it isn't Python: `else if` instead of `elif`, `let` to declare a variable, `function` to define one.

```
# Paint a square of side 3
function paint_side(length):
    repeat length:
        paint(red)
        forward()

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
| Color | `red`, `yellow`, `green`, `blue`, `purple` | The five paints of the world (their final names and shades come from the sheet of the world elements). |

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

The robot stands on one cell and faces one of the four directions. It can carry one item at a time.

| Category | Function | What it does | Error when |
|---|---|---|---|
| Move | `forward()` | Moves one cell in front. | A wall or the edge of the map is in front: the robot bumps and the program stops. |
| Move | `turn_left()`, `turn_right()` | Turns a quarter turn on its cell. | Never. |
| Paint | `paint(color)` | Paints its cell with the color, over any paint already there. | Never. |
| Paint | `erase()` | Removes the paint of its cell. | Never (an unpainted cell stays unpainted). |
| Carry | `pick_up()` | Picks up the item of its cell. | No item on the cell, or the robot already carries one. |
| Carry | `drop()` | Drops the item it carries on its cell. | The robot carries nothing, or the cell already has an item. |
| Sense | `front_is_clear()`, `left_is_clear()`, `right_is_clear()` | `true` when the cell in that direction is a floor cell of the map. | Never. |
| Sense | `here_is(color)` | `true` when its cell has that paint. | Never. |
| Sense | `front_is(color)`, `left_is(color)`, `right_is(color)` | `true` when the cell in that direction has that paint (`false` for a wall). | Never. |
| Sense | `item_here()`, `item_in_front()` | `true` when its cell, or the cell in front, has an item. | Never. |
| Sense | `is_carrying()` | `true` when the robot carries an item. | Never. |
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
| Home | The chapters and their levels, each locked, open or done; the sandbox; the manual; the settings. |
| Level | The goal of the level, the map, the code editor, the controls, the console and the variables panel. |
| Sandbox | The level screen without goal, on an open map. |
| Manual | The manual of the language. |

The layout of each screen is decided with the wireframes, then the mock-up.

The window needs at least 1280 × 720 pixels: below, a message replaces the screen (*"Stepwise needs a window at least 1280 pixels wide."*) and goes away once the window is wide enough again, without losing anything.

### Levels and progression

- The levels are grouped in chapters, each introducing one idea. The plan below is a starting point; the levels are designed in their own roadmap steps.

  | Chapter | Idea | Levels (about) |
  |---|---|---|
  | First steps | `forward()`, `turn_left()`, `turn_right()`, the order of the instructions | 3 |
  | Repeat | `repeat` and its block, the indentation | 3 |
  | Paint | `paint()`, `erase()`, the colors | 2 |
  | Decide | `if`, `else if`, `else`, the sensors | 3 |
  | Loop until | `while`, `not`, `break` | 2 |
  | Carry | `pick_up()`, `drop()`, `is_carrying()` | 2 |
  | Remember | `let`, changing a variable, `print` | 2 |
  | Functions | `function`, parameters, `return` | 3 |

- A level has a title, a goal written in a few sentences, its map, the code it starts with, and an optional hint, hidden until the learner asks for it. Each level links to the sections of the manual it needs.
- **Variants**: a level may have up to three maps, the same task on different ground (a wall further away, another number of items), so that a program written for one map only doesn't pass. The learner switches between them with tabs above the map; **Run** runs the shown variant. Each variant passed is marked; the level is done once every variant has passed with the same code. Changing the code clears the marks.
- The levels open in order: a level done opens the next one. A done level stays playable. The setting **Unlock all levels**, for a teacher or a learner who already knows, opens every level.
- **Goals** of a level, all checked when the program ends without error:
  - the robot stands on a given cell;
  - given cells have given paints (and, when the level says so, no other cell is painted);
  - items lie on given cells;
  - the robot carries an item, or carries none.
- When the program ends and every goal is met, the variant passes; once every variant has, a success panel offers the next level. When a goal isn't met, the console says which one, and the cells concerned are marked on the map.

### Code editor

- The code of each level is saved in the browser as the learner types, and comes back when they return to the level. **Reset code** brings back the starting code, after a confirmation.
- Syntax highlighting of the keywords, the names, the numbers, the texts, the colors and the comments.
- Tab inserts four spaces; a new line after a line ending with `:` is indented four spaces more; Backspace at the start of an indentation removes four spaces.
- Errors underlined, with their message on hover (see [Checks before running](#checks-before-running)).
- **Breakpoints**: a click in the margin, left of a line number, sets or removes a breakpoint on that line. They are saved with the code of the level.
- While a program runs or is paused, the editor can't be changed: a message says *"Stop the program to edit your code."*

### Command menu

**Ctrl+Space** in the editor opens a menu at the cursor, to find a command without knowing its name by heart:

- the commands grouped by category: **Move**, **Paint**, **Carry**, **Sense**, **Write** (the built-in functions), **Colors**, **Control** (`if`, `else if`, `else`, `while`, `repeat`, `break`), **Variables** (`let`), **Functions** (`function`, `return`), and **Your names** (the variables and functions of the program that can be used at the cursor);
- typing filters the list (`pai` keeps `paint()`); the arrow keys move in it, Enter or a click inserts the command, Escape closes it;
- next to the chosen command, its one-line description and an example from the manual;
- a statement that opens a block (`if`, `while`, `repeat`, `function`) is inserted with its `:` and its first indented line, the cursor on what to fill in (`if |:`);
- the same menu opens by itself after two letters of a name, and only on Ctrl+Space when the setting **Suggest while typing** is off.

### Running a program

| Control | What it does |
|---|---|
| **Run** | Checks the program, puts the map back to its starting state, then runs the program at the chosen speed. |
| **Pause** / **Resume** | Pauses the program before its next step; resumes it. |
| **Step** | Runs one step, then pauses. Available when paused or stopped (from stopped, it starts the program paused on its first step). |
| **Stop** | Ends the program at once. The map stays as the program left it. |
| **Reset** | Puts the map back to its starting state. |
| **Speed** | Five speeds, from one step per second to **Instant**, which runs as many steps as the browser allows without animation. Kept between sessions. |

- A **step** is one line run: a statement, the condition of a loop checked again, a call entered. The line of the next step is highlighted in the editor.
- A program that never ends (an infinite loop) keeps running, visibly, until **Stop**: the robot keeps moving or turning, the highlighted line keeps changing, and a step counter next to the controls keeps rising, even for a loop that doesn't move the robot. The page never freezes, even at **Instant**.
- A breakpoint pauses the program before it runs that line.
- The robot's actions are animated at the chosen speed: moving, turning, painting, picking up, dropping, and a bump against a wall that stops the program.
- The keyboard shortcuts of the controls are decided with the mock-up.

### Console

- The lines written by `print`, in order.
- The errors, before running or while running, with their line and column: a click on an error puts the cursor on its position in the editor.
- The messages of the level: the goals not met, the success.
- Cleared at each **Run**.

### Variables panel

Shown while the program is paused (by **Pause**, **Step** or a breakpoint) and after it stops on an error:

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

The manual is searchable (a search field filters its sections and commands), and each section has its own address, so that a level, the command menu and an error message can link to it.

### Sandbox

An open map with a few walls, paints and items, without goal. The code is saved like a level's; **Run**, the console and the variables panel work the same way. Editing the map of the sandbox, and a level editor, come later (see the [backlog](docs/backlog.md)).

### Settings

- **Unlock all levels**: on or off (off by default).
- **Suggest while typing**: the command menu opens by itself while typing (on by default).
- **Reset progress**: clears the levels done, the code and the breakpoints of every level, after a confirmation.

### Interface language

The interface is in English. Every text of the interface, the error messages and the manual included, comes from translation files (English is the reference), so that a French translation can come later without changing the code.

## Map

- A grid of cells, at most 16 columns and 12 rows. The coordinates start at the top left cell, `x` growing to the right and `y` downward.
- A cell is a **wall** or a **floor**. A floor cell may hold one paint (one of the five colors) and one item.
- One kind of item in version 1 (its look comes from the sheet of the world elements).
- The goal cells of a level are marked on the map (where to stand, which color where, where to put the items), so that the learner sees the target before writing code.
- Outside the grid counts as a wall.

## Local files

Stepwise has no files of its own on the learner's computer: everything is kept in the browser's `localStorage`, under keys starting with `stepwise.`, with a version number.

| Data | Content |
|---|---|
| Progress | The levels done, and the variants passed with their code. |
| Code | The code and the breakpoints of each level and of the sandbox. |
| Settings | The speed, **Unlock all levels**, **Suggest while typing**. |

The levels and the manual are files of the website, written in the repository.

## Error handling

| Situation | Behavior |
|---|---|
| A syntax or check error in the program | Underlined in the editor, listed in the console with its line and column; **Run** doesn't start. |
| An error while the program runs | The program stops; the error is shown in the console with its line, the line is marked in the editor, the variables panel shows the state at that moment. |
| An infinite loop | The program keeps running visibly until **Stop**; the page stays responsive. |
| A recursion that never ends | Stopped at 200 nested calls with an error. |
| The browser refuses `localStorage` (private browsing, storage full or blocked) | Stepwise works; a banner says *"Your progress can't be saved in this browser."* |
| Saved data that can't be read (damaged, or of an unknown version) | Ignored: the defaults are used, without error; the bad data stays until the next save replaces it. |
| A level file that isn't valid | The level is shown as unavailable in its chapter; the other levels work. The tests check every level file, so this shouldn't reach the website. |
| The WebAssembly module can't load (very old browser) | A message replaces the screen: *"Stepwise needs a recent browser."* |
| A window narrower than 1280 pixels | A message replaces the screen until the window is wide enough. |
