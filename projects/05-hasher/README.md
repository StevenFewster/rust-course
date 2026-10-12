# Project 5 – `hasher`: parallel file hashing

**You'll build:** a tool that walks a directory, hashes every file (SHA-256) in parallel, and
reports duplicates.

```
$ hasher ~/Downloads --dupes
Scanned 1,842 files (3.1 GB) in 2.4s using 8 threads
Duplicates:
  9f2c…e1  (2 copies, 48.2 MB each)
    ~/Downloads/video.mp4
    ~/Downloads/old/video (1).mp4
```

## Concepts introduced

- **OS threads** – `std::thread::spawn`, `JoinHandle`, `std::thread::scope`.
- **Message passing** – `std::sync::mpsc` channels (worker pool feeding results to a collector).
- **Shared state** – `Arc<T>` (shared ownership across threads) and `Mutex<T>`; why `Rc` won't
  compile here.
- **`Send` and `Sync`** – the marker traits that make data races a compile error.
- **`rayon`** – data parallelism with `par_iter()`, after you've done it by hand.
- **Streaming I/O** – hashing large files in chunks with `BufReader` instead of reading them
  into memory.

## TypeScript bridge

Node gives you one JS thread plus `worker_threads`, where data crosses via `postMessage`
(copied or transferred). Rust threads share memory directly, and the type system decides what
can be shared:

- `Send` – safe to *move* to another thread (most types).
- `Sync` – safe to *share a reference* between threads (`&T` is `Send`).
- `Rc<T>` is neither (non-atomic refcount); `Arc<T>` is both if `T` is.

If you try to share something unsafe, it doesn't compile – there's no runtime race to debug.

## Setup

`cargo new projects/05-hasher --name hasher`, add `sha2`, `walkdir`, `clap`, and later `rayon`.

## Milestones

1. **Single-threaded** – walk the directory with `walkdir`, hash each file by streaming through
   a `BufReader` in 64 KiB chunks, print `hash  path`. Time it on a large folder.
2. **Thread pool with channels** – spawn N worker threads. Send paths to workers via one
   channel and send `(path, hash)` results back via another. Notice what has to happen to the
   receiving end so several workers can share it (hint: `Arc<Mutex<Receiver<_>>>`).
3. **Scoped threads** – rewrite milestone 2 with `std::thread::scope`, which lets threads
   borrow from the enclosing stack frame. Compare the two versions.
4. **Duplicates** – collect into a `HashMap<Hash, Vec<PathBuf>>`, report groups with more than
   one file, sorted by wasted space. Skip hashing files whose size is unique (group by size first).
5. **Rayon** – replace your pool with `paths.par_iter().map(hash_file).collect()`. Measure.
   Reflect on what the library is doing for you.
6. **Progress** – a shared `AtomicUsize` counter updated by workers and printed by the main
   thread every 250 ms.

## Acceptance checklist

- [ ] Hashing a multi-GB folder doesn't load whole files into memory
- [ ] Threaded version is measurably faster than single-threaded
- [ ] Unreadable files are reported and skipped, not fatal
- [ ] Duplicates report is correct (test with a temp dir containing known copies – see `tempfile`)
- [ ] No `unwrap()` on lock results without a comment explaining why poisoning can't happen

## Stretch goals

- `--json` output for piping into other tools.
- A `--delete-dupes --dry-run` mode (be careful!).
- Swap SHA-256 for BLAKE3 and compare speed.

## Review questions

- Why does `thread::spawn` require `'static` closures, but `thread::scope` doesn't?
- What does `Arc<Mutex<T>>` give you that `Arc<T>` alone doesn't? What's the cost?
- Try to send an `Rc<String>` to a thread. Read the error – which trait is missing?
- When is a channel a better design than a shared `Mutex<HashMap>`?
