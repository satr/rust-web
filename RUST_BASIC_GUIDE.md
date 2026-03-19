# Rust basic guide

## 1. Core language
* `let` is immutable by default, mut makes mutation explicit.
* Ownership means one owner for a value at a time.
* Borrowing means temporary access via `&T` or `&mut T`.
* Rust allows many immutable borrows or one mutable borrow, not both.
* Lifetimes describe how long references are valid; mostly inferred.
* `Option<T>` replaces `null`.
* `Result<T, E>` models success/failure explicitly.
* `?` propagates `Err(...)` or `None` early.

## 2. Strings and references
* `String` = owned, growable heap string.
* `&str` = borrowed string slice.
* Prefer `&str` in parameters when ownership is not needed.

## 3. Traits and abstraction
* _Traits_ are Rust’s interface-like abstraction.
* `T: Trait` = generic/static dispatch.
* `dyn Trait` = trait object/dynamic dispatch.
* `Arc<dyn Trait + Send + Sync>` is common for app wiring / DI style.
* `#[async_trait]` is commonly used when trait methods are async and you want trait objects.

## 4. Error handling
* Prefer `enum` errors over `String`.
* Implement `IntoResponse` on app errors in `Axum` to centralize HTTP mapping.
* Handlers then return `Result<_, AppError>` cleanly.

## 5. Async basics
* `async fn` returns a `Future`.
* Rust futures are lazy: they do nothing until polled / awaited / spawned.
* `.await` suspends the current task if the future is not ready.
* `Tokio` runtime polls futures and wakes them when ready.

## 6. Tokio/runtime
* `Tokio` uses lightweight tasks, not one thread per request.
* `tokio::spawn` creates concurrent async tasks.
* `spawn_blocking` is for CPU-heavy or blocking synchronous work.
* Do not block runtime worker threads with heavy sync code.

## 7. Pin
* Async functions become state machines.
* `Futures` may contain self-references internally.
* Moving such a future could invalidate those references.
* `Pin` guarantees the future won’t move once polling starts.

## 8. Shared state and locks
* `Arc` = shared ownership across threads.
* `Mutex` / `RwLock` = mutation control, not ownership.
* `Arc<RwLock<T>>` means shared ownership + controlled access.
* `RwLock` allows many readers or one writer.

## 9. Locking rules in async code
* Keep lock scope as short as possible.
* Do not `.await` while holding a lock _guard_ unless absolutely necessary.
* Holding a guard means the guard variable is still alive.
* Re-locking the same async lock in the same task **can deadlock**.

## 10. Channels instead of locks
* _Actor_ model: one task owns state, others send messages.
* `tokio::mpsc` is useful when you want to avoid shared mutable state.
* Good alternative to `Arc<Mutex<_>>` / `Arc<RwLock<_>>` for some designs.

## 11. Axum basics
* `Router` defines routes and shared state.
* Handlers are async functions using extractors like `State<T>` and `Json<T>`.
* Shared app dependencies are often stored in _AppState_.
* `Axum` handlers often return `Result<Json<_>, AppError>`.

## 12. Middleware and tracing
* `TraceLayer` adds request-level logging/tracing.
* `tracing` is the standard structured logging ecosystem.
* Better than `println!` for production async services.

## 13. Clean layered architecture
* handlers = HTTP layer
* services = business logic
* repositories = storage/integration
* app state = wiring / dependencies
* This is very close to C#/Go backend layering, just with Rust’s ownership and async model.

## 14. Testing
* repository tests validate storage behavior
* service tests validate business rules
* handler tests validate HTTP wiring, JSON, and error mapping
* That is exactly the right testing pyramid for your project.