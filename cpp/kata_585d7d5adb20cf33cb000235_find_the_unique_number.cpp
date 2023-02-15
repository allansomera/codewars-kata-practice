#include <vector>
#include <set>
#include <unordered_set>
#include <unordered_map>
#include <map>
#include <iostream>
#include <bits/stdc++.h>

using std::vector;
using std::set;
using std::cout;
using std::endl;
using std::unordered_set;
using std::unordered_map;
using std::map;
#define f(x,y)*(int*)x^=*(int*)y

float find_uniq(const std::vector<float> &v)
{
    // set<float> sf{begin(v), end(v)};
    // unordered_set<float> sf{begin(v), end(v)};
    // for(auto it = sf.begin(); it != sf.end(); it++){
    //     cout << *it << endl;
    // }
    // return *sf.begin();
    // return *std::prev(sf.begin());
    // return 0.0;

    // float a = v.front();
    // for(auto& it : v){
    //     if(it != a){
    //         return a;
    //     }
    //     a = it;
    // }
    // return a;

    //map works faster on submission, even though unordered_map should be faster
    map<float, size_t> m;
    for(vector<float>::const_iterator it = v.begin(); it != v.end(); it++){
        ++m[*it];
    }

    for(auto& [key, value]: m){
        if(value == 1) {
            return key;
        }
    }

    // for(auto& x:um){
    //     if(x.second==1) return x.first;
    // }
    
    // float a = 0.0;
    // for(vector<float>::const_iterator it = v.begin(); it != v.end(); it++){
    //     f(&a, &(*it));
    // }
    //
    // return a;
    // vector<float> vf{v.begin(), v.end()};
    //
    // std::sort(vf.begin(), vf.end(), std::greater<float>());
    // for(auto& x:vf) cout << x << " ";
    // cout << endl;
    //     
    // return 0;
    throw std::runtime_error("No unique element found");
}

int main(){
    // cout << find_uniq(std::vector<float>{1, 1, 1, 2, 1, 1});
    cout << find_uniq(std::vector<float>{1, 1, 1, 2, 1, 1}) << endl;;
    cout << find_uniq(std::vector<float>{ 0, 0, 0.55, 0, 0 }) << endl;
    cout << find_uniq( std::vector<float>{ 999.666,  999.666, 999.666, 999.666, 999.666, 999.666, 999.666, 999, 999.666 }) << endl;
    return 0;
}
