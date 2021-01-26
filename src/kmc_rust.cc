#include "kmc_rust.hh"


std::unique_ptr<KmcFile>
new_ckmc_file()
{
    return std::make_unique<KmcFile>();
}


std::unique_ptr<Kmer>
new_kmerapi()
{
    return std::make_unique<Kmer>();
}
