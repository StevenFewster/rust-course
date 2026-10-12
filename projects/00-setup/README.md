# Project 0 – Setup

**Goal:** a working toolchain and editor, and your first `cargo run`.

## Steps

1. Install Rust via rustup (manages compiler versions, like `nvm` for Node):
   ```sh
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
   On Windows, use the installer from https://rustup.rs.
2. Check it worked:
   ```sh
   rustc --version
   cargo --version
   ```
3. Add the components you'll use every day:
   ```sh
   rustup component add clippy rustfmt rust-analyzer
   ```
4. Editor: in VS Code install the **rust-analyzer** extension (not the old "Rust" one).
   Turn on "check on save" with clippy:
   ```json
   "rust-analyzer.check.command": "clippy"
   ```
   JetBrains users: RustRover is free for non-commercial use.
5. Clone this repo and, from the root, run:
   ```sh
   cargo build
   cargo test -p wordcount
   ```
   The build should succeed and the tests should **fail** with `not yet implemented` – that's
   project 1 waiting for you.
6. Try a throwaway project to see the full loop:
   ```sh
   cargo new hello && cd hello
   cargo run
   ```
   Change the message, run `cargo fmt` and `cargo clippy`, run again.

## Acceptance checklist

- [ ] `cargo --version` works in your terminal
- [ ] rust-analyzer shows types inline and errors as you type
- [ ] `cargo test -p wordcount` compiles and shows failing tests
- [ ] You've run `cargo fmt` and `cargo clippy` at least once

## Review questions

- What's the difference between `cargo check`, `cargo build` and `cargo run`?
- Where does the compiled binary end up? Why is `--release` noticeably slower to build?
