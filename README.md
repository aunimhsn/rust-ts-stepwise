# Stepwise

Learn to program step by step, in the browser: write a small program, then watch a robot run it on a grid, one instruction at a time.

Stepwise is meant for beginners, from high school on, in class or on their own. Its language reads like Python (indentation, `if` / `else if` / `else`, `while`, functions and variables) and every error points to its line with a message written for a beginner. The language is interpreted in Rust, compiled to WebAssembly; the interface is written in TypeScript with Svelte. Everything runs in the browser: no account, no server.

The project is being built: see [specs.md](specs.md) for what the application does, and [roadmap.md](roadmap.md) for its progress.

## Layout

- `crates/core`: the language (lexer, parser, checker, step-by-step interpreter) and the map, a plain Rust library.
- `crates/wasm`: the bridge that exposes the core to the interface, compiled to WebAssembly.
- `web`: the interface, a SvelteKit application in TypeScript built as a static website; its texts in `web/locales/`.

## Requirements

- Rust 1.93 or later (stable toolchain, see `rust-toolchain.toml`, which also installs the `wasm32-unknown-unknown` target)
- The `wasm-bindgen` command-line tool, at the exact version of the `wasm-bindgen` crate in `Cargo.toml`: `cargo install wasm-bindgen-cli --version <version> --locked` (the build gives the command with its version when the tool is missing or at another version)
- Node.js 24 or later, with npm

## Build

```sh
cd web
npm install
npm run dev     # the website at http://localhost:5173, the core built in debug
npm run build   # the static website in web/build, the core optimized
```

Each command builds the core to WebAssembly first (`npm run wasm`, see `web/scripts/build-wasm.ts`), and stops at once if the `wasm-bindgen` tool isn't at the version of the crate.

## Development

The Rust code, from the root of the repository:

```sh
cargo fmt --all
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo clippy --workspace --all-features --target wasm32-unknown-unknown -- -D warnings
cargo test --workspace --all-features
```

The interface, from `web/`:

```sh
npm run format   # Prettier
npm run lint     # Prettier and ESLint
npm run check    # svelte-check: TypeScript and Svelte
npm test         # Vitest
```

## Deployment

Stepwise is a static website, published on GitHub Pages at each merge into `main` (set up by a coming step of the roadmap). A version tag, such as `v0.1.0`, names a version: it matches `version` in `[workspace.package]` of `Cargo.toml`.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.
