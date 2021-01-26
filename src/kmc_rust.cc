#include "kmc_rust.hh"


std::unique_ptr<KmcFile> new_ckmc_file() {
    return std::make_unique<KmcFile>();
}
