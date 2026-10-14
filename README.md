# rust_cache

A Redis-inspired cache implemented in Rust, developed incrementally from a minimal in-memory key-value store toward a production-grade service.

## Project principles

- Build in small, testable stages.
- Understand the mechanism before adding an abstraction or dependency.
- Keep the core cache independent from networking where practical.
- Add tests alongside behavior.
- Record architecture decisions, commands, and lessons in `docs/PROJECT_LOG.md`.
- Treat production readiness as a set of verified capabilities, not a label.

## Current stage

**Stage 0: Initial project foundation.**

No cache commands, network server, persistence, or external dependencies have been implemented yet.

## Prerequisites

- Rust stable toolchain
- Cargo (installed with Rust)
- Git (recommended)

Install Rust using the official instructions: https://www.rust-lang.org/tools/install

Verify the installation:

```sh
rustc --version
cargo --version
rustup show
```

## Create the project

Run these commands in the directory where you keep your code:

```sh
cargo new rust_cache
cd rust_cache
```

Cargo creates a binary crate with `Cargo.toml` and `src/main.rs`.

## First verification

From the project root:

```sh
cargo check
cargo test
cargo fmt --check
cargo clippy -- -D warnings
cargo run
```

`cargo fmt --check` and Clippy are quality checks; if Clippy reports warnings in the generated starter code or after future changes, review them rather than blindly suppressing them.

## Planned structure

We will introduce files only when there is a clear reason to do so. The early target is:

```text
rust_cache/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── docs/
│   └── PROJECT_LOG.md
├── src/
│   ├── main.rs
│   ├── lib.rs
│   └── store.rs
└── tests/
```

This is a target structure, not a claim that every file already exists. `lib.rs`, `store.rs`, and `tests/` will be added when we begin implementing the core cache.

## Development workflow

For each stage:

1. Define the behavior and the concepts we need.
2. Implement the smallest useful version.
3. Add tests for expected behavior and edge cases.
4. Run formatting, tests, and linting.
5. Update `docs/PROJECT_LOG.md` with files changed, commands run, design decisions, and next steps.

## Roadmap

1. Project setup and Rust toolchain
2. In-memory `SET`, `GET`, and `DEL`
3. TCP listener and client connections
4. RESP-compatible command parsing and responses
5. Key expiration (TTL)
6. Concurrency and request isolation
7. Persistence and recovery
8. Eviction and memory limits
9. Observability, benchmarks, fuzzing, and operational hardening

The roadmap may change as implementation and tests reveal what is needed.
