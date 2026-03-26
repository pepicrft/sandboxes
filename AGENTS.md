# Agents

This file provides context for AI agents working on this codebase.

## Project Overview

libsandbox is a Rust library that provides a universal interface for interacting with cloud sandbox providers. It uses a trait-based adapter pattern where each sandbox provider (Daytona, Modal, Fly.io, etc.) implements a common set of traits.

## Architecture

- `src/lib.rs` — Public API re-exports
- `src/models.rs` — Core data types (Sandbox, Command, ExecResult, FileEntry, etc.)
- `src/traits.rs` — Core traits (SandboxProvider, CommandExecutor, FileSystem, Snapshottable)
- `src/error.rs` — Error types
- `src/providers/` — One module per provider adapter
- `src/providers/daytona.rs` — Daytona adapter (first implementation)

## Conventions

- Use `thiserror` for error types
- Use `async-trait` for async trait definitions
- Use `reqwest` for HTTP clients
- Use `serde` for serialization/deserialization
- Use `tokio` as the async runtime
- Provider adapters implement the core traits from `src/traits.rs`
- Each provider is behind a cargo feature flag (e.g., `features = ["daytona"]`)
- Tests go in `tests/` directory, with integration tests per provider

## Commands

- `cargo build` — Build the library
- `cargo test` — Run all tests
- `cargo test --features daytona` — Run tests for a specific provider
- `cargo clippy` — Lint
- `cargo fmt` — Format
