# SAPL meets Rust

## Async Rust
Rust hat keine async Runtime bei Default. Es muus/kann die zum Projekt passende Runtime gewählt werden. Es gibt z.B. die folgenden Runtimes
* [Tokio](https://tokio.rs/)
* [Smol](https://github.com/smol-rs/smol)
* [async-std](https://async.rs/) [streams sind hier noch auf der TODO Liste](https://book.async.rs/concepts/streams)
* [Embassy](https://embassy.dev/) (embedded async runtime)

### Tokio
Tokio ist bis jetzt die umfangreichste async Runtime mit dazugehöriger Dokumentation. Hier gibt es das [Stream Konzept](https://tokio.rs/tokio/tutorial/streams). Aktuell ist dies noch in einem extra Crate [tokio-stream](https://crates.io/crates/tokio-stream).
Laut Doku soll das Crate in Tokio integriert werden, sobald das Stream Trait in der Rust Standard Bibliothek stabil ist.

> A stream is an asynchronous series of values. It is the asynchronous equivalent to Rust's std::iter::Iterator and is represented by the Stream trait. Streams can be iterated in async functions. They can also be transformed using adapters. Tokio provides a number of common adapters on the StreamExt trait.

### Eyeball
Das Crate [eyeball](https://crates.io/crates/eyeball) beschreibt sich selbst wie folgt
> This crate implements a basic form of the Observer pattern for Rust.

Ein interessanter Blog Post zu eyeball gibt es [hier](https://mnt.io/series/reactive-programming-in-rust/observability/).
