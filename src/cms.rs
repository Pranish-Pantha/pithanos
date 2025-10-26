use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::hash::Hash;
use crate::traits::FrequencySketch;
use crate::utils::multi_hash;

pub struct CountMinSketch {
    table: Vec<Vec<AtomicUsize>>,
    width: usize,
    depth: usize,
}

impl CountMinSketch {
    pub fn new(width: usize, depth: usize) -> Self {
        let mut table = Vec::with_capacity(depth);
        for _ in 0..depth {
            let mut row = Vec::with_capacity(width);
            row.resize_with(width, || AtomicUsize::new(0));
            table.push(row);
        }
        Self { table, width, depth }
    }

    fn hash_positions<T: Hash>(&self, item: &T) -> Vec<usize> {
        let mut positions = vec![0; self.depth];
        multi_hash(item, &mut positions);
        for pos in &mut positions {
            *pos %= self.width;
        }
        positions
    }

    
}

impl FrequencySketch for CountMinSketch {
    fn increment<T: Hash>(&self, item: &T, count: u32) {
        for (i, pos) in self.hash_positions(item).iter().enumerate() {
            self.table[i][*pos].fetch_add(count as usize, Ordering::Relaxed);
        }
    }

    fn frequency<T: Hash>(&self, item: &T) -> u32 {
        let mut min_count = u32::MAX;
        for (i, pos) in self.hash_positions(item).iter().enumerate() {
            let count: u32 = self.table[i][*pos].load(Ordering::Relaxed) as u32;
            if count < min_count {
                min_count = count;
            }
        }
        min_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SKETCH_WIDTH: usize = 1024;
    const SKETCH_DEPTH: usize = 4;

    #[test]
    fn construct() {
        let cms = CountMinSketch::new(SKETCH_WIDTH, SKETCH_DEPTH);

        assert_eq!(cms.table.len(), SKETCH_DEPTH);
        for row in &cms.table {
            assert_eq!(row.len(), SKETCH_WIDTH);
            assert!(row.iter().all(|x| x.load(Ordering::Relaxed) == 0));
        }
        assert_eq!(cms.width, SKETCH_WIDTH);
        assert_eq!(cms.depth, SKETCH_DEPTH);
    }

    #[test]
    fn hash_positions_consistent() {
        let cms = CountMinSketch::new(SKETCH_WIDTH, SKETCH_DEPTH);

        let item = "foo";
        let positions1 = cms.hash_positions(&item);
        let positions2 = cms.hash_positions(&item);

        assert_eq!(positions1, positions2);
        assert_eq!(positions1.len(), SKETCH_DEPTH);
        for pos in positions1 {
            assert!(pos < SKETCH_WIDTH);
        }
    }

    #[test]
    fn increment() {
        let cms = CountMinSketch::new(SKETCH_WIDTH, SKETCH_DEPTH);

        let item = "foo";
        cms.increment(&item, 3);
        
        for (i, pos) in cms.hash_positions(&item).iter().enumerate() {
            let count = cms.table[i][*pos].load(Ordering::Relaxed);
            assert_eq!(count, 3);
        }
    }

    #[test]
    fn frequency() {
        let cms = CountMinSketch::new(SKETCH_WIDTH, SKETCH_DEPTH);

        let item = "foo";
        cms.increment(&item, 3);
        assert_eq!(cms.frequency(&item), 3);

        let not_inserted_item = "bar";
        assert_eq!(cms.frequency(&not_inserted_item), 0);
    }
}