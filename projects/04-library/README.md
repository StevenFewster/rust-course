# Project 4 – `library`: traits, generics and lifetimes

**You'll build:** a small media-library engine (books, films, podcasts) with search, sorting,
reports and pluggable exporters, loaded from a CSV file.

```
$ library --file media.csv search "dune"
[Book]  Dune – Frank Herbert (1965) ★4.5
[Film]  Dune: Part One – Denis Villeneuve (2021) ★4.2
$ library --file media.csv report --by decade
1960s  ███ 3
2020s  ████████ 8
$ library --file media.csv export --format markdown > media.md
```

## Concepts introduced

- **Traits** – Rust's interfaces, but they can be implemented for types you didn't write.
- **Generics with trait bounds** (`fn top<T: Rated>(items: &[T])`) vs **trait objects**
  (`Vec<Box<dyn Exporter>>`) – static vs dynamic dispatch, and when to use which.
- **Standard traits**: `Display`, `FromStr`, `Ord`/`PartialOrd`, `From`/`Into`, `Iterator`.
- **Iterator chains and closures** – `filter`, `map`, `group`-style folds, `sort_by_key`,
  `Fn` vs `FnMut` vs `FnOnce`.
- **`HashMap` / `BTreeMap` with the entry API** – `*map.entry(k).or_insert(0) += 1`.
- **Lifetimes** – a search function returning `Vec<&Item>` that borrows from the library, and
  why the compiler needs to know how long those references live.

## TypeScript bridge

```ts
interface Exporter { name: string; export(items: Item[]): string }
const exporters: Exporter[] = [new CsvExporter(), new MarkdownExporter()];
```

```rust
trait Exporter {
    fn name(&self) -> &str;
    fn export(&self, items: &[Item]) -> String;
}
let exporters: Vec<Box<dyn Exporter>> = vec![Box::new(Csv), Box::new(Markdown)];
```

```ts
function search(items: Item[], q: string): Item[] { return items.filter(i => i.title.includes(q)) }
```

```rust
// The returned references can't outlive `items` – that's what 'a says.
fn search<'a>(items: &'a [Item], q: &str) -> Vec<&'a Item> {
    items.iter().filter(|i| i.title.contains(q)).collect()
}
```

## Setup

`cargo new projects/04-library --name library`, add it to the workspace. Use `clap` again, and
the `csv` crate with `serde` for loading. Create a `media.csv` with 20–30 rows of your own.

## Milestones

1. **Model** – an `enum Kind { Book, Film, Podcast }` and a `struct Item` with common fields
   (title, creator, year, rating, tags). Implement `Display` for both. Load the CSV.
2. **Searching & sorting** – `search` returning borrowed results (the lifetime example above),
   sort by year/rating/title with `sort_by` and `Ordering::then_with`. Implement
   `FromStr for SortKey` so clap can parse `--sort rating`.
3. **Traits for behaviour** – define `trait Rated { fn rating(&self) -> f32; }` and a generic
   `fn top_n<T: Rated>(items: &[T], n: usize) -> Vec<&T>`. Implement it for `Item` and for a
   second type (e.g. `Review`) to see the generic pay off.
4. **Reports** – group counts by decade, kind, and tag using `BTreeMap` and the entry API.
   Draw the bar chart with `"█".repeat(n)`.
5. **Exporters** – `trait Exporter` with CSV, Markdown and JSON implementations, stored in a
   `Vec<Box<dyn Exporter>>` and selected by name at runtime.
6. **Clone audit** – go back through projects 1–4 and remove every `.clone()` you don't need.
   For each one you keep, write down why.

## Acceptance checklist

- [ ] Search returns references, not clones (`Vec<&Item>`)
- [ ] At least one generic function with a trait bound and one use of `dyn Trait`
- [ ] Reports use iterator chains, not manual index loops
- [ ] Unit tests for search, sorting and each exporter
- [ ] Clippy clean, including `clippy::pedantic` if you're feeling brave

## Stretch goals

- Implement `Iterator` for a custom struct (e.g. paging through results 10 at a time).
- Make `top_n` accept anything iterable: `fn top_n<'a, T: Rated + 'a>(items: impl IntoIterator<Item = &'a T>, n: usize)`.
- Add a `Filter` builder (`Filter::new().kind(Kind::Book).min_rating(4.0)`) that produces a closure.

## Review questions

- When would you choose `Vec<Box<dyn Exporter>>` over a generic `Vec<T: Exporter>`? What can
  the generic version *not* do?
- What does the `'a` in `search` actually constrain? What error do you get if you remove it?
- Why can you implement `Display` for your own type but not for `Vec<Item>`? (The orphan rule.)
- What's the difference between a closure that is `Fn`, `FnMut` and `FnOnce`?
