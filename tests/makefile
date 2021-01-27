KMC_API_DIR = ../KMC/kmc_api
KMC_API_OBJS = $(addprefix $(KMC_API_DIR)/,mmer.o kmc_file.o kmer_api.o)

test: test_api
	./$^

test_api: test_api.cc ../src/cxxbridge.cc $(KMC_API_OBJS)
	$(CXX) -o $@ -Wall -pedantic $(filter %.cc,$^) $(filter %.o,$^)

clean:
	rm -fr *.o test_api
