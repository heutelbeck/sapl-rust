# Pest-Parser-Demo

This application is intended for development purposes only. The sapl-core crate is used to parse various policies and output the results of the parsing process.

The application can easily be extended with a new policy. To do this, simply insert a new block with the desired policy at the end of the parse_demo_policies() function in main.rs.

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
