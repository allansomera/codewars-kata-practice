
#include <numeric>
#include <iostream>
#include <vector>
#include <algorithm>
#include <unordered_set>
#include <set>
#include <iterator>

using std::cout;
using std::endl;
// using std::iota;

// had to use a python solution instead, i cant seem to find a a solution to use c++
std::vector<unsigned int> missNumsFinder(const std::vector<unsigned int>& arr) {
// void missNumsFinder(const std::vector<unsigned int>& arr) {
    
    cout << "arr is: ";
    for(const unsigned int &i : arr){
        cout << i << ' ';
    }
    cout << endl;
    // std::vector<unsigned int>::iterator max;
    // find highest number in vector
    unsigned int max = *std::max_element(arr.begin(), arr.end());
    
    std::vector<unsigned int> diff;
    // copy arr to dummy
    // std::vector<unsigned int> dummy;
    // std::unordered_set<unsigned int> dummy(arr.begin(), arr.end());
    // std::set<unsigned int> dummy(arr.begin(), arr.end());
    std::unordered_set<unsigned int> dummy;
    for(auto i: arr)
        dummy.insert(i);
    
    int n = arr.size();

    for(int i = 1; i <=n; i++){
        if(dummy.find(i) == dummy.end()){
            diff.push_back(i);
        }
    }

    cout << "dummy is: ";
    for(const unsigned int &i : dummy){
        cout << i << ' ';
    }
    cout << endl;
    //create vector of 1 to max range
    // std::vector<unsigned int> v(max);
    // std::iota(v.begin(), v.end(), 1);
    // std::set<unsigned int> v2(v.begin(), v.end());
    // std::unordered_set<unsigned int> us(v.begin(), v.end());
    // sort(dummy.begin(), dummy.end());
    
    // std::set_difference(v.begin(), v.end(), dummy.begin(), dummy.end(), 
    //         std::inserter(diff, diff.begin()));
    

    // std::copy_if(us.begin(), us.end(), std::inserter(diff, diff.begin()), 
    //         [&](unsigned int x){return !dummy.count(x);});

    // std::set<unsigned int>::iterator it = dummy.begin();
    // for(unsigned int i = 1; i <= max; i++){
    //     if(*it == i){
    //         it++;
    //     }else{
    //         diff.push_back(i);
    //     } 
    // }


    // sort(diff.begin(), diff.end());

    cout << "diff is: ";
    for(unsigned int &i: diff){
        cout << i << ' ';
    }
    cout << endl;
    
    // unsigned int one = 1;
    // if(!std::binary_search(dummy.begin(), dummy.end(), one)) {
    //     if(std::binary_search(diff.begin(), diff.end(), one)){
    //         return diff;
    //     }
    //     // else{
    //     //     diff.insert(diff.begin(), one);
    //     //     return diff;
    //     // }
    // }
    
    // cout << "max is: " << max << endl;

    return diff;
    // return {};
}

int main(){
    std::vector<unsigned int> arr = {5};
    missNumsFinder(arr);
}

