fn main() {
    cxx_build::bridge("src/cxxbridge.rs")
        .file("KMC/kmc_api/kmc_file.cpp")
        .file("KMC/kmc_api/mmer.cpp")
        .file("KMC/kmc_api/kmer_api.cpp")
        .flag_if_supported("-std=c++14")
        .compile("kmc-rs");
}
