#include "../src/cxxbridge.cc"
#include "bitstring.hh"


void
check(const char *msg, const bool good)
{
    if (!good) {
        std::cerr << msg << std::endl;
        exit(1);
    }
}


void
test_ra()
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
        uint64 count;
        check("check", io.CheckKmer(kmer, count));
    }
    {
        uint64 count;
        CKmerAPI kmer;
        check("", kmer.from_string("TAAGA"));
        check("check", io.CheckKmer(kmer, count));
        check("count correct", count == 4);
    }
}


void
test_it()
{
    CKMCFile io;
    check("OpenForListing", io.OpenForListing("../data/test1"));
    const auto k = 5;
    check("k", io.KmerLength() == 5);
    Kmer kmer(k);

    uint64 count;
    size_t no_kmers = 0;
    while (io.ReadNextKmer(kmer, count))
        no_kmers++;
    check("no of kmers", no_kmers == 291);
}


int
main()
{
    test_it();
    test_ra();
    return 0;
}
