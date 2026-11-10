// The module the interface loads, called from TypeScript: checks the build of `npm run wasm`
// and its JavaScript glue together. The browser fetches the module; the test reads its file.

import { readFileSync } from 'node:fs';
import { describe, expect, it } from 'vitest';
import { coreVersion, initSync } from '#lib/wasm/stepwise.js';

const workspaceVersion = /\[workspace\.package\]\nversion = "([^"]+)"/.exec(
	readFileSync(new URL('../../../Cargo.toml', import.meta.url), 'utf8')
)?.[1];

describe('the WebAssembly module', () => {
	it('gives the version of the workspace, computed in Rust', () => {
		expect(workspaceVersion).toMatch(/^\d+\.\d+\.\d+/);
		initSync({ module: readFileSync(new URL('./wasm/stepwise_bg.wasm', import.meta.url)) });
		expect(coreVersion()).toBe(workspaceVersion);
	});
});
