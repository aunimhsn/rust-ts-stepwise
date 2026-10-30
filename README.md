# Stepwise

Learn to program step by step, in the browser: write a small program, then watch a robot run it on a grid, one instruction at a time.

Stepwise is meant for beginners, from high school on, in class or on their own. Its language reads like Python (indentation, `if` / `else if` / `else`, `while`, functions and variables) and every error points to its line with a message written for a beginner. The language is interpreted in Rust, compiled to WebAssembly; the interface is written in TypeScript with Svelte. Everything runs in the browser: no account, no server.

The project is in its specification phase: see [specs.md](specs.md) for what the application does, and [roadmap.md](roadmap.md) for its progress.

## Requirements

- Rust 1.93 or later (stable toolchain, see `rust-toolchain.toml`)

## Build

```sh
cargo build --release
```

## Development

```sh
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
```

## Release

The release workflow still comes from the command-line template: a roadmap step replaces it with the deployment of the website.

1. Update `version` in `Cargo.toml` (then run `cargo build` to update `Cargo.lock`) and merge it into `main`.
2. Tag the merged commit with the same version and push the tag:

   ```sh
   git switch main && git pull
   git tag v0.1.0
   git push origin v0.1.0
   ```

The [Release workflow](.github/workflows/release.yml) builds the archives for Windows, macOS and Linux and publishes them as a GitHub release. It fails if the tag does not match the version of `Cargo.toml`.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.
