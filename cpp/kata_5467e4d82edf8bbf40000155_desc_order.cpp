#include <cinttypes>
#include <iostream>
#include <string>

uint64_t descendingOrder(uint64_t);

uint64_t descendingOrder(uint64_t a)
{
    std::string str = std::to_string(a);
    std::cout << "a to str: " << str << std::endl;
    return 0;
}

int main(){
    descendingOrder(1234);
    return 0;
}
