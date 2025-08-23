[![ci-build](https://github.com/FTKeV/sapl-rust/actions/workflows/ci-build.yml/badge.svg)](https://github.com/FTKeV/sapl-rust/actions/workflows/ci-build.yml)
[![Dependency security audit](https://github.com/FTKeV/sapl-rust/actions/workflows/audit.yml/badge.svg)](https://github.com/FTKeV/sapl-rust/actions/workflows/audit.yml)
<a href="https://blog.rust-lang.org/2025/06/26/Rust-1.88.0/"><img alt="Rustc Version 1.88.0+" src="https://img.shields.io/badge/rustc-1.88.0%2B-lightgrey.svg"/></a>

# SAPL Implementation in Rust

## What is SAPL?
SAPL is a powerful policy language and engine for implementing ABAC. It comes with development tools for testing, authorization servers, and authoring tools. Framework integrations are available for Spring, Axon, and Vaadin to provide flexible policy enforcement points (PEPs) in your application.


> [!TIP]
> For an explanation, overview, and documentation about the SAPL project look up our [website](https://sapl.io).

This project implements SAPL in Rust and provides an embedded Policy Decision Point(PDP) and a PDP server with a HTTP API.

## Prerequisites

A Rust installation is required. The easiest way to install is via [rustup](https://rustup.rs/). Use at minimum rust version 1.88.0. After installation, the entire workspace can be compiled with cargo using the following command.

```
cargo build
```

All tests in the workspace can be executed with the following command.

```
cargo test
```

The only executable application in the workspace is sapl-server-rs. Information on this can be found below.

## Overview
The SAPL Rust project is divided into three parts. The following diagram illustrates the relationship between the individual parts.

![Structure](assets/structure.png)

## sapl-core

This library contains a [pest](https://pest.rs) grammar for parsing SAPL policies. The logic for evaluating a policy with a given subscription or policy validation is also implemented here.

> [!NOTE]
> A detailed description can be found in the [sapl-core readme](sapl-core/README.md). 

## embedded-pdp

This library implements a embedded [PDP](https://sapl.io/docs/3.0.0-SNAPSHOT/2_3_PolicyDecisionPoint/) with the help of sapl-core.

> [!NOTE]
> A detailed description can be found in the [embedded-pdp readme](embedded-pdp/README.md). 

## sapl-server-rs

A PDP server implemented with the web framework [Rocket](https://rocket.rs/). 

> [!NOTE]
> A detailed description can be found in the [sapl-server-rs readme](sapl-server-rs/README.md). 
