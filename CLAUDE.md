# CLAUDE.md

## Communication

- Always communicate with the user in French through the Claude Code harness (chat responses, explanations, questions).
- Address the user informally, with « tu » (tutoiement), never « vous », in every session.

## Naming conventions

- Follow Rust conventions: Rust source files and module directories use snake_case (e.g. `src/file_parser.rs`, `src/user_settings/`).
- All other files and directories use kebab-case (e.g. `docs/getting-started.md`, `scripts/`).
- Standard names keep their conventional form (e.g. `Cargo.toml`, `README.md`, `CLAUDE.md`).
- File and directory names must be in English.

## Language

- Everything in the project (code, identifiers, comments, documentation, commit messages) must be written in English.
- Exception: the translations of the user interface, if the project has any. Only the translated strings and their comments are in the other language; the code stays in English.
- If the user interface is translated, its texts are never hard-coded: each message goes into the translation file of every language (English is the reference), read with the library chosen in `docs/technical-choices.md`.
- Exception: the code review notes in `outputs/` are written in French.

## Specifications and roadmap

- Before writing code, describe the expected behavior of the application in `specs.md`, then split the work into steps in `roadmap.md`, and write the technical choices and their reasons in `docs/technical-choices.md`.
- At the start of each session, read `roadmap.md` to know the progress; when asked to continue, resume at the first unchecked step.
- At the start of each step, before writing any code, judge whether splitting it into sub-steps (2a, 2b…) helps, and how finely: the split must be an advantage, never a burden, and its granularity is Claude Code's call at each step. A sub-step is one pull request with one coherent change that can be reviewed in one pass and tested on its own, without leaving the application broken in between; a small step stays whole, since each pull request also costs a review, the checks and an archive. Propose the split to the owner, then write it in `roadmap.md`.
- Each step is delivered in its own pull request, which checks the step's box in `roadmap.md`.
- Keep `roadmap.md` consistent with `specs.md` when the specifications change.
- Keep `roadmap.md` short, since it is read at every session: the steps to come in detail, one line per step done with its pull requests. Once the pull request of a step (or of a sub-step) is merged, move the detail of the step to `docs/roadmap-archive.md` in the first commit of the pull request of the next step, rather than in a pull request of its own. Reference notes (technical choices, rules of the external services) live in `docs/`, linked from `roadmap.md`, and are read only by the steps that need them.
- Ideas, polish and edge cases found along the way that the current step doesn't need go to `docs/backlog.md`, never into the steps of `roadmap.md`: the roadmap keeps only the path to the next version. The owner decides when an entry of the backlog becomes a step.
- Read `specs.md` by section: list its headings first (`grep -n '^#' specs.md`), then read only the sections the step needs.
- When the pull request of a step (or of a sub-step) is merged, tell the owner that the step is done and remind them to start a new session for the next step by typing `/clear` (give them the command): a fresh session costs far fewer tokens, and `roadmap.md` and the memory hold what is needed to resume.

## Connectors

- The claude.ai connectors (Claude Docs and the others) are turned off for the project in `.claude/settings.json` (`disableClaudeAiConnectors`): their instructions and tools would be sent with every message. Remove the setting only if the project needs one of them.
- The skills a project doesn't use (documents, slides, spreadsheets, browsers, charts, Claude API…) are hidden from the model in the same file (`skillOverrides`, `user-invocable-only`): their descriptions would be sent with every message. The owner can still run them with `/<name>`; to give one back to the model, remove its line or use `/skills`.

## Working folders

Two folders are the exchange area between the owner and Claude Code. They are ignored by Git, except their `.gitkeep`, and hold only temporary files.

- `inputs/`: what the owner gives for context: screenshots, examples, references, copied texts. Read them when the owner points to them; never commit, move or delete them unless asked.
- `outputs/`: what Claude Code produces for the owner: reports, code review notes, previews, screenshots. Name them `<YYYY-MM-DD>-<subject>.<ext>` when they are meant to be kept for a while.
- Anything meant to last goes elsewhere and is committed: documentation in `docs/`, images used by the application in `assets/`, test data next to the tests.

## Process

