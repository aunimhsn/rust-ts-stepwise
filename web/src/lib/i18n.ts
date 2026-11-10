// The texts of the interface, from the Fluent files of `locales/` (see docs/technical-choices.md):
// never written in the code, so that a language is added without touching it.

import { FluentBundle, FluentResource, type FluentVariable } from '@fluent/bundle';
import english from '../../locales/en.ftl?raw';

/** The values a message takes, such as `{ version: '0.1.0' }` for `{ $version }`. */
export type MessageArguments = Record<string, FluentVariable>;

/**
 * Reads a Fluent file into a bundle. A syntax error of the file is returned with the bundle
 * rather than thrown: the messages around it still load.
 */
export function createBundle(
	locale: string,
	source: string
): { bundle: FluentBundle; errors: Error[] } {
	// Without the invisible marks that isolate each value from the text around it: they only
	// matter when a right-to-left language mixes with a left-to-right one.
	const bundle = new FluentBundle(locale, { useIsolating: false });
	const errors = bundle.addResource(new FluentResource(source));
	return { bundle, errors };
}

/**
 * Formats a message of the bundle. A missing message, or a value it lacks, is written to the
 * console and never stops the page: the message shows its name, or the name of the value.
 */
export function formatMessage(bundle: FluentBundle, id: string, args?: MessageArguments): string {
	const message = bundle.getMessage(id);
	if (message?.value == null) {
		console.warn(`i18n: no message "${id}" in ${bundle.locales.join(', ')}`);
		return id;
	}
	const errors: Error[] = [];
	const text = bundle.formatPattern(message.value, args, errors);
	for (const error of errors) {
		console.warn(`i18n: message "${id}": ${error.message}`);
	}
	return text;
}

const { bundle, errors } = createBundle('en', english);
for (const error of errors) {
	console.warn(`i18n: locales/en.ftl: ${error.message}`);
}

/** The text of a message of the interface, in English for now. */
export function t(id: string, args?: MessageArguments): string {
	return formatMessage(bundle, id, args);
}
