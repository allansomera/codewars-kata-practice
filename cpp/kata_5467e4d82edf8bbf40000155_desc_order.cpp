#include <cinttypes>
#include <iostream>
#include <string>
#include <vector>
#include <algorithm>


uint64_t descendingOrder(uint64_t a)
{
    std::string str = std::to_string(a);
    std::vector<int> v;
    std::string res;
    // std::cout << "a to str: " << str << std::endl;
    for (const auto& ch: str){
        int num = std::stoi(std::string(1,ch));
        v.push_back(num);
        std::cout << num << " ";
    }
    std::cout << std::endl;

    std::sort(v.begin(), v.end(), std::greater<>());
    
    for(int i: v) {
        std::cout << i << " ";
        res += std::to_string(i);
    }
    //using std::stoul because stoi reached max limit for int
    return (uint64_t)std::stoul(res);
    
}

int main(){
    descendingOrder(1234);
    return 0;
}
