#pragma once
#include <memory>       // for std::unique_ptr
#include "../KMC/kmc_api/kmc_file.h"


std::unique_ptr<CKMCFile> new_ckmc_file();

