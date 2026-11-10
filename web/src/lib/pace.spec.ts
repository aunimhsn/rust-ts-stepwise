import { describe, expect, it } from 'vitest';
import { paceFrame, runInstant, START_CARRY } from './pace.ts';

/**
 * The steps run at `rate` during `seconds`, by frames `interval` milliseconds apart, each one
 * moved by up to `jitter` milliseconds as the frames of a real browser.
 */
function stepsRun(rate: number, interval: number, seconds = 1, jitter = 0): number {
	let carry = START_CARRY;
	let steps = 0;
	const frames = Math.round((seconds * 1000) / interval);
	for (let frame = 0; frame < frames; frame += 1) {
		const offset = frame % 2 === 0 ? jitter : -jitter;
		const paced = paceFrame(rate, frame === 0 ? 0 : interval + offset, carry);
		carry = paced.carry;
		steps += paced.step ? 1 : 0;
	}
	return steps;
}

describe('paceFrame', () => {
	it('runs the first step at the first frame', () => {
		expect(paceFrame(1, 0, START_CARRY)).toEqual({ step: true, carry: 0 });
	});

	it('runs as many steps per second as the speed', () => {
		expect([1, 2, 5, 20].map((rate) => stepsRun(rate, 1000 / 60))).toEqual([1, 2, 5, 20]);
	});

	it('keeps the speed on any screen, with irregular frames', () => {
		for (const interval of [1000 / 60, 1000 / 75, 1000 / 144]) {
			expect(stepsRun(20, interval, 10, 0.3)).toBeGreaterThanOrEqual(199);
			expect(stepsRun(20, interval, 10, 0.3)).toBeLessThanOrEqual(201);
			expect(stepsRun(5, interval, 10, 0.3)).toBeGreaterThanOrEqual(49);
			expect(stepsRun(5, interval, 10, 0.3)).toBeLessThanOrEqual(51);
		}
	});

	it('runs at most one step per frame, and never catches up a late frame', () => {
		expect(paceFrame(20, 10_000, 0)).toEqual({ step: true, carry: 0 });
		expect(stepsRun(20, 100)).toBe(10);
	});

	it('ignores a clock that goes back', () => {
		expect(paceFrame(5, -500, 0.5)).toEqual({ step: false, carry: 0.5 });
	});
});

describe('runInstant', () => {
	it('runs steps until the budget of the frame is spent', () => {
		let time = 0;
		// Each look at the clock finds it 1 ms later: three looks before the budget of 2 ms.
		const count = runInstant(
			() => true,
			0,
			2,
			() => time++
		);
		expect(count).toBe(2000);
	});

	it('stops at the end of the program', () => {
		let left = 5;
		expect(
			runInstant(
				() => (left -= 1) > 0,
				0,
				1000,
				() => 0
			)
		).toBe(5);
	});
});
