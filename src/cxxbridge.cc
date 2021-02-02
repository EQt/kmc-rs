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

    inline uint64_t data0() const { return this->kmer_data[0]; }

    inline uint64_t as_u64() const
    {
        const auto offset = this->kmer_length + this->byte_alignment;
        return data0() >> (64 - (offset * 2));
    }

    bool set_u64(uint64_t val)
    {
        if (this->kmer_length > 32)
            return false;
        const auto offset = this->kmer_length + this->byte_alignment;
        this->kmer_data[0] = (uint64)val << (64 - (offset * 2));
        return true;
    }

    uint32_t kmer_len() const { return this->kmer_length; }

#ifdef HAVE_RUST
    bool from_string(rust::Str kmer)
    {
        return CKmerAPI::from_string(std::string(kmer));
    }

    rust::String to_string() const { return CKmerAPI::to_string(); }
#endif
};


struct KmcFile : public CKMCFile
{
#ifdef HAVE_RUST
    bool open_for_ra(const rust::Str fname) { return OpenForRA(std::string(fname)); }
    bool open_for_iter(const rust::Str fn) { return OpenForListing(std::string(fn)); }
#endif

    inline std::size_t kmer_count() { return KmerCount(); }

    inline bool next(Kmer &kmer, size_t &count)
    {
        uint64 count2;
        const bool r = ReadNextKmer(kmer, count2);
        count = count2;
        return r;
    }

    inline bool restart_listing() { return RestartListing(); }

    inline uint32_t kmer_len() const { return KmerLength(); }

    inline size_t check_kmer(const Kmer &kmer) const
    {
        uint64 counter = 0;
        if (CheckKmer(kmer, counter))
            return (size_t)counter;
        return 0;
    }

    inline bool close() { return Close(); }
};


inline std::unique_ptr<KmcFile>
new_ckmc_file()
{
    return std::make_unique<KmcFile>();
}


inline std::unique_ptr<Kmer>
new_kmerapi()
{
    return std::make_unique<Kmer>();
}


inline std::unique_ptr<Kmer>
new_kmerapi_with_len(uint32_t k)
{
    return std::unique_ptr<Kmer>(new Kmer(k));
}
