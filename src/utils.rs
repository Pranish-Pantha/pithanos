use xxhash_rust::xxh3::Xxh3;
use std::hash::{Hash, Hasher};

/// Computes multiple hash values for a given item using double hashing technique.
pub fn multi_hash<T: Hash>(item: &T, output: &mut [usize]) {
    let mut h1 = Xxh3::new();
    item.hash(&mut h1);
    let h1 = h1.finish();

    let mut h2 = Xxh3::with_seed(0x7FFFFFFF);
    item.hash(&mut h2);
    let h2 = h2.finish();

    for i in 0..output.len() {
        output[i] = (h1.wrapping_add(i as u64).wrapping_mul(h2)) as usize;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn multi_hash_correct() {
        let item = "foo";
        let mut output = vec![0; 2];
        multi_hash(&item, &mut output);

        let first_hash = {
            let mut hasher = Xxh3::new();
            item.hash(&mut hasher);
            hasher.finish() as usize
        };
        let second_hash = {
            let mut hasher = Xxh3::with_seed(0x7FFFFFFF);
            item.hash(&mut hasher);
            first_hash.wrapping_add(hasher.finish() as usize)
        };

        assert_eq!(output[0], first_hash);
        assert_eq!(output[1], second_hash);
    }
}