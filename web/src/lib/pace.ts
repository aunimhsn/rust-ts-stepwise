// How many steps of the program run at each animation frame, at each speed (see
// docs/technical-choices.md, Execution). Written for the probe of step 4c; the running of step 16
// keeps it or rewrites it.

/** The five speeds, in steps per second; the last one, **Instant**, runs without animation. */
export const SPEEDS = [1, 2, 5, 20, Infinity] as const;

/** A speed of `SPEEDS`. */
export type Speed = (typeof SPEEDS)[number];

/**
 * The share of a step owed at an animated speed, carried from a frame to the next. A run starts
 * with a whole step owed, so that its first step runs at its first frame.
 */
export const START_CARRY = 1;

/**
 * At an animated speed, whether a step runs at this frame, `elapsed` milliseconds after the one
 * before, and the share of a step still owed after it, kept so that the speed is exact whatever
 * the rate of the screen. At most one step runs per frame, so that each one is drawn, and a frame
 * adds at most a whole step: after a frame that came late (a tab hidden, a busy browser), the
 * program goes on at its pace rather than catching up.
 */
export function paceFrame(
	rate: number,
	elapsed: number,
	carry: number
): { step: boolean; carry: number } {
	const owed = carry + Math.min((Math.max(elapsed, 0) * rate) / 1000, 1);
	return owed >= 1 ? { step: true, carry: owed - 1 } : { step: false, carry: owed };
}

/**
 * The steps an **Instant** frame runs between two looks at the clock, which costs a little. A
 * batch can run past the budget when the steps are slow: to tune with the real interpreter.
 */
const BATCH = 1000;

/**
 * At **Instant**, runs steps until `budget` milliseconds have passed since `start`, so that the
 * page keeps drawing and answering between two frames, or until `step` returns `false` (the
 * program ended at that step). Returns the number of steps run, the last one included.
 */
export function runInstant(
	step: () => boolean,
	start: number,
	budget: number,
	now: () => number
): number {
	let count = 0;
	while (now() - start < budget) {
		for (let index = 0; index < BATCH; index += 1) {
			count += 1;
			if (!step()) {
				return count;
			}
		}
	}
	return count;
}
