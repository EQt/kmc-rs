#include "kmc_file.h"


std::unique_ptr<CKMCFile> new_ckmc_file() {
    return std::make_unique<CKMCFile>();
}
