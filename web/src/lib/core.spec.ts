// The module the interface loads, called from TypeScript: checks the build of `npm run wasm`
// and its JavaScript glue together. The browser fetches the module; the test reads its file.

import { readFileSync } from 'node:fs';
import { describe, expect, it } from 'vitest';
import { coreVersion, initSync, Probe } from '#lib/wasm/stepwise.js';

const workspaceVersion = /\[workspace\.package\]\nversion = "([^"]+)"/.exec(
	readFileSync(new URL('../../../Cargo.toml', import.meta.url), 'utf8')
)?.[1];

describe('the WebAssembly module', () => {
	initSync({ module: readFileSync(new URL('./wasm/stepwise_bg.wasm', import.meta.url)) });

	it('gives the version of the workspace, computed in Rust', () => {
		expect(workspaceVersion).toMatch(/^\d+\.\d+\.\d+/);
		expect(coreVersion()).toBe(workspaceVersion);
	});

	it('runs the fake program of the probe one step at a time', () => {
		using probe = new Probe();
		expect([probe.step(), probe.step(), probe.step()]).toEqual(['moved', 'moved', 'turned']);
		expect({ x: probe.x, y: probe.y, direction: probe.direction }).toEqual({
			x: 3,
			y: 1,
			direction: 'south'
		});
		expect(probe.steps).toBe(3n);
	});
});
