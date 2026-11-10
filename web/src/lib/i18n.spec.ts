import { describe, expect, it, vi } from 'vitest';
import english from '../../locales/en.ftl?raw';
import { createBundle, formatMessage, t } from './i18n.ts';

describe('locales/en.ftl', () => {
	it('reads without a syntax error', () => {
		expect(createBundle('en', english).errors).toEqual([]);
	});
});

describe('formatMessage', () => {
	const { bundle } = createBundle('en', 'greeting = Hello, { $name }!\n');

	it('fills the values of a message', () => {
		expect(formatMessage(bundle, 'greeting', { name: 'Ada' })).toBe('Hello, Ada!');
	});

	it('shows the name of a missing message and warns', () => {
		const warn = vi.spyOn(console, 'warn').mockImplementation(() => {});
		expect(formatMessage(bundle, 'farewell')).toBe('farewell');
		expect(warn).toHaveBeenCalledOnce();
		warn.mockRestore();
	});

	it('shows the name of a missing value and warns', () => {
		const warn = vi.spyOn(console, 'warn').mockImplementation(() => {});
		expect(formatMessage(bundle, 'greeting')).toBe('Hello, {$name}!');
		expect(warn).toHaveBeenCalledOnce();
		warn.mockRestore();
	});
});

describe('t', () => {
	it('formats the messages of the interface in English', () => {
		expect(t('core-version', { version: '1.2.3' })).toBe('Core 1.2.3, computed in Rust.');
	});
});
