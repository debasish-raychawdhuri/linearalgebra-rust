//add bechmark for binary field multiplication
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use linearalgebra::{Field, Ring};

//write same benchmarks as above for u8
pub fn bench_binary_field_mul_u8(c: &mut Criterion) {
    let mut group = c.benchmark_group("binary_field_mul_u8");
    let field = linearalgebra::binary::BinaryField::<u8>::new();
    group.bench_function("binary_field_mul_u8", |b| {
        b.iter(|| {
            let a = 0x8f;
            let b = 0x33;
            let c = field.mul(&a, &b);
            black_box(c);
        })
    });
}

pub fn bench_binary_field_inv_u8(c: &mut Criterion) {
    let mut group = c.benchmark_group("binary_field_inv_u8");
    let field = linearalgebra::binary::BinaryField::<u8>::new();
    group.bench_function("binary_field_inv_u8", |b| {
        b.iter(|| {
            let a = 0x8f;
            let b = field.inv(&a).unwrap();
            black_box(b);
        })
    });
}

pub fn bench_binary_field_inv_gcd_u8(c: &mut Criterion) {
    let mut group = c.benchmark_group("binary_field_inv_gcd_u8");
    let field = linearalgebra::binary::BinaryField::<u8>::new();
    group.bench_function("binary_field_inv_gcd_u8", |b| {
        b.iter(|| {
            let a = 0x8f;
            let b = field.gcd_inv(&a).unwrap();
            black_box(b);
        })
    });
}

//add the same benchmarks for u16
pub fn bench_binary_field_mul_u16(c: &mut Criterion) {
    let mut group = c.benchmark_group("binary_field_mul_u16");
    let field = linearalgebra::binary::BinaryField::<u16>::new();
    group.bench_function("binary_field_mul_u16", |b| {
        b.iter(|| {
            let a = 0x8f33;
            let b = 0x33df;
            let c = field.mul(&a, &b);
            black_box(c);
        })
    });
}

pub fn bench_binary_field_inv_u16(c: &mut Criterion) {
    let mut group = c.benchmark_group("binary_field_inv_u16");
    let field = linearalgebra::binary::BinaryField::<u16>::new();
    group.bench_function("binary_field_inv_u16", |b| {
        b.iter(|| {
            let a = 0x8f33;
            let b = field.inv(&a).unwrap();
            black_box(b);
        })
    });
}

pub fn bench_binary_field_inv_gcd_u16(c: &mut Criterion) {
    let mut group = c.benchmark_group("binary_field_inv_gcd_u16");
    let field = linearalgebra::binary::BinaryField::<u16>::new();
    group.bench_function("binary_field_inv_gcd_u16", |b| {
        b.iter(|| {
            let a = 0x8f33;
            let b = field.gcd_inv(&a).unwrap();
            black_box(b);
        })
    });
}

//add the same benchmarks for u32
pub fn bench_binary_field_mul_u32(c: &mut Criterion) {
    let mut group = c.benchmark_group("binary_field_mul_u32");
    let field = linearalgebra::binary::BinaryField::<u32>::new();
    group.bench_function("binary_field_mul_u32", |b| {
        b.iter(|| {
            let a = 0x8f33d232;
            let b = 0x33dfdf32;
            let c = field.mul(&a, &b);
            black_box(c);
        })
    });
}

pub fn bench_binary_field_inv_u32(c: &mut Criterion) {
    let mut group = c.benchmark_group("binary_field_inv_u32");
    let field = linearalgebra::binary::BinaryField::<u32>::new();
    group.bench_function("binary_field_inv_u32", |b| {
        b.iter(|| {
            let a = 0x8f33d232;
            let b = field.inv(&a).unwrap();
            black_box(b);
        })
    });
}

