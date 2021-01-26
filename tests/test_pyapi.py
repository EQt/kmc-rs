import py_kmc_api
import pytest


@pytest.fixture
def io():
    from os import path

    fname = path.join(path.dirname(__file__), "..", "data", "test1")
    io = py_kmc_api.KMCFile()
    assert io.OpenForRA(fname)
    yield io
    io.Close()


def test_stats(io):
    assert io.KmerLength() == 5
    assert io.KmerCount() == 291


def test_construct_kmer():
    kmer = py_kmc_api.KmerAPI()
    assert kmer.from_string("TCTTA")
    assert kmer.to_string() == "TCTTA"


def test_reverse_kmer():
    kmer = py_kmc_api.KmerAPI()
    assert kmer.from_string("TCTTA")
    assert kmer.reverse()
    assert kmer.to_string() == "TAAGA"


def test_kmer_to_long():
    kmer = py_kmc_api.KmerAPI()
    assert kmer.from_string("TCTTA")
    long_kmer = py_kmc_api.LongKmerRepresentation()
    kmer.to_long(long_kmer)
    assert long_kmer.value == [0b11_01_11_11_00]


def test_kmer_counts(io):
    count = py_kmc_api.Count()
    assert count.value == 0
    kmer = py_kmc_api.KmerAPI()
    assert kmer.from_string("TAAGA")
    assert len(kmer.to_string()) == io.KmerLength()
    assert io.CheckKmer(kmer, count)
    assert count.value == 4
