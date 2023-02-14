#include <vector>
#include <unordered_map>
#include <iostream>

using std::cout;
using std::endl;
using std::vector;
using std::unordered_map;

int findOdd(const std::vector<int>& numbers){
  //your code here
    unordered_map<int, int> um;
    for(vector<int>::const_iterator it = numbers.begin(); it != numbers.end(); it++){
        um[*it]++;
    }
    for(auto& [key, value]: um){
        // cout << "key: " << key << " -- " << value << endl;
        if(value%2 != 0) {
            // cout << "odd val: " << value << endl;
            return key;
        }
        
    }
    return 0;
}

int main(){
    typedef vector<int> V;
    cout << findOdd(V{20,1,-1,2,-2,3,3,5,5,1,2,4,20,4,-1,-2,5});
    return 0;
}
