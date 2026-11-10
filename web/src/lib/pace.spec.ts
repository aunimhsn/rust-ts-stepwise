import { describe, expect, it } from 'vitest';
import { paceFrame, runInstant, START_CARRY } from './pace.ts';

/** The frames of a second at 60 frames per second, and the steps they run at `rate`. */
function stepsInOneSecond(rate: number, frames = 60): number {
	let carry = START_CARRY;
	let steps = 0;
	for (let frame = 0; frame < frames; frame += 1) {
		const paced = paceFrame(rate, frame === 0 ? 0 : 1000 / frames, carry);
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
		expect([1, 2, 5, 20].map((rate) => stepsInOneSecond(rate))).toEqual([1, 2, 5, 20]);
	});

	it('runs at most one step per frame, and never catches up a late frame', () => {
		expect(paceFrame(20, 10_000, 0)).toEqual({ step: true, carry: 0 });
		expect(stepsInOneSecond(20, 10)).toBe(10);
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
