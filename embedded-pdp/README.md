# embedded-pdp - A SAPL Policy Decision Point (PDP)
This library implements a PDP with the help of the sapl-core library.
Since this implementation cannot be accessed via a network connection, it is referred to as embedded. This library can either be used directly in an application together with a PEP or be the basis for an SAPL PDP server such as sapl-server-rs.

> [!NOTE]
> It is best to clone the entire repository, then this crate can be build without having to change any paths.

> [!NOTE]
> The crate is not published on [crates.io](https://crates.io/) and therefore cannot be imported from there via `cargo`.

## Prerequisites

A Rust installation is required. The easiest way to install is via [rustup](https://rustup.rs/). Use at minimum rust version 1.88.0. After installation, the application can be compiled with cargo using the following command.

```
cargo build
```

## How to use this crate
This library can be added as a dependency in other Rust projects to integrate SAPL pdp functionalities. 

A minimal example of integration and usage is [embedded-pdp-demo](../research/embedded-pdp-demo/README.md).

Another more detailed example of the usage of this library is [SAPL Server RS](../sapl-server-rs/README.md).

## Overview
```
└── src
    ├── file_reader.rs
    ├── lib.rs
    └── pdp_config.rs
```
### lib.rs
In `src/lib.rs`, the struct Pdp is defined and an implementation of the function `new()` is provided to create an instance. A configuration and a path where the policies are stored are required.
The two API endpoints `decide()` and `decide-once()` are also implemented here. This is the interface of the pdp intended for interaction.

### file_reader.rs
The logic for monitoring the directory for changes to the configuration or policies is contained here. The crate notify is used for the implementation.

### pdp_config.rs
The struct PdpConfig is defined here. The function `PdpConfig::new()` creates an instance. If the `pdp.json` configuration cannot be read, an instance is created using the `default()` method. There is also the `update_algorithm()` method, which is called when the configuration is changed in order to update the combination algorithm.

## Performance Analysis
One approach to identifying negative effects on performance when changes are made to the code is to measure the execution speed. The crate [Criterion.rs](https://crates.io/crates/criterion) is used to detect and measure performance improvements or regressions.

This is possible with the benchmark test found at `benches/decide_once_benchmark.rs`. The decide_once method is measured. The measurement can be performed with the following command:

```
cargo bench
```

The result is displayed immediately after the measurement. A detailed report is stored in the `target` folder of the workspace. You can find a sample report [here](benchmark_report_example/criterion/decide_once/report/index.html).

> [!NOTE]
> In order to be able to create the graphics for the report, `gnuplot` must be installed.