- Follow the phases of `process.md`: specifications, wireframes, mock-up, design system, then the build, one pull request per step.
- For a user interface, start with editable Excalidraw wireframes in `docs/wireframes/` (grayscale, one group per block; `excalidraw-wireframes` skill), and build the HTML mock-up only once the owner has approved the layout (`html-mockups` skill).
- During the specifications, check the documentation, rate limits and terms of use of every external service, and record its limits and the risks the owner accepts, with their date, in a note of `docs/` linked from `roadmap.md` (e.g. `docs/<service>-usage.md`).
- This project adds a phase of its own between the specifications and the wireframes: draw every element of the world map one by one (floor, walls, paints, objects, goals, the robot in its four directions) on a sheet of elements in `docs/mockups/`, in 3 or 4 styles compared with tabs, then all together on a small map to check that they fit together. The elements are SVG files, reused as they are by the application.
- For the mock-up, compare 3 or 4 styles that explore new directions for this application. `design-journal.md` records the past design experiences: use it as a source of ideas for one style at most, never as default rules. Give each open decision its own tab group on the mock-up page, and remove it once decided. A new section of the mock-up goes at the top of the page, so that the owner finds it at once; past ten sections, a table of contents under the header links to each.
- A mock-up is split into small files (the shared styles and scripts, one style sheet and one script per block, one script per section), so that a change reads only what it needs.
- Each time the mock-up changes (a new mock-up, a new section, any change to its page or its files), give the owner its link at once, served: serve the folder that holds it (`python3 -m http.server <port> --bind 127.0.0.1`, kept running in the background; another port when one is taken, e.g. by the server of another worktree), check that the page answers with the change, and give `http://127.0.0.1:<port>/docs/mockups/<name>.html#section-<n>`, with the titles of the section and of the windows to look at. Never a `file://` path: a sandboxed browser may not open it.
- Once approved, the mock-up (its page and its folder) is the single source of truth of the interface. A design change goes into the mock-up first, then into `specs.md` and into the code when that part is already coded. The wireframe only fixed the first layout and isn't kept up to date.
- Check a visual change with a few screenshots, of the window concerned only, and delete them once looked at: they are for this check, not for the owner, who looks at the mock-up and the application themselves.

## Git workflow

- Never commit or push directly to `main`.
- For each task, create a dedicated branch from `main`, commit and push only on that branch.
- Every change must go through a pull request targeting `main`.
- Each time a pull request is opened or updated, give the owner its full link (`https://github.com/<owner>/<repository>/pull/<number>`), not only its number.
- Merge pull requests with **Squash and merge** only, so that each pull request becomes one commit on `main`, titled in Conventional Commits. The repository allows only squash merging and deletes the branch once merged; check these settings on every new repository (`gh repo edit --enable-squash-merge --enable-merge-commit=false --enable-rebase-merge=false --delete-branch-on-merge`).
- Name branches `<type>/<short-description>` in kebab-case, using the commit types below (e.g. `feat/dark-mode`, `fix/settings-save`).
- Never switch branches in the main project directory while the owner may be testing a branch there (for example with `cargo run`): it would change the code they run. Work on any other branch in a separate Git worktree outside the project (`git worktree add <directory> -b <branch> origin/main`), and remove it with `git worktree remove <directory>` once the work is pushed.
- When the owner says they aren't running or testing anything, work on the task branch in the main project directory instead, so that they can follow the files changing in their editor; switch it back to `main` once the pull request is merged.

## Commit messages

Follow [Conventional Commits 1.0.0](https://www.conventionalcommits.org/en/v1.0.0/). Pull request titles use the same format.

```
<type>(<optional scope>): <description>

<optional body: what and why>

<optional footer>
```

| Type | Use for |
|---|---|
| `feat` | A new feature for the user. |
| `fix` | A bug fix. |
| `docs` | Documentation only (`README.md`, `specs.md`, `roadmap.md`, `CLAUDE.md`, doc comments). |
| `style` | Formatting with no code change. |
| `refactor` | A code change that neither fixes a bug nor adds a feature. |
| `perf` | A performance improvement. |
| `test` | Adding or fixing tests only. |
| `build` | Build system or dependencies (`Cargo.toml`, `Cargo.lock`). |
| `ci` | CI configuration (`.github/`). |
| `chore` | Other maintenance (tooling, `.gitignore`, Claude Code agents). |
| `revert` | Reverting a previous commit. |

- Scope: optional, the module or area concerned (e.g. `feat(settings): ...`, `fix(parser): ...`).
- Description: imperative mood, lowercase, no final period, 72 characters max (e.g. `feat(settings): save the settings atomically`).
- Breaking change: add `!` after the type or scope (e.g. `feat(settings)!: ...`) and explain it in a `BREAKING CHANGE:` footer.

## Code review

- Before opening any pull request that changes code or its build (`src/`, tests, `Cargo.toml`, `build.rs`, `rust-toolchain.toml`, `.github/workflows/`), run the `code-reviewer` subagent (`.claude/agents/code-reviewer.md`) on the branch.
- Run it once per pull request, on the whole branch, to save tokens; a step too large for one review is split into sub-steps in `roadmap.md`.
- It writes its recommendations, in French, to `outputs/<YYYY-MM-DD>-<branch>-code-review.md`. These notes are ignored by Git.
- Summarize the recommendations to the user and apply only the ones they approve, then open the pull request.

## Development

- Before committing, run and make sure all of these pass:
  - `cargo fmt --all --check`
  - `cargo clippy --all-targets --all-features -- -D warnings`
  - `cargo test --all-features`
- The same checks run in CI (`.github/workflows/ci.yml`) on every pull request.
- `unsafe` code is forbidden and Clippy pedantic lints are enabled (see `[lints]` in `Cargo.toml`).
- `rust-version` in `Cargo.toml` is the minimum Rust version: CI also builds with it. Raise it when a dependency or a language feature needs a newer Rust.

## Releases

- Pushing a version tag (e.g. `v0.1.0`) that matches the version of `Cargo.toml` publishes a GitHub release with the executables for Windows, macOS and Linux (`.github/workflows/release.yml`).
- This workflow suits a command-line or terminal application. For a desktop application (installers) or a web application (deployment), adapt or replace it, and record the choice in `docs/technical-choices.md`.
