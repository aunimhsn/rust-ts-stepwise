// The module the interface loads, called from TypeScript: checks the build of `npm run wasm`
// and its JavaScript glue together. The browser fetches the module; the test reads its file.

import { readFileSync } from 'node:fs';
import { describe, expect, it } from 'vitest';
import { check, coreVersion, initSync, Probe, type Span } from '#lib/wasm/stepwise.js';

const workspaceVersion = /\[workspace\.package\]\nversion = "([^"]+)"/.exec(
	readFileSync(new URL('../../../Cargo.toml', import.meta.url), 'utf8')
)?.[1];

initSync({ module: readFileSync(new URL('./wasm/stepwise_bg.wasm', import.meta.url)) });

describe('the WebAssembly module', () => {
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

describe('the checks before running', () => {
	/** What the editor would select with the place of an error: JavaScript cuts as it counts. */
	function text(source: string, span: Span): string {
		return source.slice(span.from, span.to);
	}

	/** The code, the line and the text underlined of each error of `source`. */
	function found(source: string): [string, number, string][] {
		return check(source).map((error) => [error.code, error.span.line, text(source, error.span)]);
	}

	it('find no error in a program that can run', () => {
		expect(check('let sides = 4\nrepeat sides:\n    forward()\n')).toEqual([]);
		expect(check('')).toEqual([]);
	});

	it('give the first syntax error alone, whatever the names around it', () => {
		expect(check('sidse = 1\nforward(\nturn()\n')).toEqual([
			{ code: 'unclosed-paren', span: { line: 2, from: 17, to: 18 } }
		]);
	});

	it('give every error of names in the order of the source, with its values', () => {
		const source = 'let red = 3\nprint(sides)\nlet sides = 4\nforward(sides)\n';
		expect(check(source)).toEqual([
			{ code: 'reserved-name', span: { line: 1, from: 4, to: 7 }, word: 'red', kind: 'color' },
			{
				code: 'variable-before-let',
				span: { line: 2, from: 18, to: 23 },
				name: 'sides',
				declared: { line: 3, from: 29, to: 34 }
			},
			{
				code: 'wrong-argument-count',
				span: { line: 4, from: 39, to: 46 },
				name: 'forward',
				expected: 0,
				given: 1
			}
		]);
	});

	it('give the values of an error of the lexer and of the parser', () => {
		expect(check('if done:\n      forward()\n')).toEqual([
			{ code: 'wrong-indent', span: { line: 2, from: 9, to: 15 }, found: 6, below: 0, above: 4 }
		]);
		expect(check('let repeat = 3\n')).toEqual([
			{ code: 'keyword-as-name', span: { line: 1, from: 4, to: 10 }, keyword: 'repeat' }
		]);
	});

	it('place an error after accents and an emoji where the editor counts it', () => {
		// An accent is one unit for the editor, an emoji two: the core counts one character for each.
		expect(found('let côté = "🤖 ok"\nprint(côté, "🤖🤖", sidse)\n')).toEqual([
			['not-snake-case', 1, 'côté'],
			['unknown-name', 2, 'sidse']
		]);
		expect(found('let sides = "côté 🤖"\nprint("🤖") forward()\n')).toEqual([
			['unexpected-token', 2, 'forward']
		]);
	});

	it('place an error at the end of a line between two characters', () => {
		const source = 'print("🤖") +\nforward()\n';
		expect(check(source)).toEqual([
			{ code: 'expected-value', span: { line: 1, from: 13, to: 13 } }
		]);
		expect(source.slice(0, 13)).toBe('print("🤖") +');
	});

	it('count a line break with a carriage return for the two units it is', () => {
		expect(found('let côté = "🤖"\r\nprint(sidse)\r\n')).toEqual([
			['not-snake-case', 1, 'côté'],
			['unknown-name', 2, 'sidse']
		]);
	});

	it('give errors that TypeScript tells apart by their code', () => {
		const [error] = check('print(sidse)\n');
		// Only some errors carry a name: this line compiles because the code narrows the type.
		expect(error?.code === 'unknown-name' ? error.name : undefined).toBe('sidse');
	});
});
