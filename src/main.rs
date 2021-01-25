// src/main.rs

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        // include!("kmc-rs/include/KMC/kmc_api/kmc_file.h");
        include!("kmc-rs/src/kmc_file.h");
        type CKMCFile;

        pub(crate) fn new_ckmc_file() -> UniquePtr<CKMCFile>;
    }
}

fn main() {
    let ckmc_file = unsafe { ffi::new_ckmc_file() };
}
