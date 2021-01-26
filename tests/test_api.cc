#include "../KMC/kmc_api/kmc_file.h"
#include "../KMC/kmc_api/kmer_api.h"
#include <bitset>
#include <iostream>


template <typename T>
inline std::string
bitstring(T x)
{
    static_assert(std::is_integral<T>::value, "Integral required.");
    const unsigned long long val = (unsigned long long)x;
    return std::bitset<8 * sizeof(T)>(val).to_string();
}


void
check(const char *msg, const bool good)
{
    if (!good) {
        std::cerr << msg << std::endl;
        exit(1);
    }
}


struct Kmer : public CKmerAPI
{
  public:
    uint64_t data0() const { return this->kmer_data[0]; }
};


int
main()
{
    CKMCFile io;
    check("OpenForRA", io.OpenForRA("../data/test1"));
    check("k", io.KmerLength() == 5);

    {
        Kmer kmer;
        check("kmer from_string", kmer.from_string("TAAGA"));
        std::cout << "data as long: " << bitstring(kmer.data0()) << std::endl
                  << "              " << bitstring(0b11'00'00'10'00L << (32 + 16))
                  << std::endl;
        check("kmer long", kmer.data0() == 0b11'00'00'10'00L << (32 + 16));
    }
    {
        uint64 count;
        CKmerAPI kmer;
        check("", kmer.from_string("TAAGA"));
        check("check", io.CheckKmer(kmer, count));
        check("count correct", count == 4);
    }

    return 0;
}
