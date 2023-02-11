#include <string>

unsigned hamming(const std::string &a, const std::string &b) {
    int counter = 0;
    for( std::string::size_type i = 0; i < a.size(); i++){
        if( a[i] != b[i]) counter++;
    }
    return counter;
}

