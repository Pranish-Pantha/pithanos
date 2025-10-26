use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::hash::Hash;
use crate::utils::multi_hash;
use crate::traits::ProbabilisticSet;

pub struct BloomFilter {
    bits: Vec<AtomicUsize>,
    size: usize,
    hash_fn_count: usize,
}

impl BloomFilter {
    pub fn new(size: usize, hash_fn_count: usize) -> Self {
        let words = (size + usize::BITS as usize - 1) / usize::BITS as usize;
        let mut bits = Vec::with_capacity(words);
        bits.resize_with(words, || AtomicUsize::new(0));
        Self { bits, size, hash_fn_count }
    }

    fn hash_positions<T: Hash>(&self, item: &T) -> Vec<usize> {
        let mut positions = vec![0; self.hash_fn_count];
        multi_hash(item, &mut positions);
        for pos in &mut positions {
            *pos %= self.size;
        }
        positions
    }
}

impl ProbabilisticSet for BloomFilter {
    fn insert<T: Hash>(&self, item: &T) {
        for pos in self.hash_positions(item) {
            let word_index = pos / usize::BITS as usize;
            let bit_offset = pos % usize::BITS as usize;
            let mask = 1usize << bit_offset;
            self.bits[word_index].fetch_or(mask, Ordering::Relaxed);
        }
    }

    fn contains<T: Hash>(&self, item: &T) -> bool {
        self.hash_positions(item).iter().all(|&pos| {
            let word_index = pos / usize::BITS as usize;
            let bit_offset = pos % usize::BITS as usize;
            let mask = 1usize << bit_offset;
            (self.bits[word_index].load(Ordering::Relaxed) & mask) != 0
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const FILTER_SIZE: usize = 1024;
    const HASH_FN_COUNT: usize = 3;

    #[test]
    fn construct() {
        let bf = BloomFilter::new(FILTER_SIZE, HASH_FN_COUNT);

        assert_eq!(bf.bits.len(), (bf.size + usize::BITS as usize - 1) / usize::BITS as usize);
        assert!(bf.bits.iter().all(|x| x.load(Ordering::Relaxed) == 0));
        assert_eq!(bf.size, FILTER_SIZE);
        assert_eq!(bf.hash_fn_count, 3);
    }

    #[test]
    fn hash_positions_consistent() {
        let bf = BloomFilter::new(FILTER_SIZE, HASH_FN_COUNT);

        let item = "foo";
        let pos = bf.hash_positions(&item);

        assert_eq!(pos, bf.hash_positions(&item));
        assert_eq!(pos.len(), HASH_FN_COUNT);
        for p in pos {
            assert!(p < FILTER_SIZE);
        }
    }

    #[test]
    fn insert() {
        let bf = BloomFilter::new(FILTER_SIZE, HASH_FN_COUNT);

        let item = "foo";
        bf.insert(&item);

        for pos in bf.hash_positions(&item) {
            let word_index = pos / usize::BITS as usize;
            let bit_offset = pos % usize::BITS as usize;
            let mask = 1usize << bit_offset;
            assert_ne!(bf.bits[word_index].load(Ordering::Relaxed) & mask, 0);
        }
    }

    #[test]
    fn contains() {
        let bf = BloomFilter::new(FILTER_SIZE, HASH_FN_COUNT);

        let item = "foo";
        bf.insert(&item);
        assert!(bf.contains(&item));

        let not_inserted_item = "bar";
        assert!(!bf.contains(&not_inserted_item));
    }
}