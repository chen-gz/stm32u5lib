# STM32U5 HAL Library

## Introduction

This is a Hardware Abstraction Layer (HAL) library for STM32 microcontrollers. The code is written primarily for the STM32U5 series, with a modular design that makes it easy to extend to other STM32 chips.

## Design

### Stateless Architecture
This library utilizes a stateless design paradigm. Most functions can be called at any time without tracking complex driver states. The library queries the hardware status directly to determine the next action. If a function is invalid for the peripheral's current hardware state, it returns an explicit error code.

For operations that cannot be implemented statelessly, clear warnings and explanations are provided in their respective function documentation.

### Asynchronous & Non-Blocking Operations
An interface redesign is planned to introduce comprehensive async/await support:
* All asynchronous operations will include default timeout mechanisms to ensure they do not block system execution indefinitely.

### Future Roadmap: Loopback Testing
Future updates plan to restructure the library to support self-loop (loopback) testing. This will enable automated self-testing capabilities directly on the chip for verification.


## Local Development

The project uses `devenv` to automate development tasks (if you use `direnv`, these commands are also exposed directly as shell commands):
*   `devenv shell test-host`: Runs target-independent unit tests on the host.
*   `devenv shell coverage`: Runs host tests and prints a text code coverage report.
*   `devenv shell coverage-html`: Generates an HTML coverage report at `target/llvm-cov/html/index.html`.
*   `devenv shell ci`: Runs clippy, tests, formatting check, and coverage.
