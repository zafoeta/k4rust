use criterion::{criterion_group, criterion_main, Criterion, black_box};
use k4rust::*;

#[link(name = "kdb_client", kind = "static")]
unsafe extern "C" {}

const N: i64 = 1_000_000;

// --- Hoisted Slice Approach ---

fn add_longs_hoisted(x: &K, y: &K) -> K {
    let res = ktn(KJ, x.n());
    let (xs, ys, rs) = (x.kJ(), y.kJ(), res.kJ());
    const NJ: i64 = i64::MIN;
    for i in 0..rs.len() {
        let xv = xs[i];
        let yv = ys[i];
        rs[i] = if xv == NJ || yv == NJ { NJ } else { xv.wrapping_add(yv) };
    }
    res
}

fn scale_floats_hoisted(x: &K, factor: f64) -> K {
    let res = ktn(KF, x.n());
    let (xs, rs) = (x.kF(), res.kF());
    for i in 0..rs.len() { rs[i] = xs[i] * factor; }
    res
}

fn dot_product_hoisted(x: &K, y: &K) -> f64 {
    let (xs, ys) = (x.kF(), y.kF());
    let mut total = 0.0;
    for i in 0..xs.len() { total += xs[i] * ys[i]; }
    total
}

fn filter_gt_hoisted(x: &K, limit: i32) -> K {
    let xs = x.kI();
    let mut count = 0i64;
    for i in 0..xs.len() { if xs[i] > limit { count += 1; } }
    let res = ktn(KI, count);
    let rs = res.kI();
    let mut idx = 0;
    for i in 0..xs.len() {
        if xs[i] > limit { rs[idx] = xs[i]; idx += 1; }
    }
    res
}

// --- Inline Approach ---

fn add_longs_inline(x: &K, y: &K) -> K {
    let res = ktn(KJ, x.n());
    const NJ: i64 = i64::MIN;
    for i in 0..res.n() as usize {
        let xv = x.kJ()[i];
        let yv = y.kJ()[i];
        res.kJ()[i] = if xv == NJ || yv == NJ { NJ } else { xv.wrapping_add(yv) };
    }
    res
}

fn scale_floats_inline(x: &K, factor: f64) -> K {
    let res = ktn(KF, x.n());
    for i in 0..res.n() as usize { res.kF()[i] = x.kF()[i] * factor; }
    res
}

fn dot_product_inline(x: &K, y: &K) -> f64 {
    let mut total = 0.0;
    for i in 0..x.n() as usize { total += x.kF()[i] * y.kF()[i]; }
    total
}

fn filter_gt_inline(x: &K, limit: i32) -> K {
    let mut count = 0i64;
    for i in 0..x.n() as usize { if x.kI()[i] > limit { count += 1; } }
    let res = ktn(KI, count);
    let mut idx = 0;
    for i in 0..x.n() as usize {
        if x.kI()[i] > limit { res.kI()[idx] = x.kI()[i]; idx += 1; }
    }
    res
}

// --- Benchmarks ---

fn bench_add_longs(c: &mut Criterion) {
    let x = ktn(KJ, N);
    let y = ktn(KJ, N);
    for i in 0..N as usize { x.kJ()[i] = i as i64; y.kJ()[i] = i as i64; }

    let mut g = c.benchmark_group("add_longs");
    g.bench_function("hoisted", |b| b.iter(|| black_box(add_longs_hoisted(&x, &y))));
    g.bench_function("inline",  |b| b.iter(|| black_box(add_longs_inline(&x, &y))));
    g.finish();
}

fn bench_scale_floats(c: &mut Criterion) {
    let x = ktn(KF, N);
    for i in 0..N as usize { x.kF()[i] = i as f64 * 0.5; }

    let mut g = c.benchmark_group("scale_floats");
    g.bench_function("hoisted", |b| b.iter(|| black_box(scale_floats_hoisted(&x, 2.5))));
    g.bench_function("inline",  |b| b.iter(|| black_box(scale_floats_inline(&x, 2.5))));
    g.finish();
}

fn bench_dot_product(c: &mut Criterion) {
    let x = ktn(KF, N);
    let y = ktn(KF, N);
    for i in 0..N as usize { x.kF()[i] = i as f64; y.kF()[i] = 1.0; }

    let mut g = c.benchmark_group("dot_product");
    g.bench_function("hoisted", |b| b.iter(|| black_box(dot_product_hoisted(&x, &y))));
    g.bench_function("inline",  |b| b.iter(|| black_box(dot_product_inline(&x, &y))));
    g.finish();
}

fn bench_filter_gt(c: &mut Criterion) {
    let x = ktn(KI, N);
    for i in 0..N as usize { x.kI()[i] = (i % 1000) as i32; }

    let mut g = c.benchmark_group("filter_gt");
    g.bench_function("hoisted", |b| b.iter(|| black_box(filter_gt_hoisted(&x, 500))));
    g.bench_function("inline",  |b| b.iter(|| black_box(filter_gt_inline(&x, 500))));
    g.finish();
}

criterion_group!(benches, bench_add_longs, bench_scale_floats, bench_dot_product, bench_filter_gt);
criterion_main!(benches);
