# Project 3 – `calc`: an expression calculator

**You'll build:** a REPL that parses and evaluates arithmetic expressions with variables.

```
$ cargo run -p calc
> 1 + 2 * 3
7
> (1 + 2) * 3
9
> let r = 2.5
> 3.14159 * r * r
19.6349375
> 1 / 0
error: division by zero
> 2 +
error: unexpected end of input
```

## Concepts introduced

- **Enums that carry data** – `Token` and `Expr` are algebraic data types.
- **Recursive types and `Box<T>`** – an `Expr` contains other `Expr`s; `Box` gives it a fixed size.
- **Pattern matching in depth** – nested patterns, bindings, guards.
- **Custom error types** – an `enum CalcError` with `impl Display` and `impl std::error::Error`.
- **Iterating over `chars()` with `Peekable`** – hand-written lexer.
- **`HashMap<String, f64>`** – variable environment.
- **Splitting a crate into `lexer`, `parser`, `eval` modules**, each with their own tests.

## TypeScript bridge

```ts
type Expr =
  | { kind: "num"; value: number }
  | { kind: "var"; name: string }
  | { kind: "binary"; op: Op; left: Expr; right: Expr };
```

```rust
enum Expr {
    Num(f64),
    Var(String),
    Binary { op: Op, left: Box<Expr>, right: Box<Expr> },
}
```

In TS every object is already behind a pointer, so recursion "just works". In Rust a value's
size must be known at compile time, and an `Expr` that directly contains an `Expr` would be
infinitely large. `Box<Expr>` is a pointer to a heap-allocated `Expr`: fixed size, single owner.

## Starter code

`src/lib.rs` declares the modules and the types. Each module has `todo!()` bodies and tests.
Work through them in order:

```sh
cargo test -p calc lexer
cargo test -p calc parser
cargo test -p calc eval
```

## Milestones

1. **Lexer** – `tokenize(&str) -> Result<Vec<Token>, CalcError>`. Use
   `input.chars().peekable()` and `match` on each char. Numbers may have a decimal point;
   identifiers are letters/digits/underscores starting with a letter. Skip whitespace.
2. **Parser** – recursive descent with precedence climbing. A classic layout:
   ```
   expr   := term (('+' | '-') term)*
   term   := factor (('*' | '/') factor)*
   factor := '-' factor | NUMBER | IDENT | '(' expr ')'
   ```
   One function per rule; each consumes tokens and returns an `Expr`. Keep a position index
   into the token slice in a small `Parser` struct.
3. **Evaluator** – `eval(&Expr, &HashMap<String, f64>) -> Result<f64, CalcError>`. A single
   recursive `match`. Division by zero and unknown variables are errors.
4. **Errors** – make `CalcError` implement `Display` so the REPL prints friendly messages.
   Notice how `?` lets errors from the lexer and parser flow straight through.
5. **REPL** – loop reading lines from stdin; handle `let name = expr` (a `Statement` enum
   might help) and plain expressions; `Ctrl-D` exits cleanly.

## Acceptance checklist

- [ ] All module tests pass
- [ ] Precedence and parentheses are correct (`2 + 3 * 4 = 14`, `(2 + 3) * 4 = 20`)
- [ ] Unary minus works (`-3 * -2 = 6`)
- [ ] Variables persist across REPL lines
- [ ] Every error is a `CalcError` printed nicely – no panics on bad input
- [ ] Clippy clean

## Stretch goals

- Add `^` (power, right-associative) and functions like `sqrt(x)`, `max(a, b)`.
- Implement `impl Display for Expr` to pretty-print the parsed tree with minimal parentheses.
- Report error positions: `error at column 5: unexpected ')'`.
- Replace your error `impl`s with `#[derive(thiserror::Error)]` and compare.

## Review questions

- Why does `Expr::Binary` need `Box` but `Expr::Var(String)` doesn't?
- What would change if `eval` took `Expr` by value instead of `&Expr`?
- `tokenize` returns `Result<Vec<Token>, CalcError>`. How would you collect an iterator of
  `Result<Token, CalcError>` into that type in one line?
- What does `impl std::error::Error for CalcError` buy you?
