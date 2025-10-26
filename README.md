# 🎯 Pithanos

[![Crates.io](https://img.shields.io/crates/v/pithanos.svg)](https://crates.io/crates/pithanos)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

> ⚡ Fast, lock-free probabilistic data structures for modern Rust. Includes **Bloom Filter** and **Count-Min Sketch** implementations optimized for speed and concurrency.

---
## ✨ Features

| Feature | Description |
|----------|--------------|
| 🧩 **Bloom Filter** | Approximate set membership check |
| 📊 **Count-Min Sketch** | Approximate frequency estimation |
| ⚙️ **No global locks** | Thread-safe access with atomic operations, fast deterministic hashing (`xxhash`) |
| 📦 **Modular design** | Shared internal traits and utilities, clean module structure |
| 🧪 **Benchmark suite** | Criterion-based microbenchmarks for queries |

---

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
pithanos = "0.1"
```

## 🚀 Quick Start
### Bloom Filter
```rust
use pithanos::bloom::BloomFilter;

fn main() {
    let filter = BloomFilter::new(1000, 3); // 1000 bits, 3 hash functions

    filter.insert(&"foo");
    filter.insert(&"bar");

    assert!(filter.contains(&"foo"));
    assert!(!filter.contains(&"bar"));
}
```

### Count-Min Sketch
```rust
use pithanos::cms::CountMinSketch;

fn main() {
    let cms = CountMinSketch::new(1000, 3); // wdith 100, depth 3

    cms.increment(&"foo", 3);
    cms.increment(&"bar", 1);

    println!("Frequency of 'foo': {}", cms.frequency(&"foo"));
    println!("Frequency of 'bar': {}", cms.frequency(&"bar"));
}
```

## 🧪 Benchmarks

Pithanos uses [criterion](https://docs.rs/criterion/latest/criterion/) for micro-benchmarks.

Run all benchmarks with ``cargo bench``

## 💬 Acknowledgments
> Pithanos (πιθανός) — Ancient Greek: “likely,” “probable,” or “plausible.”

Inspired by  [RedisBloom](https://github.com/RedisBloom/RedisBloom)