[![Build Status](https://travis-ci.com/EQt/kmc-rs.svg?token=WXPT4d6dD68rQ9ty7yDf&branch=main)](https://travis-ci.com/EQt/kmc-rs)
# Rust Bindings to KMC API

Access a [K-Mer Count (KMC)][kmc.hub] data base file and query the counts.

## Example

```rust
let db = kmc_rs::KmcFile::open_ra("data/test1")?;
let kmer = kmc_rs::Kmer::from("TAAGA")?;
assert_eq!(db.kmer_length(), 5);     // we have an index over 5-mers
assert_eq!(db.count_kmer(&kmer), 4); // "TAAGA" (or reverse complement) occurs 4 times
```

[kmc.hub]: https://github.com/refresh-bio/KMC
