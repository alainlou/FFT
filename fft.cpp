#include <cmath>
#include <complex>
#include <vector>

#include "fft.hpp"

#define TAU 6.283185307179586476925286766559005768394338798750211641949

std::vector<std::complex<double>> dft_naive(std::vector<double> signal) {
    size_t n = signal.size();
    std::vector<std::complex<double>> fourier(n, std::complex<double>(0, 0));
    for (size_t i = 0; i < n; ++i) {        
        for (size_t j = 0; j < n; ++j) {
            fourier[i] += std::complex<double>(signal[j] * std::cos(TAU/n*i*j), -signal[j] * std::sin(TAU/n*i*j));
        }
    }
    return fourier;
}
