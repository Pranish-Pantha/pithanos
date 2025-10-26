use std::sync::Arc;

use pithanos::{cms::CountMinSketch, traits::FrequencySketch};

const OPERATIONS: usize = 10_000;
const SKETCH_WIDTH: usize = 1_000_000;
const SKETCH_DEPTH: usize = 4;
const NUM_THREADS: usize = 4;

#[test]
fn concurrent_increment_and_frequency() {
    let cms = Arc::new(CountMinSketch::new(SKETCH_WIDTH, SKETCH_DEPTH));

    let mut handles = Vec::with_capacity(NUM_THREADS);
    for t in 0..NUM_THREADS {
        let f = Arc::clone(&cms);
        handles.push(std::thread::spawn(move || {
            for i in 0..OPERATIONS {
                let local_key = format!("t{}-key{}", t, i);
                f.increment(&local_key, 1);
                let shared_key = format!("key{}", i);
                f.increment(&shared_key, 1);
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    handles = Vec::with_capacity(NUM_THREADS);

    for t in 0..NUM_THREADS {
        let f = Arc::clone(&cms);
        handles.push(std::thread::spawn(move || {
            for i in 0..OPERATIONS {
                let local_key = format!("t{}-key{}", t, i);
                let freq_local = f.frequency(&local_key);
                assert!(freq_local >= 1, "Frequency of {} should be at least 1, got {}", local_key, freq_local);
                let shared_key = format!("key{}", i);
                let freq_shared = f.frequency(&shared_key);
                assert!(freq_shared >= NUM_THREADS as u32, "Frequency of {} should be at least {}, got {}", shared_key, NUM_THREADS, freq_shared);
            }
        }));
    }
}