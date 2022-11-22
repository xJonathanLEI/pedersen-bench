# Pedersen Hash Benchmark

Comparison of performance of various Pedersen hash implementations.

## Results

Results are divided into two categories: native and Node.js/WebAssembly. Node.js/WebAssembly benchmark results are only presented for Node.js as it has reasonably good WebAssembly performance with the V8 engine.

### Native results

|                 | [pathfinder](https://github.com/eqlabs/pathfinder) :crown: | [starknet-rs](https://github.com/xJonathanLEI/starknet-rs) | [starknet-signatures](https://github.com/geometryresearch/starknet-signatures) |
| --------------- | ---------------------------------------------------------- | ---------------------------------------------------------- | ------------------------------------------------------------------------------ |
| `pedersen_hash` | 27.858 µs                                                  | 31.990 µs                                                  | 143.21 µs                                                                      |
| **Relative**    | 1.00x                                                      | 1.15x                                                      | 5.14x                                                                          |

### Node.js/WebAssembly results

|                 | [starknet-rs](https://github.com/xJonathanLEI/starknet-rs) :crown: | [paulmillr/micro-starknet](https://github.com/paulmillr/micro-starknet) |
| --------------- | ------------------------------------------------------------------ | ----------------------------------------------------------------------- |
| `pedersen_hash` | 228.67 µs                                                          | 1.573 ms                                                                |
| **Relative**    | 1.00x                                                              | 6.88x                                                                   |

## Implementations

Here's the list of implementations tested along with their platform availability.

| Implementation                                                                                  | Native             | Node.js/WebAssembly |
| ----------------------------------------------------------------------------------------------- | ------------------ | ------------------- |
| [geometryresearch/starknet-signatures](https://github.com/geometryresearch/starknet-signatures) | :white_check_mark: | :x:                 |
| [eqlabs/pathfinder](https://github.com/eqlabs/pathfinder)                                       | :white_check_mark: | :x:                 |
| [xJonathanLEI/starknet-rs](https://github.com/xJonathanLEI/starknet-rs)                         | :white_check_mark: | :white_check_mark:  |
| [paulmillr/micro-starknet](https://github.com/paulmillr/micro-starknet)                         | :x:                | :white_check_mark:  |

## Environment

- **CPU**

  _AMD Ryzen 9 5950X 16-Core Processor_

- **OS**

  _Ubuntu 20.04.5 LTS_

- Runtimes

  - `wasmer-js`: `wasmer-js 0.4.1`
  - Node.js: `v18.12.1`

## Running benchmarks

### Native benchmarks

To run the native benchmarks:

```console
$ cargo bench
```

### WebAssembly benchmarks

To run the WebAssembly benchmarks, make sure you have `wasm32-wasi` target and `cargo-wasi` installed. Then build the wasm file:

```console
$ ./scripts/build_bench_wasm.sh
```

Then you can run the benchmark with your target runtime. For example, to run the benchmark against `wasmer-js` (Node.js):

```console
$ ./scripts/run_bench_wasm.sh wasmer-js
```

For more information regarding wasm benchmarks with `criterion.rs`, check out [this guide](https://github.com/bheisler/criterion.rs/blob/version-0.4/book/src/user_guide/wasi.md).

### Node.js benchmarks

To run non-wasm Node.js benchmarks, first build the benchmark script:

```console
$ yarn install
$ yarn build
```

Then run the script with `node`:

```console
$ node build/js-bench.js
```
