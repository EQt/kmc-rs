//! A small bridge to KMC's API using [cxx].
//! # Example
//! ```rust
//! let db = kmc_rs::KmcFile::open_ra("data/test1")?;
//! let kmer = kmc_rs::Kmer::from("TAAGA")?;
//! assert_eq!(db.kmer_length(), 5);     // we have an index over 5-mers
//! assert_eq!(db.count_kmer(&kmer), 4); // "TAAGA" (or reverse complement) occurs 4 times
//! # Ok::<(), String>(())
//! ```
mod cxxbridge;

/// A KMC data base; usually consisting of two files ending `.kmc_pre` and `.kmc_suf`.
/// You can open a [KmcFile] in two modes of which currently only the *random access mode**
/// is supported (see [KmcFile::open_ra]).
pub struct KmcFile {
    handle: cxx::UniquePtr<cxxbridge::ffi::KmcFile>,
}

/// Binary representation of a kmer to be queried by [KmcFile::count_kmer].
pub struct Kmer {
    handle: cxx::UniquePtr<cxxbridge::ffi::Kmer>,
}

impl KmcFile {
    /// Open for random access mode.
    /// The file name `fname` must not include the suffixes `.kmc_pre` or `.kmc_suf`.
    /// The file is automatically closed by [Drop]
    pub fn open_ra(fname: &str) -> Result<Self, String> {
        let mut handle = cxxbridge::ffi::new_ckmc_file();
        if handle.pin_mut().OpenForRA(fname) {
            Ok(Self { handle })
        } else {
            Err(format!("Could not open '{}' for random access", fname))
        }
    }

    /// The parameter `k` when this data base was constructed with.
    pub fn kmer_length(&self) -> u32 {
        self.handle.KmerLength()
    }

    /// Number of (canical) k-mers in the data base.
    pub fn num_kmers(&mut self) -> usize {
        self.handle.pin_mut().KmerCount()
    }

    /// How often is the canonical `kmer` recorded in the data base?
    pub fn count_kmer(&self, kmer: &Kmer) -> usize {
        self.handle.CheckKmer(&kmer.handle)
    }
}

impl Drop for KmcFile {
    fn drop(&mut self) {
        if !self.handle.pin_mut().Close() {
            panic!("error while closing");
        }
    }
}

impl Kmer {
    /// Construct a kmer by a `&str`.
    pub fn from(kmer: &str) -> Result<Self, String> {
        let mut handle = cxxbridge::ffi::new_kmerapi();
        if !handle.pin_mut().from_string(kmer) {
            return Err(format!("Internal Error in CKmerApi::from_string"));
        }
        Ok(Self { handle })
    }

    /// Useful to check what this kmer represents.
    pub fn to_string(&mut self) -> String {
        self.handle.pin_mut().to_string()
    }

    /// Construct a new kmer and reserve space for `k` symbols.
    pub fn with_k(k: u8) -> Self {
        Self {
            handle: cxxbridge::ffi::new_kmerapi_with_len(k as u32),
        }
    }

    /// Number of symbols `k` of this kmer.
    pub fn len(&self) -> u32 {
        self.handle.KmerLength()
    }

    /// Construct a kmer from bit encoded kmer `val` with `k` symbols.
    /// Note: `k` must be at most `32`!
    pub fn from_u64(k: u8, val: u64) -> Self {
        let mut kmer = Self::with_k(k);
        kmer.set_u64(val);
        kmer
    }

    /// Reset the kmer to a new bit encoded kmer of same length.
    /// Note: length `k` must be at most `32`!
    pub fn set_u64(&mut self, val: u64) {
        assert!(self.len() <= 32);
        self.handle.pin_mut().set_u64(val);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open() -> Result<(), String> {
        let mut io = KmcFile::open_ra("./data/test1")?;
        assert_eq!(io.kmer_length(), 5);
        assert_eq!(io.num_kmers(), 291);
        Ok(())
    }

    #[test]
    fn test_kmer() -> Result<(), String> {
        let mut kmer = Kmer::from("TAAGA")?;
        let s = kmer.to_string();
        assert_eq!(&s, "TAAGA", "got {}", &s);
        Ok(())
    }

    #[test]
    fn test_count_kmer() -> Result<(), String> {
        let mut kmer = Kmer::from("TAAGA")?;
        let io = KmcFile::open_ra("./data/test1")?;
        assert_eq!(io.count_kmer(&mut kmer), 4);
        Ok(())
    }

    #[test]
    fn test_from_u64_taaga() {
        let mut kmer = Kmer::from_u64(5, 0b11_00_00_10_00);
        assert_eq!(kmer.to_string(), "TAAGA");
    }

    #[test]
    fn test_from_u64_tcaaccttggaa() {
        assert_eq!("TCAACCTTGGAA".len(), 12);
        let mut kmer = Kmer::from_u64(12, 0b1101_0000_0101_1111_1010_0000);
        assert_eq!(kmer.to_string(), "TCAACCTTGGAA");
    }

    #[test]
    fn test_from_u64_ttttttttttttttttttttttttttttttc() {
        assert_eq!("TCAACCTTGGAA".len(), 12);
        let mut kmer = Kmer::from_u64(
            31,
            0b1111111111111111111111111111111111111111111111111111111111111_01,
        );
        assert_eq!(kmer.to_string(), "TTTTTTTTTTTTTTTTTTTTTTTTTTTTTTC");
    }
}
