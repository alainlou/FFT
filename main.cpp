#include <iostream>
#include <vector>

#include "util.hpp"

int main(int argc, char *argv[]) {
    if(argc != 2) {
        std::cout << "Invalid number of parameters" << std::endl;
        return -1;
    }
    try 
    {
        std::vector<double> signal = parse(argv[1]);
    }
    catch (const std::invalid_argument& e)
    {
        std::cout << e.what() << std::endl;
    }
    return 0;
}