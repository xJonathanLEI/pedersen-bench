# Pedersen Hash Benchmark

Comparison of performance of various Pedersen hash implementations.

|                 | [pathfinder](https://github.com/eqlabs/pathfinder) :crown: | [starknet-signatures](https://github.com/geometryresearch/starknet-signatures) | [starknet-rs](https://github.com/xJonathanLEI/starknet-rs) | [starknet-rs](https://github.com/xJonathanLEI/starknet-rs) (wasmtime) | [starknet-rs](https://github.com/xJonathanLEI/starknet-rs) (Node.js) | [starknet-rs](https://github.com/xJonathanLEI/starknet-rs) (wasmer) |
| --------------- | ---------------------------------------------------------- | ------------------------------------------------------------------------------ | ---------------------------------------------------------- | --------------------------------------------------------------------- | -------------------------------------------------------------------- | ------------------------------------------------------------------- |
| `pedersen_hash` | 25.604 µs                                                  | 138.83 µs                                                                      | 224.90 µs                                                  | 1.6478 ms                                                             | 1.7715 ms                                                            | 2.0084 ms                                                           |
| **Relative**    | 1.00x                                                      | 5.42x                                                                          | 8.78x                                                      | 64.36x                                                                | 69.19x                                                               | 78.44x                                                              |

## Implementations

Here's the list of implementations tested along with their platform availability.

| Implementation                                                                                  | Native             | Node.js/WebAssembly |
| ----------------------------------------------------------------------------------------------- | ------------------ | ------------------- |
| [geometryresearch/starknet-signatures](https://github.com/geometryresearch/starknet-signatures) | :white_check_mark: | :x:                 |
| [eqlabs/pathfinder](https://github.com/eqlabs/pathfinder)                                       | :white_check_mark: | :x:                 |
| [xJonathanLEI/starknet-rs](https://github.com/xJonathanLEI/starknet-rs)                         | :white_check_mark: | :white_check_mark:  |

## Environment

- **CPU**

  _AMD Ryzen 9 5950X 16-Core Processor_

- **OS**

  _Ubuntu 20.04.5 LTS_

- Runtimes

  - `wasmer`: `wasmer 2.3.0`
  - `wasmtime`: `wasmtime-cli 2.0.2`
  - `wasmer-js`: `wasmer-js 0.4.1`
  - Node.js: `v18.12.1`

## Running benchmarks

To run the native benchmarks:

```console
$ cargo bench
```

To run the WebAssembly benchmarks, make sure you have `wasm32-wasi` target and `cargo-wasi` installed. Then build the wasm file:

```console
$ ./scripts/build_bench_wasm.sh
```

Then you can run the benchmark with your taret runtime. For example, to run the benchmark against `wasmer`:

```console
$ ./scripts/run_bench_wasm.sh wasmer
```

For more information regarding wasm benchmarks, check out [this guide](https://github.com/bheisler/criterion.rs/blob/version-0.4/book/src/user_guide/wasi.md).
