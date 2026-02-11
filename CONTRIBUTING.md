# Contributing

Thank you for your interest in contributing to Rusty SunSpec Collector.

## Getting Started

1. Fork the repository and clone it locally
2. Install prerequisites: Rust 1.80+, cmake, libssl-dev, pkg-config
3. Build: `cargo build --workspace`
4. Test: `cargo test --workspace`

See the [Developer Guide](https://imrans110.github.io/rusty-sunspec-collector/DEVELOPER_GUIDE.html) for full setup instructions.

## Before Submitting a PR

Run the same checks that CI enforces:

```sh
cargo fmt --all -- --check
cargo clippy --all-targets --workspace -- -D warnings
cargo build --workspace
cargo test --workspace
```

## Conventions

- **Error handling**: Library crates use `thiserror`; the application crate uses `anyhow`.
- **Logging**: Use `tracing` macros with structured fields.
- **No `#![allow(dead_code)]`**: Address warnings before merging.
- **Keep `types` lightweight**: Only `serde` as a dependency.
- **Formatting**: `rustfmt.toml` enforces max width 100 and auto-reordered imports.

## What to Work On

Check the [backlog in plan.md](docs/plan.md) or open a GitHub Issue to discuss your idea before starting large changes.

## License

By contributing, you agree that your contributions will be dual-licensed under MIT and Apache-2.0.
