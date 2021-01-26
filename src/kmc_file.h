#ifndef FUNCTIONS_H_INCLUDED
#define FUNCTIONS_H_INCLUDED
#include <memory>       // for std::unique_ptr
#include "../include/KMC/kmc_api/kmc_file.h"


std::unique_ptr<CKMCFile> new_ckmc_file();


#endif
