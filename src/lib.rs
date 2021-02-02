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
/// You can open a [KmcFile] in two modes:
///  * **random access mode** (see [KmcFile::open_ra]), and
///  * **iterator mode** (see [KmcFile::open_iter]).
pub struct KmcFile {
    ptr: cxx::UniquePtr<cxxbridge::ffi::KmcFile>,
}

/// Binary representation of a kmer to be queried by [KmcFile::count_kmer].
pub struct Kmer {
    handle: cxx::UniquePtr<cxxbridge::ffi::Kmer>,
}

#[doc(hidden)]
pub struct KmcFileIter<'a> {
    file: &'a mut cxx::UniquePtr<cxxbridge::ffi::KmcFile>,
    kmer: Kmer,
}

impl KmcFile {
    /// Open in random access mode.
    /// The file name `fname` must not include the suffixes `.kmc_pre` or `.kmc_suf`.
    /// The file is automatically closed by [Drop].
    pub fn open_ra(fname: &str) -> Result<Self, String> {
        let mut ptr = cxxbridge::ffi::new_ckmc_file();
        if ptr.pin_mut().open_for_ra(fname) {
            Ok(Self { ptr })
        } else {
            Err(format!("Could not open '{}' for random access", fname))
        }
    }

    /// Open in iterator mode (also called „listing“ mode).
    /// The file name `fname` must not include the suffixes `.kmc_pre` or `.kmc_suf`.
    /// The file is automatically closed by [Drop].
    pub fn open_iter(fname: &str) -> Result<Self, String> {
        let mut ptr = cxxbridge::ffi::new_ckmc_file();
        if ptr.pin_mut().open_for_iter(fname) {
            Ok(Self { ptr })
        } else {
            Err(format!("Could not open '{}' in listing mode", fname))
        }
    }

    /// The parameter `k` when this data base was constructed with.
    pub fn kmer_length(&self) -> u32 {
        self.ptr.kmer_len()
    }

    pub fn iter<'a>(&'a mut self) -> KmcFileIter<'a> {
        use std::convert::TryInto;

        let k = self.kmer_length().try_into().unwrap();
        KmcFileIter {
            file: &mut self.ptr,
            kmer: Kmer::with_k(k),
        }
    }

    /// Number of (canical) k-mers in the data base.
    ///
    /// It might be necessary to iterate through the whole file; that is why a `&mut self`
    /// is needed, here.
    pub fn num_kmers(&mut self) -> usize {
        self.ptr.pin_mut().kmer_count()
    }

    /// How often is the canonical `kmer` recorded in the data base?
    /// Only works when opened as [KmcFile::open_ra].
    pub fn count_kmer(&self, kmer: &Kmer) -> usize {
        self.ptr.check_kmer(&kmer.handle)
    }

    /// Reset the file pointer to the beginning.
    /// Only useful when opened as [KmcFile::open_iter].
    pub fn restart(&mut self) -> bool {
        self.ptr.pin_mut().restart_listing()
    }
}

impl Drop for KmcFile {
    fn drop(&mut self) {
        if !self.ptr.pin_mut().close() {
            panic!("error while closing");
        }
    }
}

