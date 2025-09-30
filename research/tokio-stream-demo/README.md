# tokio stream demo

The project was created at the beginning of rust sapl development to explore the handling of streams and evaluate different approaches.

All test functions are called in the file `src/main.rs` in the `main()` function. Unwanted functions can be commented out here.

## Boolean streams
There are two variants of boolean streams that toggle with a defined clock cycle. The first variant consists of the two files
* `src/boolean_stream.rs` implements the trait (interface) Stream with the method `poll_next()`
* `src/delay.rs` implements the trait (interface) Future with the method `poll()`

The second variant can be found in the function `simple_stream_1()`. It uses an AtomicBool and the Struct IntervalStream. A very compact solution.

## Eager variant
* The first variant is a manual implementation, which can be found in the file `src/combine_eager.rs`. The last value of both streams is stored in the struct `CombineEager`. In the method `poll_next()`, both streams are queried in lines 62-63 and the result is processed as a tuple in the match block. The application can be found in the file `src/main.rs`  in the function `combine_eager_solution1()`.
* The second variant works heavily with Marcos. This results in significantly shorter code, with a lot of abstraction. However, you also need to know how to get the respective Marko to generate the desired code. The Marko solution can be found in the function `both_true_stream()` in `src/main.rs`.

## Lazy variant
I have not yet succeeded in finding a working solution using Markos. A manual implementation can be found in `src/combine_lazy.rs`. The last state is stored again in the corresponding struct. In the `poll_next()` method, stream a is always queried first. As long as it returns true stream b is queried. If stream b also returns true, the `poll_next()` function returns true.
The application of this implementation can be found in the `src/main.rs` file with the function ` combine_lazy_solution1()`.

## Prerequisites

A Rust installation is required. The easiest way to install is via [rustup](https://rustup.rs/). Use at minimum rust version 1.88.0. After installation, the application can be compiled with cargo using the following command.

```
cargo build
```

Another option is to compile the application and run it directly.

```
cargo run
```
