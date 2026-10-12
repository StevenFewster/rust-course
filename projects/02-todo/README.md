# Project 2 – `todo`: a CLI with JSON storage

**You'll build:** a to-do manager that stores tasks in a JSON file.

```
$ todo add "Buy milk" --priority high
Added #1: Buy milk
$ todo add "Read chapter 6 of the book"
Added #2: Read chapter 6 of the book
$ todo list
 #1 [ ] (high)   Buy milk
 #2 [ ] (normal) Read chapter 6 of the book
$ todo done 1
$ todo list --pending
 #2 [ ] (normal) Read chapter 6 of the book
```

## Concepts introduced

- **Starting a crate yourself** – `cargo new`, adding it to the workspace, `cargo add`.
- **Structs, enums, `impl` blocks** – modelling `Task`, `Priority`, `Status`.
- **`match` and exhaustiveness** – the compiler makes sure you handle every case.
- **`Option<T>`** – `find_by_id` returns `Option<&mut Task>`, not `Task | undefined`.
- **`Vec<T>` and iterators** – filtering, finding, max id.
- **Crates:** `serde` + `serde_json` (like `JSON.parse` with types baked in), `clap` (argument
  parsing via derive – think `commander`, but the CLI shape *is* a type).
- **Modules** – split into `main.rs`, `task.rs`, `store.rs`.

## TypeScript bridge

```ts
type Priority = "low" | "normal" | "high";
interface Task { id: number; title: string; priority: Priority; done: boolean }
```

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, clap::ValueEnum)]
#[serde(rename_all = "lowercase")]
enum Priority { Low, Normal, High }

#[derive(Debug, Serialize, Deserialize)]
struct Task { id: u32, title: String, priority: Priority, done: bool }
```

The `derive`s generate code at compile time – no runtime reflection, no schema library. If the
JSON on disk doesn't match the struct, `serde_json::from_str` returns an `Err`.

The subcommands become an enum too:

```rust
#[derive(clap::Subcommand)]
enum Command {
    Add { title: String, #[arg(long, value_enum, default_value_t = Priority::Normal)] priority: Priority },
    List { #[arg(long)] pending: bool },
    Done { id: u32 },
    Remove { id: u32 },
}
```

…and `main` is one big `match` over it.

## Setup

```sh
cd projects
cargo new 02-todo --name todo
cd 02-todo
cargo add serde --features derive
cargo add serde_json
cargo add clap --features derive
```

Then add `"projects/02-todo"` to `members` in the root `Cargo.toml`.

## Milestones

1. **Model** – define `Priority` and `Task` in `src/task.rs`. Write a unit test that serialises
   a `Task` to JSON and back and checks it round-trips.
2. **Store** – in `src/store.rs`, write `load(path) -> Result<Vec<Task>, …>` and
   `save(path, &[Task])`. A missing file should load as an empty list, not an error (match on
   `io::ErrorKind::NotFound`).
3. **CLI** – use `clap` to parse `add`, `list`, `done`, `remove`. Implement each with the
   store. New ids are one more than the current max (`iter().map(|t| t.id).max()`).
4. **Errors** – `todo done 99` should print a friendly error and exit non-zero. Return
   `Result<(), String>` from a `run()` function for now; you'll learn better error types in
   project 3.
5. **Polish** – `list` sorts by priority then id (derive `PartialOrd, Ord` on `Priority` – the
   variant order is the sort order). Store the file at `~/.todo.json` or a path from a
   `TODO_FILE` env var.

## Acceptance checklist

- [ ] All four subcommands work and persist across runs
- [ ] Corrupt JSON gives a clear error, not a panic
- [ ] Unknown id gives a clear error and non-zero exit
- [ ] Code is split into at least three modules
- [ ] Tests cover JSON round-trip and the "missing file = empty list" case
- [ ] `cargo clippy` is clean; no `unwrap()` outside tests

## Stretch goals

- `todo edit <id> --title … --priority …` with optional flags (`Option<String>` in the clap struct).
- Due dates using the `chrono` or `jiff` crate, and `list --overdue`.
- Write the file atomically (write to a temp file, then `fs::rename`). Why does this matter?

## Review questions

- Why does finding a task to mark done return `Option<&mut Task>` rather than `Option<Task>`?
- What happens if you add a new variant to `Command` but forget to handle it in `main`?
- `serde` derives work without any runtime reflection. Where does the code come from?
- When would you use `if let` instead of `match`?
