//! Criterion benchmarks for pricer_kernel Monte Carlo pricing.
//!
//! Benchmarks cover:
//! - Monte Carlo path generation (1K, 10K, 100K paths)
//! - European option pricing with varying path counts
//! - Greeks computation (Delta via bump-and-revalue / forward AD)
//! - RNG performance

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use pricer_kernel::mc::{GbmParams, MonteCarloConfig, MonteCarloPricer, PayoffParams};
use pricer_kernel::rng::PricerRng;

/// Benchmark RNG generation (foundation for MC simulations).
fn bench_rng_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("rng_generation");

    for n_samples in [1_000, 10_000, 100_000] {
        group.bench_with_input(
            BenchmarkId::new("normal_samples", n_samples),
            &n_samples,
            |b, &n| {
                let mut rng = PricerRng::from_seed(42);
                b.iter(|| {
                    let mut sum = 0.0;
                    for _ in 0..n {
                        sum += rng.gen_normal();
                    }
                    black_box(sum)
                });
            },
        );
    }

    // Batch generation (more efficient)
    for n_samples in [1_000, 10_000, 100_000] {
        group.bench_with_input(
            BenchmarkId::new("normal_batch", n_samples),
            &n_samples,
            |b, &n| {
                let mut rng = PricerRng::from_seed(42);
                let mut buffer = vec![0.0; n];
                b.iter(|| {
                    rng.fill_normal(&mut buffer);
                    black_box(buffer.iter().sum::<f64>())
                });
            },
        );
    }

    group.finish();
}

/// Benchmark Monte Carlo pricing with varying path counts.
fn bench_mc_pricing(c: &mut Criterion) {
    let mut group = c.benchmark_group("mc_pricing");
    group.sample_size(50); // Reduce sample size for slower benchmarks

    let gbm = GbmParams::default();
    let payoff = PayoffParams::call(100.0);
    let discount_factor = 0.95;
    let n_steps = 50; // Fixed time steps for pricing benchmark

    // Benchmark different path counts
    for n_paths in [1_000, 10_000, 100_000] {
        group.bench_with_input(
            BenchmarkId::new("european_call", n_paths),
            &n_paths,
            |b, &n| {
                let config = MonteCarloConfig::builder()
                    .n_paths(n)
                    .n_steps(n_steps)
                    .seed(42)
                    .build()
                    .unwrap();
                let mut pricer = MonteCarloPricer::new(config).unwrap();
                b.iter(|| {
                    pricer.price_european(
                        black_box(gbm),
                        black_box(payoff),
                        black_box(discount_factor),
                    )
                });
            },
        );
    }

    // Benchmark put option
    let put_payoff = PayoffParams::put(100.0);
    group.bench_with_input(
        BenchmarkId::new("european_put", 10_000),
        &10_000,
        |b, &n| {
            let config = MonteCarloConfig::builder()
                .n_paths(n)
                .n_steps(n_steps)
                .seed(42)
                .build()
                .unwrap();
            let mut pricer = MonteCarloPricer::new(config).unwrap();
            b.iter(|| {
                pricer.price_european(
                    black_box(gbm),
                    black_box(put_payoff),
                    black_box(discount_factor),
                )
            });
        },
    );

    group.finish();
}

/// Benchmark pricing with varying time steps.
fn bench_mc_steps_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("mc_steps_scaling");
    group.sample_size(50);

    let gbm = GbmParams::default();
    let payoff = PayoffParams::call(100.0);
    let discount_factor = 0.95;
    let n_paths = 10_000; // Fixed path count

    // Benchmark different step counts
    for n_steps in [10, 50, 252, 500] {
        group.bench_with_input(
            BenchmarkId::new("european_call", n_steps),
            &n_steps,
            |b, &steps| {
                let config = MonteCarloConfig::builder()
                    .n_paths(n_paths)
                    .n_steps(steps)
                    .seed(42)
                    .build()
                    .unwrap();
                let mut pricer = MonteCarloPricer::new(config).unwrap();
                b.iter(|| {
                    pricer.price_european(
                        black_box(gbm),
                        black_box(payoff),
                        black_box(discount_factor),
                    )
                });
            },
        );
    }

    group.finish();
}

/// Benchmark Greeks computation.
fn bench_greeks(c: &mut Criterion) {
    let mut group = c.benchmark_group("greeks");
    group.sample_size(30); // Greeks are slower to compute

    let gbm = GbmParams::default();
    let payoff = PayoffParams::call(100.0);
    let discount_factor = 0.95;
    let n_steps = 50;

    // Benchmark Delta computation (bump-and-revalue or forward AD)
    for n_paths in [1_000, 10_000] {
        group.bench_with_input(BenchmarkId::new("delta", n_paths), &n_paths, |b, &n| {
            let config = MonteCarloConfig::builder()
                .n_paths(n)
                .n_steps(n_steps)
                .seed(42)
                .build()
                .unwrap();
            let mut pricer = MonteCarloPricer::new(config).unwrap();
            b.iter(|| {
                pricer.price_with_delta_ad(
                    black_box(gbm),
                    black_box(payoff),
                    black_box(discount_factor),
                )
            });
        });
    }

    group.finish();
}

/// Benchmark workspace allocation overhead.
fn bench_workspace_allocation(c: &mut Criterion) {
    let mut group = c.benchmark_group("workspace_allocation");

    // Benchmark pricer creation (includes workspace allocation)
    for n_paths in [1_000, 10_000, 100_000] {
        group.bench_with_input(
            BenchmarkId::new("pricer_creation", n_paths),
            &n_paths,
            |b, &n| {
                let config = MonteCarloConfig::builder()
                    .n_paths(n)
                    .n_steps(50)
                    .seed(42)
                    .build()
                    .unwrap();
                b.iter(|| black_box(MonteCarloPricer::new(config.clone()).unwrap()));
            },
        );
    }

    // Benchmark workspace reuse (multiple pricing calls with same pricer)
    group.bench_function("reuse_vs_recreate", |b| {
        let config = MonteCarloConfig::builder()
            .n_paths(10_000)
            .n_steps(50)
            .seed(42)
            .build()
            .unwrap();
        let mut pricer = MonteCarloPricer::new(config).unwrap();
        let gbm = GbmParams::default();
        let payoff = PayoffParams::call(100.0);
        let discount_factor = 0.95;

        b.iter(|| {
            // Multiple pricing calls reusing workspace
            for _ in 0..10 {
                black_box(pricer.price_european(gbm, payoff, discount_factor));
            }
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_rng_generation,
    bench_mc_pricing,
    bench_mc_steps_scaling,
    bench_greeks,
    bench_workspace_allocation
);
criterion_main!(benches);
