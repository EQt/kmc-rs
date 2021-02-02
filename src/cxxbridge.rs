#[cxx::bridge]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("kmc-rs/src/cxxbridge.cc");
        type KmcFile;
        type Kmer;

        fn new_ckmc_file() -> UniquePtr<KmcFile>;
        fn open_for_ra(self: Pin<&mut KmcFile>, fname: &str) -> bool;
        fn open_for_iter(self: Pin<&mut KmcFile>, fname: &str) -> bool;
        fn kmer_len(self: &KmcFile) -> u32;
        fn kmer_count(self: Pin<&mut KmcFile>) -> usize;
        fn check_kmer(self: &KmcFile, kmer: &Kmer) -> usize;
        fn close(self: Pin<&mut KmcFile>) -> bool;
        fn next(self: Pin<&mut KmcFile>, kmer: Pin<&mut Kmer>, count: &mut usize) -> bool;
        fn restart_listing(self: Pin<&mut KmcFile>) -> bool;

        fn new_kmerapi() -> UniquePtr<Kmer>;
        fn new_kmerapi_with_len(k: u32) -> UniquePtr<Kmer>;
        fn from_string(self: Pin<&mut Kmer>, kmer: &str) -> bool;
        fn set_u64(self: Pin<&mut Kmer>, val: u64) -> bool;
        fn to_string(self: &Kmer) -> String;
        fn kmer_len(self: &Kmer) -> u32;
        fn as_u64(self: &Kmer) -> u64;
    }
}
