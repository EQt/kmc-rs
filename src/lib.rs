#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("kmc-rs/src/kmc_rust.hh");
        type KmcFile;
        type Kmer;

        fn new_ckmc_file() -> UniquePtr<KmcFile>;
        fn OpenForRA(self: Pin<&mut KmcFile>, fname: &CxxString) -> bool;
        fn KmerLength(self: Pin<&mut KmcFile>) -> u32;
        fn KmerCount(self: Pin<&mut KmcFile>) -> usize;
        fn CheckKmer(self: Pin<&mut KmcFile>, kmer: Pin<&mut Kmer>) -> usize;

        fn new_kmerapi() -> UniquePtr<Kmer>;
        fn from_string(self: Pin<&mut Kmer>, kmer: &[u8]) -> bool;
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
    /// Open for random access mode
    pub fn open_ra(fname: &str) -> Result<Self, String> {
        let mut handle = ffi::new_ckmc_file();
        cxx::let_cxx_string!(fstr = fname.as_bytes());
        if handle.pin_mut().OpenForRA(&fstr) {
            Ok(Self { handle })
        } else {
            Err(format!("Could not open '{}' for random access", fname))
        }
    }

    pub fn kmer_length(&mut self) -> u32 {
        self.handle.pin_mut().KmerLength()
    }

    pub fn num_kmers(&mut self) -> usize {
        self.handle.pin_mut().KmerCount()
    }

    pub fn count_kmer(&mut self, kmer: &mut Kmer) -> usize {
        self.handle.pin_mut().CheckKmer(kmer.handle.pin_mut())
    }
}

impl Kmer {
    pub fn from(kmer: &[u8]) -> Result<Self, String> {
        if kmer.last().unwrap() != &b'\0' {
            return Err("Must end with zero byte!".into());
        }
        let mut handle = ffi::new_kmerapi();
        if !handle.pin_mut().from_string(kmer) {
            return Err(format!("Internal Error in CKmerApi::from_string"));
        }
        Ok(Self { handle })
    }

    pub fn to_string(&mut self) -> String {
        self.handle.pin_mut().to_string()
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
        let mut kmer = Kmer::from(b"TAAGA\0")?;
        let s = kmer.to_string();
        assert_eq!(&s, "TAAGA", "got {}", &s);
        Ok(())
    }

    #[test]
    fn test_count_kmer() -> Result<(), String> {
        let mut kmer = Kmer::from(b"TAAGA\0")?;
        let mut io = KmcFile::open_ra("./data/test1")?;
        assert_eq!(io.count_kmer(&mut kmer), 4);
        Ok(())
    }
}
