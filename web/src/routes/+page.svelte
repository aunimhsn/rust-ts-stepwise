<!-- The probe of step 4: a message from Fluent, a value computed by the core in Rust (step 4b),
     and a fake program advanced at each animation frame (step 4c). Replaced by the home screen
     in step 13. -->
<script lang="ts">
	import { loadCore } from '#lib/core.ts';
	import { t } from '#lib/i18n.ts';
	import Probe from '#lib/probe.svelte';

	const core = loadCore();
</script>

<svelte:head>
	<title>{t('app-name')}</title>
</svelte:head>

<main>
	<h1>{t('app-name')}</h1>
	<p>{t('app-tagline')}</p>
	{#await core then { version }}
		<p>{t('core-version', { version })}</p>
		<Probe />
	{:catch}
		<p role="alert">{t('core-load-failed')}</p>
	{/await}
</main>
