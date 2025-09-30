# Demo embedded-pdp

This application is a minimal example of how the crate embedded-pdp can be used. The application uses the crate embedded-pdp for the SAPL functionalities.

A pdp instance is created in `main()` and passed as a parameter to the two functions `decide()` and `decide-once()`. The `decide-once()` method is synchronous and prints the result it gets from the `pdp.decide_once()` function.
The `decide()` method consumes a stream from the pdp instance and print every update as soon as the pdp decision changes.

> [!NOTE]
> It is best to clone the entire repository, then this application can be started without having to change any paths.

## Prerequisites

A Rust installation is required. The easiest way to install is via [rustup](https://rustup.rs/). Use at minimum rust version 1.88.0. After installation, the application can be compiled with cargo using the following command.

```
cargo build
```

Another option is to compile the application and run it directly.

```
cargo run
```
