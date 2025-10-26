use criterion::{criterion_group, criterion_main, Criterion, black_box};
use std::sync::{Arc};
use std::thread;
use pithanos::traits::ProbabilisticSet;
use pithanos::bloom::BloomFilter;

const OPERATIONS: usize = 100_000;
const FILTER_SIZE: usize = 1_000_000;
const HASH_FN_COUNT: usize = 3;
const NUM_THREADS: usize = 4;

fn bench_insert(c: &mut Criterion) {
    c.bench_function("bloom_insert", |b| {
        let filter: BloomFilter = BloomFilter::new(FILTER_SIZE, HASH_FN_COUNT);

        b.iter(|| {
            for i in 0..OPERATIONS {
                let key = format!("key-{}", i);
                filter.insert(black_box(&key));
            }
        });
    });
}

fn bench_contains(c: &mut Criterion) {
    c.bench_function("bloom_contains", |b| {
        let filter = BloomFilter::new(FILTER_SIZE, HASH_FN_COUNT);
        for i in 0..OPERATIONS/2 {
            let key = format!("key-{}", i);
            filter.insert(&key);
        }

        b.iter(|| {
            for i in 0..OPERATIONS {
                let key = format!("key-{}", i);
                filter.contains(black_box(&key));
            }
        });
    });
}

fn bench_concurrent_insert(c: &mut Criterion) {
    c.bench_function("bloom_concurrent_insert", |b| {
        b.iter(|| {
            let filter = Arc::new(BloomFilter::new(FILTER_SIZE, HASH_FN_COUNT));
            let mut handles = Vec::with_capacity(NUM_THREADS);

            for t in 0..NUM_THREADS {
                let f = Arc::clone(&filter);
                handles.push(thread::spawn(move || {
                    for i in 0..(OPERATIONS/NUM_THREADS) {
                        let key = format!("t{}-key{}", t, i);
                        f.insert(&key);
                    }
                }));
            }

            for h in handles {
                h.join().unwrap();
            }
        });
    });
}

fn bench_concurrent_contains(c: &mut Criterion) {
    c.bench_function("bloom_concurrent_contains", |b| {
        b.iter(|| {
            let filter = Arc::new(BloomFilter::new(FILTER_SIZE, HASH_FN_COUNT));
            for i in 0..OPERATIONS/2 {
                let key = format!("t{}-key-{}", i%NUM_THREADS, i);
                filter.insert(&key);
            }

            let mut handles = Vec::with_capacity(NUM_THREADS);

            for t in 0..NUM_THREADS {
                let f = Arc::clone(&filter);
                handles.push(thread::spawn(move || {
                    for i in 0..(OPERATIONS/NUM_THREADS) {
                        let key = format!("t{}-key{}", t, i);
                        f.contains(&key);
                    }
                }));
            }

            for h in handles {
                h.join().unwrap();
            }
        });
    });
}

criterion_group!(benches, bench_concurrent_insert, bench_concurrent_contains, bench_insert, bench_contains);
criterion_main!(benches);