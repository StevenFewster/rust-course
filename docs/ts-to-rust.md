# TypeScript → Rust cheat sheet

Quick mappings for habits you already have. Not exhaustive; add to it as you learn.

## Tooling

| TypeScript | Rust |
|---|---|
| `npm init` | `cargo new my-app` (binary) / `cargo new --lib my-lib` |
| `package.json` | `Cargo.toml` |
| `package-lock.json` | `Cargo.lock` |
| `npm install zod` | `cargo add serde --features derive` |
| `npm run build` | `cargo build` (`--release` for optimised) |
| `ts-node index.ts` | `cargo run` (`cargo run -- arg1 arg2` to pass args) |
| `jest` / `vitest` | `cargo test` (built in) |
| `eslint` | `cargo clippy` |
| `prettier` | `cargo fmt` |
| `tsc --noEmit` | `cargo check` (fast, no binary) |
| npm workspaces | Cargo workspaces (`[workspace] members = [...]`) |

## Values and types

| TypeScript | Rust | Note |
|---|---|---|
| `const x = 5` | `let x = 5;` | Immutable by default |
| `let x = 5` | `let mut x = 5;` | Mutability is opt-in |
| `number` | `i32`, `i64`, `u32`, `usize`, `f64`… | Pick a width; no implicit conversion (`as` or `try_from`) |
| `string` | `String` (owned) / `&str` (borrowed view) | Take `&str` in params, return `String` |
| `T[]` | `Vec<T>` (owned) / `&[T]` (borrowed slice) | Same owned/borrowed split |
| `Record<K, V>` / `Map` | `HashMap<K, V>`, `BTreeMap<K, V>` (sorted) | |
| `Set<T>` | `HashSet<T>`, `BTreeSet<T>` | |
| `[A, B]` tuple | `(A, B)` | |
| `T \| undefined` | `Option<T>` | `Some(v)` / `None`; no null |
| `throw` / `try…catch` | `Result<T, E>` + `?` | Errors are values |
| `interface` | `struct` (data) or `trait` (behaviour) | |
| discriminated union | `enum` with data | `match` is exhaustive by default |
| `type Id = string` | `type Id = String;` or a newtype `struct Id(String);` | Newtypes give real type safety |
| `readonly` | default | Everything is immutable unless `mut` |
| `any` | (none) | `Box<dyn Any>` exists, but you won't need it |

## Control flow

```ts
// TS
switch (shape.kind) {
  case "circle": return Math.PI * shape.r ** 2;
  case "rect":   return shape.w * shape.h;
}
```

```rust
// Rust
match shape {
    Shape::Circle { r } => std::f64::consts::PI * r * r,
    Shape::Rect { w, h } => w * h,
}
```

- `if` and `match` are expressions: `let x = if a { 1 } else { 2 };`
- The last expression in a block (no `;`) is its value – that's how most functions "return".
- `for x in items.iter()` ≈ `for (const x of items)`.
- `if let Some(v) = maybe { … }` ≈ `if (maybe !== undefined) { … }`.

## Functions and closures

| TypeScript | Rust |
|---|---|
| `(a: number) => a * 2` | `\|a: i32\| a * 2` |
| `arr.map(f).filter(g)` | `v.iter().map(f).filter(g).collect::<Vec<_>>()` |
| `arr.reduce((acc, x) => …, 0)` | `v.iter().fold(0, \|acc, x\| …)` |
| `arr.find(…)` | `v.iter().find(…)` → `Option<&T>` |
| `arr.some(…)` / `every(…)` | `.any(…)` / `.all(…)` |
| `arr.length` | `v.len()` |
| `Object.keys(o)` | `map.keys()` |

Rust iterators are lazy (like generator chains) – nothing runs until `collect`, `sum`, `for`, etc.

## Ownership in one paragraph

Every value has exactly one owner. Assigning or passing a non-`Copy` value **moves** it; the old
variable can't be used afterwards. To let someone use a value without taking it, lend a
reference: `&x` (shared, read-only, many allowed) or `&mut x` (exclusive, one at a time, no
shared refs alive simultaneously). References can't outlive what they point to. That's the whole
model – everything else (lifetimes, `Rc`, `Arc`, `RefCell`) is detail on top of it.

In TS terms: imagine every object could be referenced from only one variable, unless you
explicitly hand out a temporary read-only or read-write "loan" that the compiler tracks.

## Async

| TypeScript | Rust |
|---|---|
| `Promise<T>` | `impl Future<Output = T>` |
| `async function f()` | `async fn f()` |
| `await p` | `p.await` |
| built-in event loop | choose a runtime: `tokio` (standard choice) |
| `Promise.all([...])` | `futures::future::join_all(...)` / `tokio::join!` |
| `Promise.race` | `tokio::select!` |

Rust futures are lazy: calling an `async fn` does nothing until it's awaited or spawned.

## Errors

- Library code: define an error `enum` (often with `thiserror`).
- Application code: `anyhow::Result<T>` and `.context("what I was doing")`.
- `unwrap()` / `expect("why this can't fail")` are fine in tests and prototypes; avoid elsewhere.
