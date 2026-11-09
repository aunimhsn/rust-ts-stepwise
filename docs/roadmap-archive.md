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
