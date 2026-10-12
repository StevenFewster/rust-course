# Project ★ – `wasm-bridge`: Rust inside a TypeScript app (optional)

**You'll build:** your project 3 calculator compiled to WebAssembly and used from a small
TypeScript web page (Vite), with proper TS types generated for you.

This is where your two languages meet, and a realistic way Rust shows up in TS-heavy teams:
performance-sensitive or shared logic written once in Rust, consumed from the front end.

## Concepts introduced

- **`wasm-bindgen`** – exporting Rust functions and types to JavaScript.
- **`wasm-pack`** – building an npm-ready package with `.d.ts` typings.
- **Crossing the boundary** – what types can pass (numbers, strings, `Vec<u8>`, structs via
  `serde-wasm-bindgen`), and the cost of copying across it.
- **Errors across the boundary** – `Result<T, JsError>` becomes a thrown exception in JS.
- **Reusing a crate** – depending on your `calc` library from a new crate via a path dependency.

## Setup

```sh
rustup target add wasm32-unknown-unknown
cargo install wasm-pack
cargo new --lib projects/08-wasm-bridge --name calc-wasm
```

In its `Cargo.toml`:

```toml
[lib]
crate-type = ["cdylib"]

[dependencies]
calc = { path = "../03-calc" }
wasm-bindgen = "0.2"
```

## Milestones

1. **Export one function** – `#[wasm_bindgen] pub fn evaluate(src: &str) -> Result<f64, JsError>`
   wrapping your calculator. `wasm-pack build --target web`. Inspect the generated `pkg/*.d.ts`.
2. **Use it from TS** – a Vite + TypeScript page with an input box that evaluates as you type.
   Import from `pkg/` and `await init()` first.
3. **Stateful object** – export a `#[wasm_bindgen] pub struct Session` holding the variable
   map, with `new()`, `eval(&mut self, line: &str)` and `variables()` returning a JS object.
   Use it from TS like a class.
4. **Measure** – write the same evaluator in TS (or use a library) and compare speed on a
   long expression evaluated 100k times. Is crossing the boundary worth it here? Where would it be?

## Acceptance checklist

- [ ] The TS project imports the Wasm package with full type information
- [ ] Rust errors show up as catchable JS errors with your `Display` messages
- [ ] Variables persist across evaluations via the exported `Session`
- [ ] You have a written opinion on when Rust→Wasm is worth it in a TS codebase

## Review questions

- Why does `evaluate` take `&str` and what actually happens to the JS string on the way in?
- Who frees the memory of a `Session` created from JS? (Look at the generated `free()` method.)
- What would you need to change to run this in Node instead of the browser?
