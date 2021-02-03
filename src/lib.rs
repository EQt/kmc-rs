//! A small bridge to KMC's API using [cxx].
//! # Example
//! ```rust
//! let db = kmc_rs::KmcFile::open_ra("data/test1")?;
//! let kmer = kmc_rs::Kmer::from("TAAGA")?;
//! assert_eq!(db.kmer_length(), 5);     // we have an index over 5-mers
//! assert_eq!(db.count_kmer(&kmer), 4); // "TAAGA" (or reverse complement) occurs 4 times
//! # Ok::<(), String>(())
//! ```

use cxxbridge::ffi;
mod cxxbridge;

/// A KMC data base; usually consisting of two files ending `.kmc_pre` and `.kmc_suf`.
/// You can open a [KmcFile] in two modes:
///  * **random access mode** (see [KmcFile::open_ra]), and
///  * **iterator mode** (see [KmcFile::open_iter]).
pub struct KmcFile {
    ptr: cxx::UniquePtr<ffi::KmcFile>,
}

/// Binary representation of a kmer to be queried by [KmcFile::count_kmer].
pub struct Kmer {
    ptr: cxx::UniquePtr<ffi::Kmer>,
}

impl KmcFile {
    /// Open in random access mode.
    /// The file name `fname` must not include the suffixes `.kmc_pre` or `.kmc_suf`.
    /// The file is automatically closed by [Drop].
    pub fn open_ra(fname: &str) -> Result<Self, String> {
        let mut ptr = ffi::new_ckmc_file();
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
        let mut ptr = ffi::new_ckmc_file();
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

    /// Start a new iterator yielding 64-bit encoded kmer items
    /// `(kmer, count): (u64, usize)`.
    ///
    /// For example, count all kmers starting with `"TG"`
    /// ```
    /// let mut db = kmc_rs::KmcFile::open_iter("data/test1")?;
    /// assert_eq!(db.kmer_length(), 5);
    /// let mut kmer = kmc_rs::Kmer::with_k(5);
    /// let mut count_tg = 0;
    /// while let Some(count) = db.read_next(&mut kmer) {
    ///     if kmer.as_u64() >> 3 == 0b11_10 {
    ///         count_tg += count;
    ///     }
    /// }
    /// assert_eq!(count_tg, 18);
    /// # Ok::<(), String>(())
    /// ```
    ///
    /// Only works when opened as [KmcFile::open_iter].
    pub fn iter_u64<'a>(&'a mut self) -> impl Iterator<Item = (u64, usize)> + 'a {
        use std::convert::TryInto;
        use std::pin::Pin;
        use std::ptr::NonNull;

        struct KmcFileIterU64<'a> {
            kmer: Kmer,
            cxx_kmer: NonNull<ffi::Kmer>,
            cxx_file: Pin<&'a mut ffi::KmcFile>,
        }

        impl<'a> KmcFileIterU64<'a> {
            fn new(file: &'a mut KmcFile, k: u8) -> KmcFileIterU64<'a> {
                let mut it = KmcFileIterU64 {
                    kmer: Kmer::with_k(k),
                    cxx_kmer: NonNull::dangling(),
                    cxx_file: file.ptr.pin_mut(),
                };
                let kref = unsafe { it.kmer.ptr.as_mut().unwrap().get_unchecked_mut() };
                it.cxx_kmer = NonNull::from(kref);
                it
            }
        }

        impl<'a> Iterator for KmcFileIterU64<'a> {
            type Item = (u64, usize);

            #[inline]
            fn next(&mut self) -> Option<Self::Item> {
                let mut count = 0;
                let kmer = unsafe { Pin::new_unchecked(self.cxx_kmer.as_mut()) };
                if self.cxx_file.as_mut().next(kmer, &mut count) {
                    Some((unsafe { self.cxx_kmer.as_ref() }.as_u64(), count))
                } else {
                    None
                }
            }
        }

        let k = self.kmer_length().try_into().unwrap();
        KmcFileIterU64::new(self, k)
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
        self.ptr.check_kmer(&kmer.ptr)
    }

    /// Reset the file pointer to the beginning.
    /// Only useful when opened as [KmcFile::open_iter].
    pub fn restart(&mut self) -> bool {
        self.ptr.pin_mut().restart_listing()
    }

    /// Read next entry into `kmer`.
    ///
    /// If there was one available return `Some(count)`; otherwise
    /// return None to indicate the end of the file
    /// ([KmcFile::restart] might be useful then).
    ///
    /// Only works when opened as [KmcFile::open_iter].
    #[inline]
    pub fn read_next(&mut self, kmer: &mut Kmer) -> Option<usize> {
        if kmer.len() == self.kmer_length() {
            unsafe { self.read_next_unchecked(kmer) }
        } else {
            None
        }
    }

    /// Like [KmcFile::read_next] but do not check the lengths.
    ///
    /// # Safety
    /// Might crash when `self.kmer_length() != kmer.len()`.
    #[inline]
    pub unsafe fn read_next_unchecked(&mut self, kmer: &mut Kmer) -> Option<usize> {
        let mut count = 0;
        if self.ptr.pin_mut().next(kmer.ptr.pin_mut(), &mut count) {
            Some(count)
        } else {
            None
        }
    }
}

impl Drop for KmcFile {
    fn drop(&mut self) {
        if !self.ptr.pin_mut().close() {
            panic!("error while closing");
        }
    }
}

impl Kmer {
    /// Construct a kmer by a `&str`.
    pub fn from(kmer: &str) -> Result<Self, String> {
        let mut handle = ffi::new_kmerapi();
        if !handle.pin_mut().from_string(kmer) {
            return Err(format!("Internal Error in CKmerApi::from_string"));
        }
        Ok(Self { ptr: handle })
    }

    /// Construct a new kmer and reserve space for `k` symbols.
    pub fn with_k(k: u8) -> Self {
        Self {
            ptr: ffi::new_kmerapi_with_len(k as u32),
        }
    }

    /// Number of symbols `k` of this kmer.
    pub fn len(&self) -> u32 {
        self.ptr.kmer_len()
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
        self.ptr.pin_mut().set_u64(val);
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
        self.ptr.as_u64()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl std::fmt::Display for ffi::Kmer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(&self.to_string())
    }
}

impl std::fmt::Display for Kmer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.ptr.fmt(f)
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
        assert_eq!(KmcFile::open_iter("./data/test1")?.iter_u64().count(), 291);
        Ok(())
    }

    #[test]
    fn test_iter_count_taaga() -> Result<(), String> {
        assert_eq!(
            KmcFile::open_iter("./data/test1")?
                .iter_u64()
                .filter(|&(b, _)| { b == 0b11_00_00_10_00 })
                .map(|(_, c)| c)
                .next()
                .ok_or("should not happen")?,
            4
        );
        Ok(())
    }
}
