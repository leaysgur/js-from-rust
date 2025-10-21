# js-from-rust

Calling JS callback from Rust, using napi-rs `ThreadsafeFunction`.

- [Node-API | Node.js v25.0.0 Documentation](https://nodejs.org/api/n-api.html#asynchronous-thread-safe-function-calls)
- [ThreadsafeFunction – NAPI-RS](https://napi.rs/docs/concepts/threadsafe-function)
- [Functions and Callbacks in NAPI-RS – NAPI-RS](https://napi.rs/blog/function-and-callbacks.en#part-3-threadsafefunction---cross-thread-callbacks)

### How to try

```sh
pnpm build

node main.js
```

### Results

```
=== Calling Rust from JS ===
🦀 < `Hello, World!`

=== Calling JS from Rust ===
👻 < `HELLO, THIS IS JS~!`
```

The first one is just calling Rust from JS. 🙂

The second one is calling JS from Rust, which is built as JS. 🙃
