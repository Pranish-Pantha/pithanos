use std::sync::Arc;

use pithanos::{bloom::BloomFilter, traits::ProbabilisticSet};

const OPERATIONS: usize = 10_000;
const FILTER_SIZE: usize = 1_000_000;
const HASH_FN_COUNT: usize = 3;
const NUM_THREADS: usize = 4;

#[test]
fn concurrent_insert_and_contains() {
    let filter = Arc::new(BloomFilter::new(FILTER_SIZE, HASH_FN_COUNT));

    let mut handles = Vec::with_capacity(NUM_THREADS);
    for t in 0..NUM_THREADS {
        let f = Arc::clone(&filter);
        handles.push(std::thread::spawn(move || {
            for i in 0..OPERATIONS {
                let local_key = format!("t{}-key{}", t, i);
                f.insert(&local_key);
                let shared_key = format!("key{}", i);
                f.insert(&shared_key);
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    handles = Vec::with_capacity(NUM_THREADS);

    for t in 0..NUM_THREADS {
        let f = Arc::clone(&filter);
        handles.push(std::thread::spawn(move || {
            for i in 0..OPERATIONS {
                let local_key = format!("t{}-key{}", t, i);
                assert!(f.contains(&local_key), "Filter should contain {}", local_key);
                let shared_key = format!("key{}", i);
                assert!(f.contains(&shared_key), "Filter should contain {}", shared_key);
            }
        }));
    }
}