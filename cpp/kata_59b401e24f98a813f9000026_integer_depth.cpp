#include <iostream> 
#include <vector>
#include <unordered_map>
#include <string>
using std::cout; using std::endl;
using std::vector; using std::unordered_map;
using std::string;

bool check_map(unordered_map<int, int> &um){
    for(auto& [key, value] : um){
        // cout << "key is: " << key << endl;
        // cout << "value is: " << value << endl;
        if(value == 0) return 0;
    }
    return 1;
}

unsigned integer_depth(unsigned n){
    // vector<int> v;
    unordered_map<int, int> um;
    //initialize um
    for(int i = 0; i <= 9; i++){
        um[i] = 0;
    }

    //need to use a while loop instead
    int i = 1;
    while(!check_map(um)) {
        int res = n * i;
        string str_res;
        // cout << res << endl;
        str_res = std::to_string(res);
        // cout << "value: " << str_res << endl;
        for(string::size_type i = 0; i < str_res.size(); i++){
            //store char as key on map
            um[std::stoi(string(1, str_res[i]))]++; 
            // cout << "digits " << str_res[i] << endl;
        }
        if(check_map(um)){
            // cout << "used all numbers, i is: " << i << endl;
            return i;
        } 
        i++;

    }
    return 0;
}


int main(){
    integer_depth(42);
    return 0;
}
