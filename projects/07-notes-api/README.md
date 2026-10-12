# Project 7 – `notes-api`: capstone REST API

**You'll build:** a JSON REST API for notes with tags, backed by SQLite, with validation,
pagination, structured logging and integration tests. Basically the kind of Express/Fastify
service you've written before – this time in Rust.

```
POST   /notes            { "title": "...", "body": "...", "tags": ["rust"] }  → 201
GET    /notes?tag=rust&limit=20&cursor=...                                    → 200
GET    /notes/:id                                                             → 200 / 404
PATCH  /notes/:id        { "title"?: "...", "body"?: "...", "tags"?: [...] }   → 200 / 404 / 422
DELETE /notes/:id                                                             → 204 / 404
GET    /health                                                                → 200
```

## Concepts introduced

- **`axum`** – routers, handlers as plain async functions, extractors (`Path`, `Query`,
  `Json`, `State`), and `IntoResponse`.
- **Shared application state** – `State<AppState>` holding a connection pool; why it's `Clone`.
- **`sqlx` with SQLite** – async queries, migrations, compile-time checked SQL (`query_as!`).
- **Error mapping** – one `AppError` enum that implements `IntoResponse`, so handlers can use
  `?` and still return proper HTTP status codes.
- **Validation and DTOs** – separate request/response structs from the DB row struct.
- **`tracing`** – structured logs and a request-logging layer from `tower-http`.
- **Integration testing** – spin up the router in-process with a fresh SQLite DB per test.
- **Configuration** – env vars, a `Config` struct, graceful shutdown.

## TypeScript bridge

```ts
app.get("/notes/:id", async (req, res) => {
  const note = await db.notes.find(req.params.id);
  if (!note) return res.status(404).json({ error: "not found" });
  res.json(note);
});
```

```rust
async fn get_note(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<Note>, AppError> {
    let note = sqlx::query_as!(Note, "SELECT id, title, body FROM notes WHERE id = ?", id)
        .fetch_optional(&state.db)
        .await?
        .ok_or(AppError::NotFound)?;
    Ok(Json(note))
}
```

The extractors in the signature do the parsing/validation that middleware or manual code would
do in Express. If `id` isn't an integer, axum returns 400 before your handler runs.

## Setup

`cargo new projects/07-notes-api --name notes-api`, add `axum`, `tokio` (full), `serde`,
`serde_json`, `sqlx` (features `runtime-tokio`, `sqlite`, `macros`, `migrate`), `tracing`,
`tracing-subscriber`, `tower-http` (features `trace`), `thiserror`, `anyhow`. Install the CLI:
`cargo install sqlx-cli --no-default-features --features sqlite`.

## Milestones

1. **Hello axum** – `/health` returns `{"status":"ok"}`. Add `tracing` and the `TraceLayer`.
2. **Database** – a migration creating `notes` and `note_tags` tables; `AppState { db: SqlitePool }`;
   run migrations on startup.
3. **CRUD** – implement the five note endpoints with `query_as!`. Separate `CreateNote`,
   `UpdateNote` (all `Option` fields) and `NoteResponse` structs.
4. **Errors** – `AppError` with `NotFound`, `Validation(String)`, `Internal(anyhow::Error)`;
   `impl IntoResponse` returning JSON bodies. `From<sqlx::Error>` so `?` works. Never leak
   internal error details to the client; log them instead.
5. **Tags & pagination** – filter by tag, cursor-based pagination (`?cursor=<last id>`), use a
   transaction when updating a note's tags.
6. **Tests** – build the `Router` from a function so tests can call it with
   `tower::ServiceExt::oneshot` against an in-memory SQLite DB. Cover the happy path and every
   error status.
7. **Ship it** – config from env (`DATABASE_URL`, `PORT`), graceful shutdown on Ctrl-C, a
   multi-stage `Dockerfile` producing a small image, `cargo build --release` and look at the
   binary size.

## Acceptance checklist

- [ ] All endpoints behave as specified, including status codes
- [ ] SQL is checked at compile time (`query!`/`query_as!`), with `sqlx prepare` for offline builds
- [ ] Handlers contain no `unwrap()`; every failure maps to an `AppError`
- [ ] Integration tests cover each endpoint and error case
- [ ] Logs show method, path, status and latency for every request
- [ ] Runs in Docker

## Stretch goals

- Full-text search with SQLite FTS5.
- Auth with API keys via a custom extractor (`impl FromRequestParts`).
- OpenAPI docs with `utoipa`.
- Call your own `linkcheck` (project 6) as a library to validate links in note bodies.
- Write a tiny TypeScript client and compare the developer experience end to end.

## Review questions

- How does axum know how to build a `Json<CreateNote>` from a request? What trait is involved?
- Why must `AppState` be `Clone`, and why is cloning a `SqlitePool` cheap?
- What does `From<sqlx::Error> for AppError` let you write in handlers?
- Compare this service to an Express equivalent: what was easier, what was harder, and where
  did the compiler catch something TypeScript wouldn't have?
