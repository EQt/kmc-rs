// src/main.rs

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("kmc-rs/src/kmc_file.hh");
        type CKMCFile;

        pub(crate) fn new_ckmc_file() -> UniquePtr<CKMCFile>;
    }
}

fn main() {
    let ckmc_file = ffi::new_ckmc_file();
}
