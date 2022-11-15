# Pedersen Hash Benchmark

Comparison of performance of various Pedersen hash implementations.

|                 | [pathfinder](https://github.com/eqlabs/pathfinder) :crown: | [starknet-signatures](https://github.com/geometryresearch/starknet-signatures) | [starknet-rs](https://github.com/xJonathanLEI/starknet-rs) | [starknet-rs](https://github.com/xJonathanLEI/starknet-rs) (wasmtime) | [starknet-rs](https://github.com/xJonathanLEI/starknet-rs) (Node.js) | [starknet-rs](https://github.com/xJonathanLEI/starknet-rs) (wasmer) |
| --------------- | ---------------------------------------------------------- | ------------------------------------------------------------------------------ | ---------------------------------------------------------- | --------------------------------------------------------------------- | -------------------------------------------------------------------- | ------------------------------------------------------------------- |
| `pedersen_hash` | 25.604 µs                                                  | 138.83 µs                                                                      | 224.90 µs                                                  | 1.6478 ms                                                             | 1.7715 ms                                                            | 2.0084 ms                                                           |
| **Relative**    | 1.00x                                                      | 5.42x                                                                          | 8.78x                                                      | 64.36x                                                                | 69.19x                                                               | 78.44x                                                              |

## Implementations

- [geometryresearch/starknet-signatures](https://github.com/geometryresearch/starknet-signatures)
- [eqlabs/pathfinder](https://github.com/eqlabs/pathfinder)
- [xJonathanLEI/starknet-rs](https://github.com/xJonathanLEI/starknet-rs)

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
