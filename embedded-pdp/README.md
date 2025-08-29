# embedded-pdp - A SAPL Policy Decision Point (PDP)
This library implements a PDP with the help of the sapl-core library.
Since this implementation cannot be accessed via a network connection, it is referred to as embedded. This library can either be used directly in an application together with a PEP or be the basis for an SAPL PDP server such as sapl-server-rs.

## How it works

## Core Components


## Performance Analysis
One approach to identifying negative effects on performance when changes are made to the code is to measure the execution speed. The crate [Criterion.rs](https://crates.io/crates/criterion) is used to detect and measure performance improvements or regressions.


This is possible with the benchmark test found at `benches/decide_once_benchmark.rs`. The decide_once method is measured. The measurement can be performed with the following command:

```
cargo bench
```

The result is displayed immediately after the measurement. A detailed report is stored in the `target` folder of the workspace. You can find a sample report [here](benchmark_report_example/criterion/decide_once/report/index.html).

> [!NOTE]
> In order to be able to create the graphics for the report, `gnuplot` must be installed.
