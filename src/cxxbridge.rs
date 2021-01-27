#[cxx::bridge]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("kmc-rs/src/cxxbridge.cc");
        type KmcFile;
        type Kmer;

        fn new_ckmc_file() -> UniquePtr<KmcFile>;
        fn OpenForRA(self: Pin<&mut KmcFile>, fname: &str) -> bool;
        fn KmerLength(self: &KmcFile) -> u32;
        fn KmerCount(self: Pin<&mut KmcFile>) -> usize;
        fn CheckKmer(self: &KmcFile, kmer: &Kmer) -> usize;
        fn Close(self: Pin<&mut KmcFile>) -> bool;

        fn new_kmerapi() -> UniquePtr<Kmer>;
        fn new_kmerapi_with_len(k: u32) -> UniquePtr<Kmer>;
        fn from_string(self: Pin<&mut Kmer>, kmer: &str) -> bool;
        fn set_u64(self: Pin<&mut Kmer>, val: u64) -> bool;
        fn to_string(self: &Kmer) -> String;
        fn KmerLength(self: &Kmer) -> u32;
    }
}
