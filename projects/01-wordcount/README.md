# Project 1 – `wordcount`: a `wc` clone

**You'll build:** a CLI that counts lines, words and bytes in files (or stdin), like Unix `wc`.

```
$ cargo run -p wordcount -- poem.txt notes.txt
   4   21  112 poem.txt
  10   57  301 notes.txt
  14   78  413 total
```

## Concepts introduced

- **Ownership and borrowing** – the core idea of Rust. You'll pass file contents around and
  see when a value is moved vs borrowed.
- **`String` vs `&str`** – owned text vs a borrowed view of text.
- **`Result<T, E>` and `?`** – errors as return values instead of exceptions.
- **Structs and `#[derive]`** – a `Counts` struct with `Debug`, `PartialEq`, `Default`.
- **Unit tests** – `#[cfg(test)] mod tests` living next to the code.
- **`std::env::args`, `std::fs`, `std::io`** – the minimum standard library for a CLI.

## TypeScript bridge

```ts
// TS: this compiles, and both variables point at the same object
const a = { text: "hi" };
const b = a;
console.log(a.text);
```

```rust
// Rust: `a` is *moved* into `b`; using `a` afterwards is a compile error
let a = String::from("hi");
let b = a;
// println!("{a}"); // error[E0382]: borrow of moved value: `a`
```

To use a value without taking it, borrow it: `fn count(text: &str) -> Counts`. The caller keeps
ownership; `count` gets a read-only loan for the duration of the call.

```ts
// TS
function readCounts(path: string): Counts {
  const text = fs.readFileSync(path, "utf8"); // throws on error
  return count(text);
}
```

```rust
// Rust – the `?` returns early with the error, like an implicit rethrow
fn read_counts(path: &str) -> std::io::Result<Counts> {
    let text = std::fs::read_to_string(path)?;
    Ok(count(&text))
}
```

## Starter code

`src/lib.rs` has a `Counts` struct, function signatures with `todo!()` bodies, and tests.
`src/main.rs` is the CLI shell. Run the tests:

```sh
cargo test -p wordcount
```

They fail until you implement the functions. Make them pass one at a time.

## Milestones

1. **`count`** – implement `count(text: &str) -> Counts`. Words are separated by any whitespace
   (look at `str::split_whitespace`); bytes are `str::len`; lines are newline characters
   (`wc` counts `\n`, so `"a\nb"` has 1 line). Get the `count_*` tests green.
2. **`Counts` arithmetic** – implement `Counts::add` so you can build a total. Then try
   replacing it with `impl std::ops::AddAssign for Counts` so `total += c` works.
3. **Files** – implement `count_file(path: &str) -> io::Result<Counts>`. Wire up `main.rs` to
   loop over `env::args().skip(1)`, print one row per file, and a `total` row when there's
   more than one file.
4. **Errors** – a missing file should print `wordcount: nope.txt: No such file or directory`
   to **stderr** (`eprintln!`), continue with the remaining files, and exit with status 1 at
   the end (`std::process::exit` or return `ExitCode`).
5. **Stdin** – with no file arguments, read from stdin
   (`std::io::read_to_string(std::io::stdin())`). Test with `echo "a b c" | cargo run -p wordcount`.

## Acceptance checklist

- [ ] `cargo test -p wordcount` passes
- [ ] `cargo clippy -p wordcount` has no warnings
- [ ] Output columns line up for several files, with a total
- [ ] Missing files are reported on stderr without stopping the others; exit code is non-zero
- [ ] Reading from stdin works
- [ ] No `unwrap()` in `main.rs` (tests are fine)

## Stretch goals

- Add `-l`, `-w`, `-c` flags by hand (no crates yet) to show only some columns.
- Add a `chars` count and compare it to bytes on a file containing emoji. Why do they differ?
  (Hint: `str::chars`, UTF-8.)
- Read large files line by line with `BufReader::lines()` instead of loading everything.
  What changes about ownership when each line is a fresh `String`?

## Review questions

- Why does `count` take `&str` rather than `String`? What would the call site look like if it
  took `String`?
- In `for path in env::args().skip(1)`, what is the type of `path`, and who owns it?
- What does `?` desugar to? What has to be true about the function's return type for `?` to
  compile?
- `Counts` derives `Default`. What does that give you, and why might you want `Copy` too?
