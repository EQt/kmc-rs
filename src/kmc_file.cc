#include "kmc_file.hh"


std::unique_ptr<CKMCFile> new_ckmc_file() {
    return std::make_unique<CKMCFile>();
}
