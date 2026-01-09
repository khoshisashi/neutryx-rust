//! Benchmarks for pricer_optimiser.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pricer_optimiser::solvers::{Bfgs, LevenbergMarquardt};

fn benchmark_levenberg_marquardt(c: &mut Criterion) {
    let lm = LevenbergMarquardt::new();

    c.bench_function("lm_rosenbrock", |b| {
        b.iter(|| {
            let residuals = |p: &[f64]| vec![1.0 - p[0], 10.0 * (p[1] - p[0] * p[0])];
            let _ = lm.solve(black_box(&[-1.0, 1.0]), residuals);
        })
    });
}

fn benchmark_bfgs(c: &mut Criterion) {
    let bfgs = Bfgs::new();

    c.bench_function("bfgs_quadratic", |b| {
        b.iter(|| {
            let objective = |p: &[f64]| (p[0] - 2.0).powi(2) + (p[1] - 3.0).powi(2);
            let _ = bfgs.minimise(black_box(&[0.0, 0.0]), objective);
        })
    });
}

criterion_group!(benches, benchmark_levenberg_marquardt, benchmark_bfgs);
criterion_main!(benches);
