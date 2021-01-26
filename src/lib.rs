#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("kmc-rs/src/kmc_rust.hh");
        type KmcFile;

        fn new_ckmc_file() -> UniquePtr<KmcFile>;
        fn OpenForRA(self: Pin<&mut KmcFile>, fname: &CxxString) -> bool;
        fn KmerLength(self: Pin<&mut KmcFile>) -> u32;
        fn KmerCount(self: Pin<&mut KmcFile>) -> usize;
    }
}

pub struct KmcFile {
    handle: cxx::UniquePtr<ffi::KmcFile>,
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

    pub fn kmer_count(&mut self) -> usize {
        self.handle.pin_mut().KmerCount()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open() -> Result<(), String> {
        let mut io = KmcFile::open_ra("./data/test1")?;
        assert_eq!(io.kmer_length(), 5);
        assert_eq!(io.kmer_count(), 291);
        Ok(())
    }
}
