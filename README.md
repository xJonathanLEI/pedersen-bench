# Pedersen Hash Benchmark

Comparison of performance of various Pedersen hash implementations.

## Implementations

- [geometryresearch/starknet-signatures](https://github.com/geometryresearch/starknet-signatures)
- [eqlabs/pathfinder](https://github.com/eqlabs/pathfinder)
- [xJonathanLEI/starknet-rs](https://github.com/xJonathanLEI/starknet-rs)

## Results

On the author's machine with _AMD Ryzen 9 5950X 16-Core Processor_ running _Ubuntu 20.04.5 LTS_:

```log
geometryresearch/starknet-signatures@722c598
                        time:   [138.69 µs 138.83 µs 139.00 µs]
eqlabs/pathfinder@5e0f442
                        time:   [25.509 µs 25.604 µs 25.707 µs]
xJonathanLEI/starknet-rs@89a724f
                        time:   [224.40 µs 224.90 µs 225.49 µs]
```
