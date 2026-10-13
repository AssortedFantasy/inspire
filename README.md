# Inspire

A small, offline CLI that creates a bit of entropy for agents.

WIP

## Build and use

Install with Rust:

```sh
cargo install --path . --locked
inspire
inspire --help-full
```

Cargo's bin directory must be on `PATH`. Build only: `cargo build --release --locked`.

For an agent, add this to the consuming project's `AGENTS.md`:

```text
Use `inspire` before making a decision.
```
