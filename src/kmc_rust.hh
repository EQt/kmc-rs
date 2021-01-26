#pragma once
#include "../KMC/kmc_api/kmc_file.h"
#include "../KMC/kmc_api/kmer_api.h"
#if __has_include("rust/cxx.h")
#    include "rust/cxx.h"
#    define HAVE_RUST
#else
#    undef HAVE_RUST
#endif
#include <memory> // for std::unique_ptr


struct Kmer : public CKmerAPI
{
    uint64_t data0() const { return this->kmer_data[0]; }
#ifdef HAVE_RUST
    bool from_string(rust::Slice<const unsigned char> kmer)
    {
        const char *cstr = (const char *)kmer.data();
        return CKmerAPI::from_string(std::string(cstr, kmer.length()));
    }

    rust::String to_string() { return CKmerAPI::to_string(); }
#endif
};


std::unique_ptr<Kmer>
new_kmerapi();


struct KmcFile : public CKMCFile
{
    std::size_t KmerCount() { return CKMCFile::KmerCount(); }

    size_t CheckKmer(Kmer &kmer)
    {
        uint64 counter = 0;
        CKMCFile::CheckKmer(kmer, counter);
        return (size_t)counter;
    }
};


std::unique_ptr<KmcFile>
new_ckmc_file();