impl<'a> Iterator for KmcFileIter<'a> {
    type Item = (u64, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let mut count = 0;
        let ptr: &mut cxx::UniquePtr<cxxbridge::ffi::KmcFile> = &mut self.file;
        let r = ptr.pin_mut().next(self.kmer.handle.pin_mut(), &mut count);
        if r {
            Some((self.kmer.as_u64(), count))
        } else {
            None
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

    /// Construct a new kmer and reserve space for `k` symbols.
    pub fn with_k(k: u8) -> Self {
        Self {
            handle: cxxbridge::ffi::new_kmerapi_with_len(k as u32),
        }
    }

    /// Number of symbols `k` of this kmer.
    pub fn len(&self) -> u32 {
        self.handle.kmer_len()
    }

    /// Construct a kmer from bit encoded kmer `val` with `k` symbols.
    /// Note: `k` must be at most `32`!
    /// See [Kmer::set_u64] for further details.
    pub fn from_u64(k: u8, val: u64) -> Self {
        let mut kmer = Self::with_k(k);
        kmer.set_u64(val);
        kmer
    }

    /// Reset the kmer to a new bit encoded kmer of same length.
    /// Note: length `k` must be at most `32`!
    ///
    /// The coding is as usual:
    ///  - `A` -> `0b00`
    ///  - `C` -> `0b01`
    ///  - `T` -> `0b10`
    ///  - `G` -> `0b11`
    ///
    /// # Example
    /// ```rust
    /// let mut kmer = kmc_rs::Kmer::with_k(5);
    /// kmer.set_u64(0b11_00_00_10_00);
    /// assert_eq!(kmer.to_string(), "TAAGA");
    /// ```
    #[inline]
    pub fn set_u64(&mut self, val: u64) {
        debug_assert!(self.len() <= 32);
        self.handle.pin_mut().set_u64(val);
    }

    /// Obtain the first 64 bits of this Kmer.
    /// When `self.len() > 32` the bits are incomplete.
    /// ```rust
    /// let kmer = kmc_rs::Kmer::from("TAAGA")?;
    /// assert_eq!(kmer.as_u64(), 0b11_00_00_10_00);
    /// Ok::<(), String>(())
    /// ```
    #[inline]
    pub fn as_u64(&self) -> u64 {
        self.handle.as_u64()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl std::fmt::Display for cxxbridge::ffi::Kmer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(&self.to_string())
    }
}

impl std::fmt::Display for Kmer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.handle.fmt(f)
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
        let kmer = Kmer::from("TAAGA")?;
        let s = kmer.to_string();
        assert_eq!(&s, "TAAGA", "got {}", &s);
        Ok(())
    }

    #[test]
    fn test_kmer_errors() {
        assert!(Kmer::from("TCN").is_err());
        assert!(Kmer::from("actG").is_ok());
    }

    #[test]
    fn test_count_kmer() -> Result<(), String> {
        let mut kmer = Kmer::from("TAAGA")?;
        let io = KmcFile::open_ra("./data/test1")?;
        assert_eq!(io.count_kmer(&mut kmer), 4);
        Ok(())
    }

    #[test]
    fn test_from_u64_tcaaccttggaa() {
        assert_eq!("TCAACCTTGGAA".len(), 12);
        let kmer = Kmer::from_u64(12, 0b1101_0000_0101_1111_1010_0000);
        assert_eq!(kmer.to_string(), "TCAACCTTGGAA");
    }

    #[test]
    fn test_from_u64_ttttttttttttttttttttttttttttttc() {
        assert_eq!("TTTTTTTTTTTTTTTTTTTTTTTTTTTTTTC".len(), 31);
        assert_eq!(
            "TTTTTTTTTTTTTTTTTTTTTTTTTTTTTTC",
            Kmer::from_u64(
                31,
                0b1111111111111111111111111111111111111111111111111111111111111_01,
            )
            .to_string()
        );
    }

    #[test]
    fn test_open_iter() -> Result<(), String> {
        let io = KmcFile::open_iter("./data/test1")?;
        assert_eq!(io.kmer_length(), 5);
        Ok(())
    }

    #[test]
    fn test_iter_count() -> Result<(), String> {
        assert_eq!(KmcFile::open_iter("./data/test1")?.iter().count(), 291);
        Ok(())
    }

    #[test]
    fn test_iter_count_taaga() -> Result<(), String> {
        assert_eq!(
            KmcFile::open_iter("./data/test1")?
                .iter()
                .filter(|&(b, _)| { b == 0b11_00_00_10_00 })
                .map(|(_, c)| c)
                .next()
                .ok_or("should not happen")?,
            4
        );
        Ok(())
    }
}
