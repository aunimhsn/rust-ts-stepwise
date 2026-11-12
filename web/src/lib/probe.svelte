<!-- The probe of step 4c, thrown away by the map view of step 14 and the running of step 17:
     the fake program of the core, advanced by `step()` at each animation frame at the chosen
     speed, the robot moved by CSS transitions. Created once the core has loaded. -->
<script lang="ts">
	import { onDestroy } from 'svelte';
	import { Probe, type Direction } from '#lib/core.ts';
	import { t } from '#lib/i18n.ts';
	import { paceFrame, runInstant, SPEEDS, START_CARRY } from '#lib/pace.ts';

	/** The size of a cell, in the units of the drawings of the world elements. */
	const CELL = 64;
	/** The time an **Instant** frame spends running steps, so that the page still draws. */
	const INSTANT_BUDGET = 8;
	/** The share of the time between two steps an animation takes, at an animated speed. */
	const ANIMATION_SHARE = 0.8;
	/** The robot of the drawings looks north. */
	const ANGLES: Record<Direction, number> = { north: 0, east: 90, south: 180, west: 270 };

	const size = Probe.mapSize();
	const cells = Array.from({ length: size * size }, (_, index) => ({
		x: index % size,
		y: Math.floor(index / size)
	}));

	let probe = new Probe();
	let running = $state(false);
	let speed = $state(2);
	let x = $state(probe.x);
	let y = $state(probe.y);
	let angle = $state(ANGLES[probe.direction]);
	let steps = $state(0);
	let measured = $state<number>();

	const rate = $derived(SPEEDS[speed]);
	const duration = $derived(Number.isFinite(rate) ? (ANIMATION_SHARE * 1000) / rate : 0);

	let frame: number | undefined;
	let last = 0;
	let carry = START_CARRY;
	let measureStart = 0;
	let measureSteps = 0;

	/** Copies the robot of the core into the page, once per frame whatever the steps run. */
	function draw(): void {
		x = probe.x;
		y = probe.y;
		// The shortest turn to the new direction, so that the robot never spins the long way
		// around: the angle grows or shrinks without bound rather than wrapping at 360.
		const target = ANGLES[probe.direction];
		angle += ((((target - angle) % 360) + 540) % 360) - 180;
		steps = Number(probe.steps);
	}

	function tick(time: number): void {
		if (Number.isFinite(rate)) {
			const paced = paceFrame(rate, time - last, carry);
			carry = paced.carry;
			if (paced.step) {
				probe.step();
			}
		} else {
			const count = runInstant(
				() => {
					probe.step();
					return true;
				},
				performance.now(),
				INSTANT_BUDGET,
				() => performance.now()
			);
			measureSteps += count;
			if (time - measureStart >= 1000) {
				measured = Math.round((measureSteps * 1000) / (time - measureStart));
				measureStart = time;
				measureSteps = 0;
			}
		}
		last = time;
		draw();
		frame = requestAnimationFrame(tick);
	}

	function startMeasure(): void {
		measured = undefined;
		measureStart = performance.now();
		measureSteps = 0;
	}

	function run(): void {
		running = true;
		last = performance.now();
		startMeasure();
		frame = requestAnimationFrame(tick);
	}

	function pause(): void {
		running = false;
		if (frame !== undefined) {
			cancelAnimationFrame(frame);
			frame = undefined;
		}
	}

	function reset(): void {
		pause();
		probe.free();
		probe = new Probe();
		carry = START_CARRY;
		measured = undefined;
		draw();
	}

	function chooseSpeed(index: number): void {
		speed = index;
		startMeasure();
	}

	onDestroy(() => {
		pause();
		probe.free();
	});
</script>

<section>
	<h2>{t('probe-title')}</h2>
	<svg
		viewBox="0 0 {size * CELL} {size * CELL}"
		width={size * CELL}
		height={size * CELL}
		role="img"
		aria-label={t('probe-map')}
	>
		{#each cells as cell (`${cell.x},${cell.y}`)}
			<rect class="floor" x={cell.x * CELL} y={cell.y * CELL} width={CELL} height={CELL} />
		{/each}
		<g
			class="robot"
			style:transform="translate({x * CELL}px, {y * CELL}px) rotate({angle}deg)"
			style:transition-duration="{duration}ms"
		>
			<!-- The robot of docs/mockups/world-elements/daylight/robot.svg, copied for the probe. -->
			<rect x="11" y="15" width="8" height="34" rx="3" fill="#222935" />
			<rect x="45" y="15" width="8" height="34" rx="3" fill="#222935" />
			<rect
				x="16"
				y="14"
				width="32"
				height="36"
				rx="9"
				fill="#ffffff"
				stroke="#222935"
				stroke-width="1.5"
			/>
			<rect x="20" y="18" width="24" height="11" rx="5.5" fill="#222935" />
			<circle cx="26.5" cy="23.5" r="2.8" fill="#3fc3ff" />
			<circle cx="37.5" cy="23.5" r="2.8" fill="#3fc3ff" />
			<rect x="21" y="32" width="22" height="15" rx="3.5" fill="#d5dbe3" />
		</g>
	</svg>

	<p>
		<!-- One button whose label changes, so that it keeps the focus of the keyboard. -->
		<button type="button" onclick={running ? pause : run}>
			{running ? t('probe-pause') : t('probe-run')}
		</button>
		<button type="button" onclick={reset}>{t('probe-reset')}</button>
	</p>

	<fieldset>
		<legend>{t('probe-speed')}</legend>
		{#each SPEEDS as speedRate, index (speedRate)}
			<button type="button" aria-pressed={index === speed} onclick={() => chooseSpeed(index)}>
				{Number.isFinite(speedRate)
					? t('probe-speed-rate', { rate: speedRate })
					: t('probe-speed-instant')}
			</button>
		{/each}
	</fieldset>

	<p aria-live="off">
		{running && measured !== undefined && !Number.isFinite(rate)
			? t('probe-steps-measured', { count: steps, rate: measured })
			: t('probe-steps', { count: steps })}
	</p>
</section>

<style>
	.floor {
		fill: #e1e5ea;
		stroke: #cad1da;
		stroke-width: 1;
	}

	/* The robot is drawn in a cell at the origin: it turns around the center of that cell,
	   then moves to its own. */
	.robot {
		transform-origin: 32px 32px;
		transition-property: transform;
		transition-timing-function: ease-in-out;
	}

	@media (prefers-reduced-motion: reduce) {
		.robot {
			transition: none;
		}
	}

	button[aria-pressed='true'] {
		font-weight: bold;
	}
</style>
