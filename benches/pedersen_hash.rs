use ark_ff::field_new;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use stark_hash::{stark_hash, StarkHash};
use starknet_curve::Fq;
use starknet_signature::pedersen_hash;

pub fn criterion_benchmark(c: &mut Criterion) {
    let e0 = field_new!(
        Fq,
        "1740729136829561885683894917751815192814966525555656371386868611731128807883"
    );
    let e1 = field_new!(
        Fq,
        "919869093895560023824014392670608914007817594969197822578496829435657368346"
    );

    c.bench_function("geometryresearch/starknet-signatures@722c598", |b| {
        b.iter(|| {
            black_box(pedersen_hash(&e0, &e1));
        });
    });

    let e0 = "03d937c035c878245caf64531a5756109c53068da139362728feb561405371cb";
    let e1 = "0208a0a10250e382e1e4bbe2880906c2791bf6275695e02fbbc6aeff9cd8b31a";

    let e0 = StarkHash::from_hex_str(e0).unwrap();
    let e1 = StarkHash::from_hex_str(e1).unwrap();

    c.bench_function("eqlabs/pathfinder@5e0f442", |b| {
        b.iter(|| {
            black_box(stark_hash(e0, e1));
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
