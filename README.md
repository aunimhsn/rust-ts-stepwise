# Stepwise

Learn to program step by step, in the browser: write a small program, then watch a robot run it on a grid, one instruction at a time.

Stepwise is meant for beginners, from high school on, in class or on their own. Its language reads like Python (indentation, `if` / `else if` / `else`, `while`, functions and variables) and every error points to its line with a message written for a beginner. The language is interpreted in Rust, compiled to WebAssembly; the interface is written in TypeScript with Svelte. Everything runs in the browser: no account, no server.

The project is being built: see [specs.md](specs.md) for what the application does, and [roadmap.md](roadmap.md) for its progress.

## Layout

- `crates/core`: the language (lexer, parser, checker, step-by-step interpreter) and the map, a plain Rust library.
- `crates/wasm`: the bridge that exposes the core to the interface, compiled to WebAssembly.

## Requirements

- Rust 1.93 or later (stable toolchain, see `rust-toolchain.toml`, which also installs the `wasm32-unknown-unknown` target)

## Build

```sh
cargo build --workspace
cargo build --release -p stepwise-wasm --target wasm32-unknown-unknown
```

## Development

```sh
cargo fmt --all
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo clippy --workspace --all-features --target wasm32-unknown-unknown -- -D warnings
cargo test --workspace --all-features
```

## Deployment

Stepwise is a static website, published on GitHub Pages at each merge into `main` (set up by a coming step of the roadmap). A version tag, such as `v0.1.0`, names a version: it matches `version` in `[workspace.package]` of `Cargo.toml`.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.
