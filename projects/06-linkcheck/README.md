# Project 6 – `linkcheck`: an async link checker

**You'll build:** a crawler that starts from a URL (or a folder of Markdown files), finds every
link, and checks them concurrently with a configurable limit.

```
$ linkcheck https://example.com --depth 2 --concurrency 16
Checked 214 links in 6.3s
✗ 404  https://example.com/old-page        (found on /blog/2019)
✗ ERR  https://dead.example.org/           (dns error)  (found on /about)
✓ 212 ok
```

## Concepts introduced

- **`async`/`await` and futures** – Rust futures are lazy and need a runtime to drive them.
- **`tokio`** – the runtime (`#[tokio::main]`), `tokio::spawn`, `JoinSet`, `Semaphore`.
- **`reqwest`** – HTTP client (feels like `fetch`).
- **Bounded concurrency** – the Rust equivalent of `p-limit`.
- **Error handling for applications** – `anyhow` for the binary, `thiserror` for your own
  library-style error enum. When to use which.
- **Streams** – `futures::stream::iter(urls).buffer_unordered(n)` as an alternative to
  spawning tasks.

## TypeScript bridge

```ts
const limit = pLimit(16);
const results = await Promise.all(urls.map(u => limit(() => fetch(u, { method: "HEAD" }))));
```

```rust
use futures::stream::{self, StreamExt};

let results: Vec<_> = stream::iter(urls)
    .map(|u| { let client = client.clone(); async move { client.head(u).send().await } })
    .buffer_unordered(16)
    .collect()
    .await;
```

Differences worth knowing:

- Calling an `async fn` does nothing until awaited – unlike a JS `Promise`, which starts eagerly.
- `tokio::spawn` needs `'static + Send` futures, which is why you see `async move` and cloned
  `Arc`s/clients. (`reqwest::Client` is cheap to clone – it's an `Arc` inside.)
- Blocking the runtime thread (e.g. `std::thread::sleep`, heavy CPU work) stalls other tasks,
  just like blocking Node's event loop. Use `tokio::time::sleep` / `spawn_blocking`.

## Setup

`cargo new projects/06-linkcheck --name linkcheck`, add `tokio` (features `full`),
`reqwest`, `futures`, `scraper` (HTML parsing), `url`, `anyhow`, `thiserror`, `clap`.

## Milestones

1. **One request** – `#[tokio::main] async fn main() -> anyhow::Result<()>`, fetch one URL,
   print status. Add `.context("fetching …")` to errors and see how they display.
2. **Extract links** – parse the HTML with `scraper`, resolve relative links with
   `Url::join`, dedupe with a `HashSet`.
3. **Concurrent checking** – check all links with `buffer_unordered(n)`. Then rewrite with
   `JoinSet` + `Semaphore` and compare the two styles.
4. **Crawling** – breadth-first up to `--depth`, only following same-origin links. Keep a
   `visited` set. Make sure the whole thing still respects the concurrency limit.
5. **Errors & retries** – a `thiserror` enum `CheckError { Http(StatusCode), Timeout, Dns, … }`.
   Per-request timeouts, one retry with backoff for 5xx/timeouts, and treat `HEAD` 405 as
   "try `GET`".
6. **Markdown mode** – `linkcheck ./docs` scans `.md` files for links instead of crawling.
   Reuse the checking code via a trait or a shared function.

## Acceptance checklist

- [ ] Concurrency limit is honoured (test against a local server with a counter)
- [ ] Never visits the same URL twice
- [ ] A single bad host doesn't stall or crash the run
- [ ] Output lists each broken link with the page it was found on
- [ ] Integration test using a local mock server (`wiremock` or an `axum` server in the test)

## Stretch goals

- Respect `robots.txt` and a per-host rate limit.
- Live progress bar with `indicatif`.
- Run it in CI against your own blog/docs.

## Review questions

- What is a `Future` actually, and what does `.await` do to it?
- Why does `tokio::spawn` need `Send + 'static` but `buffer_unordered` doesn't?
- When would you choose `anyhow` vs `thiserror`?
- What happens if you call a blocking function inside an async task? How would you notice?
