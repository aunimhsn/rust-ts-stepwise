// Builds the bridge of the core to WebAssembly, then generates its JavaScript glue in
// `src/lib/wasm/` (ignored by Git).
//
//     node scripts/build-wasm.ts            the debug build, quick to compile, for `npm run dev`
//     node scripts/build-wasm.ts --release  the optimized build, for `npm run build`
//
// The glue comes from the `wasm-bindgen` command-line tool, which must be at the very version
// of the `wasm-bindgen` crate, or the module refuses to load: the version is read from
// `Cargo.lock` and checked before anything is built (see docs/technical-choices.md).

import { execFileSync } from 'node:child_process';
import { readFileSync } from 'node:fs';
import path from 'node:path';

const webDirectory = path.resolve(import.meta.dirname, '..');
const workspaceDirectory = path.resolve(webDirectory, '..');
const outDirectory = path.join(webDirectory, 'src', 'lib', 'wasm');
const release = process.argv.includes('--release');

/** Stops the script with a message, without the stack trace of an exception. */
function fail(message: string): never {
	console.error(`build-wasm: ${message}`);
	process.exit(1);
}

/** Runs a command in the workspace, its output shown as it goes. */
function run(command: string, args: string[]): void {
	execFileSync(command, args, { cwd: workspaceDirectory, stdio: 'inherit' });
}

/** The version of the `wasm-bindgen` crate the workspace is built with. */
function crateVersion(): string {
	const lock = readFileSync(path.join(workspaceDirectory, 'Cargo.lock'), 'utf8');
	const version = /\[\[package\]\]\nname = "wasm-bindgen"\nversion = "([^"]+)"/.exec(lock)?.[1];
	return version ?? fail('the wasm-bindgen crate is missing from Cargo.lock.');
}

/** The version of the `wasm-bindgen` command-line tool, or `undefined` if it isn't installed. */
function toolVersion(): string | undefined {
	try {
		const output = execFileSync('wasm-bindgen', ['--version'], { encoding: 'utf8' });
		return /^wasm-bindgen (\S+)/.exec(output)?.[1];
	} catch {
		return undefined;
	}
}

/** The folder where Cargo writes its builds, which `CARGO_TARGET_DIR` can move. */
function targetDirectory(): string {
	const metadata = execFileSync('cargo', ['metadata', '--format-version', '1', '--no-deps'], {
		cwd: workspaceDirectory,
		encoding: 'utf8'
	});
	return (JSON.parse(metadata) as { target_directory: string }).target_directory;
}

const expected = crateVersion();
const installed = toolVersion();
if (installed !== expected) {
	const found = installed === undefined ? 'is not installed' : `is at ${installed}`;
	fail(
		`the wasm-bindgen command-line tool ${found}, while the crate is at ${expected}.\n` +
			`Install it with: cargo install wasm-bindgen-cli --version ${expected} --locked`
	);
}

const profile = release ? 'release' : 'debug';
run('cargo', [
	'build',
	'--package',
	'stepwise-wasm',
	'--target',
	'wasm32-unknown-unknown',
	...(release ? ['--release'] : [])
]);
run('wasm-bindgen', [
	'--target',
	'web',
	'--out-dir',
	outDirectory,
	'--out-name',
	'stepwise',
	path.join(targetDirectory(), 'wasm32-unknown-unknown', profile, 'stepwise_wasm.wasm')
]);
