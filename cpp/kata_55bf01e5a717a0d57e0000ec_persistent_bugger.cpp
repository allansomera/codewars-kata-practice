#include <iostream>
#include <vector>
#include <functional>
#include <numeric>

using std::cout;
using std::endl;
using std::vector;
using std::multiplies;
using std::accumulate;
using std::begin;
using std::end;

int persistence(long long n){
    vector<int> v;
    cout << endl;
    // int count = 0;
    // for(;n>0;n/=10)
    //     count++;
    
    //single out a digit from n and store in vector
    cout << "n is: " << n << endl;
    for(;n>0;) {
        v.push_back(n%10);
        n/=10;
    }

    
    //multiply each element with each other
    int count = 0;
    for(;v.size() != 1; ++count){
        auto multi = std::accumulate(begin(v), end(v), 1, std::multiplies<>());
        cout << "multi is: " << multi << endl;
        v.clear();
        for(;multi>0;) {
            v.push_back(multi%10);
            multi/=10;
        }
        if(v.size() == 0 || v.size() == 1 ) return ++count;
        
        cout << "v.size() is: " << v.size() << endl;
    }
    // cout << multi;
    //print in reverse
    // for(vector<int>::iterator it = v.end()-1; it >= v.begin(); it--) cout  << *it;
    return count;
}


int main(){
    cout << persistence(39) << endl;
    cout << persistence(4) << endl;
    cout << persistence(25) << endl;
    cout << persistence(999) << endl;
    cout << persistence(444) << endl;
    return 0;
}
