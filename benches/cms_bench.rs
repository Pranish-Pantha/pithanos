use criterion::{criterion_group, criterion_main, Criterion, black_box};
use std::sync::{Arc};
use std::thread;
use pithanos::traits::FrequencySketch;
use pithanos::cms::CountMinSketch;

const OPERATIONS: usize = 100_000;
const WIDTH: usize = 200_000;
const DEPTH: usize = 5;
const NUM_THREADS: usize = 4;

fn bench_increment(c: &mut Criterion) {
    c.bench_function("cms_increment", |b| {
        let cms = CountMinSketch::new(WIDTH, DEPTH);

        b.iter(|| {
            for i in 0..OPERATIONS {
                let key = format!("key-{}", i);
                cms.increment(black_box(&key), 1);
            }
        });
    });
}

fn bench_frequency(c: &mut Criterion) {
    c.bench_function("cms_frequency", |b| {
        let cms = CountMinSketch::new(WIDTH, DEPTH);
        for i in 0..OPERATIONS/2 {
            let key = format!("key-{}", i);
            cms.increment(black_box(&key), 1);
        }

        b.iter(|| {
            for i in 0..OPERATIONS {
                let key = format!("key-{}", i);
                cms.frequency(black_box(&key));
            }
        });
    });
}

fn bench_concurrent_increment(c: &mut Criterion) {
    c.bench_function("cms_concurrent_increment", |b| {
        let cms = Arc::new(CountMinSketch::new(WIDTH, DEPTH));

        b.iter(|| {    
            let mut handles = Vec::with_capacity(NUM_THREADS);

            for t in 0..NUM_THREADS {
                let c = Arc::clone(&cms);
                handles.push(thread::spawn(move || {
                    for i in 0..(OPERATIONS/NUM_THREADS) {
                        let key = format!("t{}-key{}", t, i);
                        c.increment(&key, 1);
                    }
                }));
            }

            for h in handles {
                h.join().unwrap();
            }
        });
    });
}

fn bench_concurrent_frequency(c: &mut Criterion) {
    c.bench_function("cms_concurrent_frequency", |b| {
        let cms = Arc::new(CountMinSketch::new(WIDTH, DEPTH));
            for i in 0..OPERATIONS/2 {
                let key = format!("key-{}", i);
                cms.increment(&key, 1);
            }

        b.iter(|| {
            let mut handles = Vec::with_capacity(NUM_THREADS);

            for t in 0..NUM_THREADS {
                let c = Arc::clone(&cms);
                handles.push(thread::spawn(move || {
                    for i in 0..(OPERATIONS/NUM_THREADS) {
                        let key = format!("t{}-key{}", t, i);
                        c.frequency(&key);
                    }
                }));
            }

            for h in handles {
                h.join().unwrap();
            }
        });
    });
}

criterion_group!(benches, bench_concurrent_increment, bench_concurrent_frequency, bench_increment, bench_frequency);
criterion_main!(benches);