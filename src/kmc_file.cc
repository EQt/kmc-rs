#include "kmc-rs/include/KMC/kmc_api/kmc_file.h"
#include "kmc-rs/src/kmc_file.h"

CKMCFile::CKMCFile() {}


std::unique_ptr<CKMCFile> new_ckmc_file() {
  return std::make_unique<CKMCFile>();
}