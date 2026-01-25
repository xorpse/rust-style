# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

A collection of custom Rust lints built with [Dylint](https://github.com/trailofbits/dylint). These lints enforce stylistic preferences beyond what Clippy provides.

## Build and Test Commands

```bash
# Build all lints
cargo build

# Run all tests
cargo test

# Build/test a specific lint
cargo build -p manual_type_annotations_in_let_statements
cargo test -p to_string_on_string_types
```

## Architecture

This is a Cargo workspace where each member is a standalone Dylint lint library:

- **manual_type_annotations_in_let_statements**: Warns when `let x: Type = ...` is used instead of type inference or turbofish (e.g., `let x = val.collect::<Vec<_>>()`)
- **to_string_on_string_types**: Warns when `.to_string()` is called on `String`/`&str`, suggesting `.to_owned()` instead

### Lint Structure

Each lint follows this pattern:
- `src/lib.rs`: Implements `LateLintPass` using `dylint_linting::declare_late_lint!` macro
- `ui/main.rs`: Test cases that should trigger the lint
- `ui/main.stderr`: Expected lint output (used by `dylint_testing::ui_test`)
- Compiles to `cdylib` for Dylint to load dynamically

### Key Dependencies

- `dylint_linting`: Framework for writing lints
- `dylint_testing`: UI test harness
- `clippy_utils`: Utilities for working with rustc internals (used by some lints)
- Requires `#![feature(rustc_private)]` for rustc API access

## Issue Tracking

This project uses **bd** (beads) for issue tracking. Run `bd onboard` to get started. See AGENTS.md for workflow commands.
