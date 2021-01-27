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
    using CKmerAPI::CKmerAPI;

    uint64_t data0() const { return this->kmer_data[0]; }

    bool set_u64(uint64_t val)
    {
        if (this->kmer_length > 32)
            return false;
        const auto offset = this->kmer_length + this->byte_alignment;
        this->kmer_data[0] = (uint64)val << (64 - (offset * 2));
        return true;
    }

    uint32_t KmerLength() const { return this->kmer_length; }

#ifdef HAVE_RUST
    bool from_string(rust::Str kmer)
    {
        return CKmerAPI::from_string(std::string(kmer));
    }

    rust::String to_string() const { return CKmerAPI::to_string(); }
#endif
};


std::unique_ptr<Kmer>
new_kmerapi();


std::unique_ptr<Kmer>
new_kmerapi_with_len(uint32_t k);


struct KmcFile : public CKMCFile
{
#ifdef HAVE_RUST
    bool OpenForRA(const rust::Str fname)
    {
        return CKMCFile::OpenForRA(std::string(fname));
    }
#endif

    std::size_t KmerCount() { return CKMCFile::KmerCount(); }

    size_t CheckKmer(const Kmer &kmer) const
    {
        uint64 counter = 0;
        CKMCFile::CheckKmer(kmer, counter);
        return (size_t)counter;
    }
};


std::unique_ptr<KmcFile>
new_ckmc_file();
