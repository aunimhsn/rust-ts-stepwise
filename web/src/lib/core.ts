// The core of Stepwise, compiled to WebAssembly by `npm run wasm` (see scripts/build-wasm.ts).

import init, { coreVersion } from '#lib/wasm/stepwise.js';

/** What the interface reads from the core once it is loaded. */
export interface Core {
	version: string;
}

/**
 * Loads the WebAssembly module, fetched once whatever the number of calls. A failure is written
 * to the console with its cause, then passed on to the page, which says it.
 */
export async function loadCore(): Promise<Core> {
	try {
		await init();
	} catch (error) {
		console.error('core: the WebAssembly module could not load', error);
		throw error;
	}
	return { version: coreVersion() };
}

/**
 * The checks before running (see crates/wasm/src/check.rs), to call once `loadCore` has resolved:
 * the errors of a program, each with its place and its kind, a code with its values.
 *
 * A place is given in the text that was checked: `text.slice(span.from, span.to)`. Always check
 * the text of the document of the editor, never a text from elsewhere: the editor counts one unit
 * for every line break, so the places of a text with `\r\n` would not be those of its document.
 */
export {
	check,
	type ProgramError,
	type ProgramErrorKind,
	type ReservedKind,
	type Span
} from '#lib/wasm/stepwise.js';

/**
 * The levels and the sandbox (see crates/wasm/src/level.rs and world.rs), to create once
 * `loadCore` has resolved: `new Level(toml)` and `new Sandbox(toml)` read the text of a file of
 * `web/levels/`, and throw an `Error` with the reason, in English, when it isn't valid.
 *
 * A level stays in the module: free it once its screen closes (`using`, or `free()`), and don't
 * call it afterwards. What it gives (`maps()`, `world()`) are plain objects, built again at each
 * call: read them once and keep them, in a component in `$state.raw`, since a world is never
 * changed but replaced whole, and `$state` would wrap each of its cells in a proxy.
 */
export {
	Level,
	Sandbox,
	type Cell,
	type Color,
	type Direction,
	type Goals,
	type LevelMap,
	type Paint,
	type PaintGoal,
	type Point,
	type Robot,
	type World
} from '#lib/wasm/stepwise.js';

/** The probe of step 4c (see crates/core/src/probe.rs), to create once `loadCore` has resolved. */
export { Probe, type Action, type ProbeDirection } from '#lib/wasm/stepwise.js';
