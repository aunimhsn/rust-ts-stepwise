// The core of Stepwise, compiled to WebAssembly by `npm run wasm` (see scripts/build-wasm.ts).

import init, { coreVersion } from '#lib/wasm/stepwise.js';

/** What the interface reads from the core once it is loaded. */
export interface Core {
	version: string;
}

/** Loads the WebAssembly module, fetched once whatever the number of calls. */
export async function loadCore(): Promise<Core> {
	await init();
	return { version: coreVersion() };
}
