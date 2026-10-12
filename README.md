# Rust Learning Path (for a TypeScript developer)

A project-based route from "never written Rust" to "comfortable shipping a small Rust service",
built for someone who already thinks fluently in TypeScript and has worked in Python and Java.

The path is eight small projects. Each one introduces a handful of new ideas, leans on the
previous ones, and ends with something that actually runs. Work through them in order with
Claude as a coach (see [`COACHING.md`](COACHING.md)) and tick them off in
[`PROGRESS.md`](PROGRESS.md).

## Getting started

**1. Install the toolchain** (rustup manages Rust versions, a bit like `nvm`):

```sh
# macOS / Linux / WSL
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# Windows: download and run rustup-init.exe from https://rustup.rs
```

Restart your terminal, then:

```sh
rustc --version && cargo --version
rustup component add clippy rustfmt rust-analyzer
```

**2. Set up your editor.** In VS Code, install the **rust-analyzer** extension (not the old
"Rust" one) and add this to your settings so clippy runs on save:

```json
"rust-analyzer.check.command": "clippy"
```

JetBrains users: RustRover is free for non-commercial use.

**3. Clone and check everything works:**

```sh
git clone https://github.com/<your-username>/rust-course.git
cd rust-course
cargo build
cargo test -p wordcount
```

The build should succeed and the tests should **fail** with `not yet implemented`. That's
project 1 waiting for you. (More detail and review questions in
[Project 0](projects/00-setup/README.md).)

**4. Start learning.** Open a chat with Claude and say **"let's start project 1"**.

## How to use this repo

1. Complete the [Getting started](#getting-started) steps above (= Project 0).
2. Open a chat with Claude, point it at this repo, and say **"let's start project N"**.
   Claude reads `COACHING.md` and walks you through the brief, one milestone at a time.
3. Write the code yourself. Ask for hints, not answers (the coaching notes enforce this).
4. When the acceptance checklist passes, commit, update `PROGRESS.md`, and move on.

Each project lives in `projects/NN-name/` and has:

- **README.md** – the brief: what you'll build, concepts introduced, TS ↔ Rust comparisons,
  milestones, an acceptance checklist, stretch goals and review questions.
- **A Cargo crate** (from project 1 onwards) – some ship with starter code and failing tests,
  later ones start from `cargo new` so you practise setting a project up yourself.

The crates are members of one Cargo workspace, so from the repo root you can run
`cargo test -p wordcount`, `cargo run -p todo -- add "buy milk"`, and so on.

## The path

| # | Project | Main ideas | Rough time |
|---|---------|------------|-----------|
| 0 | [Setup](projects/00-setup/README.md) | rustup, cargo, rust-analyzer, clippy, rustfmt | 1 h |
| 1 | [`wordcount` – a `wc` clone](projects/01-wordcount/README.md) | ownership & borrowing, `String` vs `&str`, `Result` and `?`, file I/O, unit tests | 3–5 h |
| 2 | [`todo` – a CLI with JSON storage](projects/02-todo/README.md) | structs, enums, `match`, `Option`, `Vec`, crates (`serde`, `clap`), modules | 4–6 h |
| 3 | [`calc` – an expression calculator](projects/03-calc/README.md) | enums with data, recursion, `Box`, custom error types, `impl Display`, iterators over chars | 5–8 h |
| 4 | [`library` – traits & generics](projects/04-library/README.md) | traits, generics vs `dyn Trait`, `HashMap`, iterator chains, closures, lifetimes | 5–8 h |
| 5 | [`hasher` – parallel file hashing](projects/05-hasher/README.md) | threads, channels, `Arc`/`Mutex`, `Send`/`Sync`, `rayon` | 4–6 h |
| 6 | [`linkcheck` – async link checker](projects/06-linkcheck/README.md) | `async`/`await`, `tokio`, `reqwest`, concurrency limits, `anyhow`/`thiserror` | 5–8 h |
| 7 | [`notes-api` – capstone REST API](projects/07-notes-api/README.md) | `axum`, `sqlx` + SQLite, shared state, integration tests, structured logging | 10–15 h |
| ★ | [`wasm-bridge` – Rust in your TS app](projects/08-wasm-bridge/README.md) *(optional)* | `wasm-bindgen`, `wasm-pack`, calling Rust from TypeScript | 3–5 h |

Total: roughly 40–60 hours, i.e. 6–10 weeks at a few evenings a week.

## Why this order

- **Ownership first, in a tiny program.** The borrow checker is the one genuinely new idea
  coming from TS/Python/Java. Project 1 is small on purpose so all your attention goes there.
- **Data modelling next.** Rust enums are TypeScript discriminated unions with the compiler
  enforcing exhaustiveness. Projects 2–3 make that your default tool.
- **Abstraction before concurrency.** Traits and generics (project 4) explain the
  `Send`/`Sync`/`Future` signatures you'll meet in projects 5–6.
- **Finish on familiar ground.** A REST API is something you already know how to design, so in
  project 7 the only unknown is Rust itself.

## Reference material to keep open

- *The Rust Programming Language* ("the book") – https://doc.rust-lang.org/book/
- *Rust by Example* – https://doc.rust-lang.org/rust-by-example/
- `std` docs – https://doc.rust-lang.org/std/ (or `rustup doc --std` offline)
- Rustlings – small exercises, good for a 15-minute warm-up – https://github.com/rust-lang/rustlings
- [`docs/ts-to-rust.md`](docs/ts-to-rust.md) – a cheat sheet mapping TS habits to Rust equivalents
