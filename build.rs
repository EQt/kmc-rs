fn main() {
    cxx_build::bridge("src/main.rs")
        .file("src/kmc_file.cc")
        .file("include/KMC/kmc_api/kmc_file.cpp")
        .flag_if_supported("-std=c++14")
        .compile("kmc-rs");
}
