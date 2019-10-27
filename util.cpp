#include <regex>
#include <stdexcept>
#include <string>

#include "util.hpp"

const std::regex REGEX = std::regex("\\[([0-9]+(\\.[0-9]+|))+(,[0-9]+(\\.[0-9]+|))*\\]");

std::vector<double> parse(char *p_str) {
    std::string str(p_str);
    if (!std::regex_match(str, REGEX)) {
        throw std::invalid_argument("Invalid array");
    }
    std::vector<double> vec;    
    std::string buffer;
    for (size_t i = 1; i < str.size(); ++i) {
        if (str[i] == ',' || str[i] == ']') {
            vec.push_back(std::stod(buffer));
            buffer = "";
        } else {
            buffer.push_back(str[i]);    
        }   
    }
    return vec;
}
