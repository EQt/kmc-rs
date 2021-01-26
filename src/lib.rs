#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("kmc-rs/src/kmc_rust.hh");
        type KmcFile;
        type Kmer;

        fn new_ckmc_file() -> UniquePtr<KmcFile>;
        fn OpenForRA(self: Pin<&mut KmcFile>, fname: &str) -> bool;
        fn KmerLength(self: Pin<&mut KmcFile>) -> u32;
        fn KmerCount(self: Pin<&mut KmcFile>) -> usize;
        fn CheckKmer(self: Pin<&mut KmcFile>, kmer: Pin<&mut Kmer>) -> usize;
        fn Close(self: Pin<&mut KmcFile>) -> bool;

        fn new_kmerapi() -> UniquePtr<Kmer>;
        fn new_kmerapi_with_len(k: u32) -> UniquePtr<Kmer>;
        fn from_string(self: Pin<&mut Kmer>, kmer: &str) -> bool;
        fn set_u64(self: Pin<&mut Kmer>, val: u64) -> bool;
        fn to_string(self: Pin<&mut Kmer>) -> String;
    }
}


pub struct KmcFile {
    handle: cxx::UniquePtr<ffi::KmcFile>,
}

pub struct Kmer {
    handle: cxx::UniquePtr<ffi::Kmer>,
}

impl KmcFile {
    /// Open for random access mode.
    /// The file name `fname` must not include the suffixes `.kmc_pre` or `.kmc_suf`.
    pub fn open_ra(fname: &str) -> Result<Self, String> {
        let mut handle = ffi::new_ckmc_file();
        if handle.pin_mut().OpenForRA(fname) {
            Ok(Self { handle })
        } else {
            Err(format!("Could not open '{}' for random access", fname))
        }
    }

    /// The parameter `k` when this data base was constructed with.
    pub fn kmer_length(&mut self) -> u32 {
        self.handle.pin_mut().KmerLength()
    }

    /// Number of (canical) k-mers in the data base
    pub fn num_kmers(&mut self) -> usize {
        self.handle.pin_mut().KmerCount()
    }

    /// How often did the canonical `kmer` occur?
    pub fn count_kmer(&mut self, kmer: &mut Kmer) -> usize {
        self.handle.pin_mut().CheckKmer(kmer.handle.pin_mut())
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
    pub fn from(kmer: &str) -> Result<Self, String> {
        let mut handle = ffi::new_kmerapi();
        if !handle.pin_mut().from_string(kmer) {
            return Err(format!("Internal Error in CKmerApi::from_string"));
        }
        Ok(Self { handle })
    }

    pub fn to_string(&mut self) -> String {
        self.handle.pin_mut().to_string()
    }

    pub fn with_k(k: u8) -> Self {
        assert!(k <= 32);
        Self {
            handle: ffi::new_kmerapi_with_len(k as u32),
        }
    }

    pub fn from_u64(k: u8, val: u64) -> Self {
        let mut kmer = Self::with_k(k);
        kmer.set_u64(val);
        kmer
    }

    pub fn set_u64(&mut self, val: u64) {
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
        let mut io = KmcFile::open_ra("./data/test1")?;
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
