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
	/** What JavaScript cuts out of `source` with the place of an error. */
	function text(source: string, span: Span): string {
		return source.slice(span.from, span.to);
	}

	/** The code, the line and the text underlined of each error of `source`. */
	function found(source: string): [string, number, string][] {
		return check(source).map((error) => [
			error.kind.code,
			error.span.line,
			text(source, error.span)
		]);
	}

	it('find no error in a program that can run', () => {
		expect(check('let sides = 4\nrepeat sides:\n    forward()\n')).toEqual([]);
		expect(check('')).toEqual([]);
	});

	it('give the first syntax error alone, whatever the names around it', () => {
		expect(check('sidse = 1\nforward(\nturn()\n')).toEqual([
			{ span: { line: 2, from: 17, to: 18 }, kind: { code: 'unclosed-paren' } }
		]);
	});

	it('give every error of names in the order of the source, with its values', () => {
		const source = 'let red = 3\nprint(sides)\nlet sides = 4\nforward(sides)\n';
		expect(check(source)).toEqual([
			{
				span: { line: 1, from: 4, to: 7 },
				kind: { code: 'reserved-name', word: 'red', belongsTo: 'color' }
			},
			{
				span: { line: 2, from: 18, to: 23 },
				kind: {
					code: 'variable-before-let',
					name: 'sides',
					declared: { line: 3, from: 29, to: 34 }
				}
			},
			{
				span: { line: 4, from: 39, to: 46 },
				kind: { code: 'wrong-argument-count', name: 'forward', expected: 0, given: 1 }
			}
		]);
	});

	it('give the values of an error of the lexer and of the parser', () => {
		expect(check('if done:\n      forward()\n')).toEqual([
			{
				span: { line: 2, from: 9, to: 15 },
				kind: { code: 'wrong-indent', found: 6, below: 0, above: 4 }
			}
		]);
		expect(check('let repeat = 3\n')).toEqual([
			{ span: { line: 1, from: 4, to: 10 }, kind: { code: 'keyword-as-name', keyword: 'repeat' } }
		]);
	});

	it('give a character as a text, of two units for an emoji', () => {
		expect(check('print("\\🤖")\n')).toEqual([
			{ span: { line: 1, from: 7, to: 10 }, kind: { code: 'unknown-escape', escape: '🤖' } }
		]);
	});

	it('place an error after accents and an emoji where JavaScript counts it', () => {
		// An accent is one unit for JavaScript, an emoji two: the core counts one character for each.
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
			{ span: { line: 1, from: 13, to: 13 }, kind: { code: 'expected-value' } }
		]);
		expect(source.slice(0, 13)).toBe('print("🤖") +');
	});

	it('place an error in the text it was given, a line break with a carriage return included', () => {
		// The two units of each `\r\n` are counted: these places are those of this text, not those
		// of an editor, which counts one unit for a line break.
		expect(found('let côté = "🤖"\r\nprint(sidse)\r\n')).toEqual([
			['not-snake-case', 1, 'côté'],
			['unknown-name', 2, 'sidse']
		]);
	});

	it('give errors that TypeScript tells apart by their code', () => {
		const kind = check('print(sidse)\n')[0]?.kind;
		// Only some errors carry a name: this line compiles because the code narrows the type.
		expect(kind?.code === 'unknown-name' ? kind.name : undefined).toBe('sidse');
	});
});