pub fn bench_binary_field_inv_gcd_u32(c: &mut Criterion) {
    let mut group = c.benchmark_group("binary_field_inv_gcd_u32");
    let field = linearalgebra::binary::BinaryField::<u32>::new();
    group.bench_function("binary_field_inv_gcd_u32", |b| {
        b.iter(|| {
            let a = 0x8f33d232;
            let b = field.gcd_inv(&a).unwrap();
            black_box(b);
        })
    });
}

//add the same benchmarks for u64
pub fn bench_binary_field_mul_u64(c: &mut Criterion) {
    let mut group = c.benchmark_group("binary_field_mul_u64");
    let field = linearalgebra::binary::BinaryField::<u64>::new();
    group.bench_function("binary_field_mul_u64", |b| {
        b.iter(|| {
            let a = 0x8f33d232df323232;
            let b = 0x33dfdf3232322323;
            let c = field.mul(&a, &b);
            black_box(c);
        })
    });
}

pub fn bench_binary_field_inv_u64(c: &mut Criterion) {
    let mut group = c.benchmark_group("binary_field_inv_u64");
    let field = linearalgebra::binary::BinaryField::<u64>::new();
    group.bench_function("binary_field_inv_u64", |b| {
        b.iter(|| {
            let a = 0x8f33d232df323232;
            let b = field.inv(&a).unwrap();
            black_box(b);
        })
    });
}

pub fn bench_binary_field_inv_gcd_u64(c: &mut Criterion) {
    let mut group = c.benchmark_group("binary_field_inv_gcd_u64");
    let field = linearalgebra::binary::BinaryField::<u64>::new();
    group.bench_function("binary_field_inv_gcd_u64", |b| {
        b.iter(|| {
            let a = 0x8f33d232df323232;
            let b = field.gcd_inv(&a).unwrap();
            black_box(b);
        })
    });
}

pub fn bench_binary_field_mul_u128(c: &mut Criterion) {
    let mut group = c.benchmark_group("binary_field_mul_u128");
    let field = linearalgebra::binary::BinaryField::<u128>::new();
    group.bench_function("binary_field_mul_u128", |b| {
        b.iter(|| {
            let a = 0x8f33d232df323232;
            let b = 0x33dfdf3232322323;
            let c = field.mul(&a, &b);
            black_box(c);
        })
    });
}

pub fn bench_binary_field_inv_u128(c: &mut Criterion) {
    let mut group = c.benchmark_group("binary_field_inv_u128");
    let field = linearalgebra::binary::BinaryField::<u128>::new();
    group.bench_function("binary_field_inv_u128", |b| {
        b.iter(|| {
            let a = 0x8f33d232df323232;
            let b = field.inv(&a).unwrap();
            black_box(b);
        })
    });
}

pub fn bench_binary_field_inv_gcd_u128(c: &mut Criterion) {
    let mut group = c.benchmark_group("binary_field_inv_gcd_u128");
    let field = linearalgebra::binary::BinaryField::<u128>::new();
    group.bench_function("binary_field_inv_gcd_u128", |b| {
        b.iter(|| {
            let a = 0x8f33d232df323232;
            let b = field.gcd_inv(&a).unwrap();
            black_box(b);
        })
    });
}

criterion_group!(
    benches,
    bench_binary_field_inv_gcd_u8,
    bench_binary_field_inv_u8,
    bench_binary_field_mul_u8,
    //add the same benchmarks for u16
    bench_binary_field_inv_gcd_u16,
    bench_binary_field_inv_u16,
    bench_binary_field_mul_u16,
    //add the same benchmarks for u32
    bench_binary_field_inv_gcd_u32,
    bench_binary_field_inv_u32,
    bench_binary_field_mul_u32,
    //add the same benchmarks for u64
    bench_binary_field_inv_gcd_u64,
    bench_binary_field_inv_u64,
    bench_binary_field_mul_u64,
    //add the same benchmarks for u128
    bench_binary_field_inv_gcd_u128,
    bench_binary_field_inv_u128,
    bench_binary_field_mul_u128
);
criterion_main!(benches);
