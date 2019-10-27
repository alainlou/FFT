#include <complex>
#include <iostream>
#include <cmath>
#include <vector>

#include "fft.hpp"
#include "util.hpp"

int main(int argc, char *argv[]) {
    if (argc != 2) {
        std::cout << "Invalid number of parameters" << std::endl;
        return -1;
    }
    try {
        std::vector<double> signal = parse(argv[1]);
        std::vector<std::complex<double>> fourier = dft_naive(signal);
        for(std::complex<double> n : fourier) {
            std::cout << std::sqrt(std::pow(n.real(), 2) + std::pow(n.imag(), 2)) << std::endl;
        }
    } catch (const std::invalid_argument &e) {
        std::cout << e.what() << std::endl;
        return -1;
    }
    return 0;
}