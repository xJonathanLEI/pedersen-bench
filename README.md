# Pedersen Hash Benchmark

Comparison of performance of various Pedersen hash implementations.

## Results

Results are divided into two categories: native and Node.js/WebAssembly. Node.js/WebAssembly benchmark results are only presented for Node.js as it has reasonably good WebAssembly performance with the V8 engine.

### Native results

|                 | [pathfinder](https://github.com/eqlabs/pathfinder) :crown: | [starknet-signatures](https://github.com/geometryresearch/starknet-signatures) | [starknet-rs](https://github.com/xJonathanLEI/starknet-rs) |
| --------------- | ---------------------------------------------------------- | ------------------------------------------------------------------------------ | ---------------------------------------------------------- |
| `pedersen_hash` | 25.604 µs                                                  | 138.83 µs                                                                      | 224.90 µs                                                  |
| **Relative**    | 1.00x                                                      | 5.42x                                                                          | 8.78x                                                      |

### Node.js/WebAssembly results

|                 | [starknet-rs](https://github.com/xJonathanLEI/starknet-rs) |
| --------------- | ---------------------------------------------------------- |
| `pedersen_hash` | 1.7715 ms                                                  |
| **Relative**    | 1.00x                                                      |

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
