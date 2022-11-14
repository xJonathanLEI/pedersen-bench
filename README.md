# Pedersen Hash Benchmark

Comparison of performance of various Pedersen hash implementations.

## Implementations

- [geometryresearch/starknet-signatures](https://github.com/geometryresearch/starknet-signatures)
- [eqlabs/pathfinder](https://github.com/eqlabs/pathfinder)

## Results

On the author's machine with _AMD Ryzen 9 5950X 16-Core Processor_ running _Ubuntu 20.04.5 LTS_:

```log
geometryresearch/starknet-signatures@722c598
                        time:   [134.36 µs 134.94 µs 135.50 µs]
eqlabs/pathfinder@5e0f442
                        time:   [25.748 µs 25.831 µs 25.923 µs]
```
