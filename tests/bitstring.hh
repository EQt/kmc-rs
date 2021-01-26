#pragma once
#include <bitset>
#include <iostream>


template <typename T>
inline std::string
bitstring(T x)
{
    static_assert(std::is_integral<T>::value, "Integral required.");
    const unsigned long long val = (unsigned long long)x;
    return "0b" + std::bitset<8 * sizeof(T)>(val).to_string('_', '1');
}
