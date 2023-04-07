//add bechmark for binary field multiplication
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use linearalgebra::{Field, Ring};

pub fn bench_binary_field_mul(c: &mut Criterion) {
    let mut group = c.benchmark_group("binary_field_mul");
    let field = linearalgebra::binary::BinaryField::<u64>::new();
    group.bench_function("binary_field_mul", |b| {
        b.iter(|| {
            let a = 0xff8f33d23232323u64;
            let b = 0xff8f33dfdf32333u64;
            let c = field.mul(&a, &b);
            black_box(c);
        })
    });
}

// add benchmark for field inversion
pub fn bench_binary_field_inv(c: &mut Criterion) {
    let mut group = c.benchmark_group("binary_field_inv");
    let field = linearalgebra::binary::BinaryField::<u64>::new();
    group.bench_function("binary_field_inv", |b| {
        b.iter(|| {
            let a = 0xff8f33d23232323u64;
            let b = field.inv(&a).unwrap();
            black_box(b);
        })
    });
}

pub fn bench_binary_field_inv_gcd(c: &mut Criterion) {
    let mut group = c.benchmark_group("binary_field_inv_gcd");
    let field = linearalgebra::binary::BinaryField::<u64>::new();
    group.bench_function("binary_field_inv", |b| {
        b.iter(|| {
            let a = 0xff8f33d23232323u64;
            let b = field.gcd_inv(&a).unwrap();
            black_box(b);
        })
    });
}

criterion_group!(
    benches,
    bench_binary_field_mul,
    bench_binary_field_inv,
    bench_binary_field_inv_gcd
);
criterion_main!(benches);
