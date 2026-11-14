// The module the interface loads, called from TypeScript: checks the build of `npm run wasm`
// and its JavaScript glue together. The browser fetches the module; the test reads its file.

import { readFileSync } from 'node:fs';
import { describe, expect, it } from 'vitest';
import {
	check,
	coreVersion,
	initSync,
	Level,
	Probe,
	Sandbox,
	type Cell,
	type Span
} from '#lib/wasm/stepwise.js';

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

describe('a level file', () => {
	// Two maps: a corner to paint, with an item to bring back; then a corridor, the item carried.
	const LEVEL = `
version = 1

starting_code = '''
# Paint the corner
repeat 2:
    forward()
'''

[[maps]]
terrain = '''
..#
...
'''
robot = { at = [0, 0], facing = "east" }
items = [[2, 1]]
paints = [{ color = "blue", path = [[0, 1], [1, 1]] }]

[maps.goals]
robot_at = [1, 1]
paints = [{ color = "red", path = [[0, 0], [1, 0], [1, 1]] }]
nothing_else_painted = true
items_at = [[0, 0]]
carrying = false

[[maps]]
terrain = '''
....
'''
robot = { at = [3, 0], facing = "west", carrying = true }

[maps.goals]
items_at = [[0, 0]]
`;

	/** The file of a level of one map, the body of its `[[maps]]` table. */
	function levelOf(map: string): string {
		return `version = 1\nstarting_code = ''\n\n[[maps]]\n${map}`;
	}

	it('gives the code the learner starts with, as written', () => {
		using level = new Level(LEVEL);
		expect(level.startingCode()).toBe('# Paint the corner\nrepeat 2:\n    forward()\n');
	});

	it('gives the world of each map as plain objects', () => {
		using level = new Level(LEVEL);
		const empty: Cell = { type: 'floor', item: false };
		// A key is left out rather than undefined: a cell without paint has no `paint`.
		expect(level.maps().map((map) => map.world)).toStrictEqual([
			{
				width: 3,
				height: 2,
				cells: [
					empty,
					empty,
					{ type: 'wall' },
					{ type: 'floor', paint: { color: 'blue', passages: ['east'] }, item: false },
					{ type: 'floor', paint: { color: 'blue', passages: ['west'] }, item: false },
					{ type: 'floor', item: true }
				],
				robot: { position: { x: 0, y: 0 }, direction: 'east', carrying: false }
			},
			{
				width: 4,
				height: 1,
				cells: [empty, empty, empty, empty],
				robot: { position: { x: 3, y: 0 }, direction: 'west', carrying: true }
			}
		]);
	});

	it('gives the goals of each map that have a cell, to mark on it', () => {
		using level = new Level(LEVEL);
		expect(level.maps().map((map) => map.goals)).toStrictEqual([
			{
				robotAt: { x: 1, y: 1 },
				paints: [
					{ at: { x: 0, y: 0 }, color: 'red', passages: ['east'] },
					{ at: { x: 1, y: 0 }, color: 'red', passages: ['south', 'west'] },
					{ at: { x: 1, y: 1 }, color: 'red', passages: ['north'] }
				],
				itemsAt: [{ x: 0, y: 0 }]
			},
			{ paints: [], itemsAt: [{ x: 0, y: 0 }] }
		]);
	});

	it('gives cells that TypeScript tells apart by their type', () => {
		using level = new Level(LEVEL);
		const cells = level.maps()[0]?.world.cells ?? [];
		// Only a floor cell has a paint: this line compiles because the type narrows the cell.
		const colors = cells.map((cell) => (cell.type === 'floor' ? cell.paint?.color : undefined));
		expect(colors).toEqual([undefined, undefined, undefined, 'blue', 'blue', undefined]);
	});

	it('throws an error that says why a file is refused', () => {
		const onWall = levelOf(
			`terrain = '.#.'\nrobot = { at = [1, 0], facing = "east" }\n\n[maps.goals]\nrobot_at = [2, 0]\n`
		);
		// The sentences are those of the core, tested there: one of a map, one of the file, and
		// the start of what the reader of TOML says.
		expect(() => new Level(onWall)).toThrow(new Error('map 1: [1, 0] is a wall'));
		expect(() => new Level('version = 2\n')).toThrow(
			new Error('the file has version 2, while this core reads version 1')
		);
		expect(() => new Level('version = \n')).toThrow(/^TOML parse error/);
	});
});

describe('the file of the sandbox', () => {
	it('gives its starting code and its world as a plain object', () => {
		const small = `version = 1\nstarting_code = '# Try'\n\n[map]\nterrain = '.#'\nrobot = { at = [0, 0], facing = "north" }\n`;
		using sandbox = new Sandbox(small);
		expect(sandbox.startingCode()).toBe('# Try');
		expect(sandbox.world()).toStrictEqual({
			width: 2,
			height: 1,
			cells: [{ type: 'floor', item: false }, { type: 'wall' }],
			robot: { position: { x: 0, y: 0 }, direction: 'north', carrying: false }
		});
	});

	it('reads the file of the website, whatever its map becomes', () => {
		const file = readFileSync(new URL('../../levels/sandbox.toml', import.meta.url), 'utf8');
		using sandbox = new Sandbox(file);
		const world = sandbox.world();
		expect(world.cells).toHaveLength(world.width * world.height);
		expect(check(sandbox.startingCode())).toEqual([]);
	});

	it('throws the error of a file that has goals, or that is the one of a level', () => {
		const map = `terrain = '..'\nrobot = { at = [0, 0], facing = "east" }\n`;
		const withGoals = `version = 1\nstarting_code = ''\n\n[map]\n${map}\n[map.goals]\nrobot_at = [1, 0]\n`;
		expect(() => new Sandbox(withGoals)).toThrow(new Error('the map of the sandbox has goals'));
		const level = `version = 1\nstarting_code = ''\n\n[[maps]]\n${map}`;
		expect(() => new Sandbox(level)).toThrow(/^TOML parse error/);
	});
});
