#pragma once
#include "../KMC/kmc_api/kmc_file.h"
#include <memory> // for std::unique_ptr


struct KmcFile : public CKMCFile
{
    std::size_t KmerCount() { return CKMCFile::KmerCount(); }
};


std::unique_ptr<KmcFile>
new_ckmc_file();
