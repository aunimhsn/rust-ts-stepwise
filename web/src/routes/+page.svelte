<!-- The probe of step 4b: a message from Fluent and a value computed by the core in Rust.
     Replaced by the home screen in step 13. -->
<script lang="ts">
	import { loadCore } from '#lib/core.ts';
	import { t } from '#lib/i18n.ts';

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
	{:catch}
		<p role="alert">{t('core-load-failed')}</p>
	{/await}
</main>
