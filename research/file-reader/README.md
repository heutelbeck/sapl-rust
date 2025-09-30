# File-Reader

This project was a research project on how crate notify can be used to read the PDP configuration and update the configuration in the event of a change.

The pdp.json configuration is located in the policies folder and is monitored for changes.

## Prerequisites

A Rust installation is required. The easiest way to install is via [rustup](https://rustup.rs/). Use at minimum rust version 1.88.0. After installation, the application can be compiled with cargo using the following command.

```
cargo build
```

Another option is to compile the application and run it directly.

```
cargo run
```
