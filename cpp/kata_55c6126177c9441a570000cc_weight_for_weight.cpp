#include <string>
#include <iostream>
#include <sstream> 
#include <cinttypes>
#include <vector>
#include <algorithm>

using std::cout;
using std::endl;
using std::string;
using std::vector;
using std::istringstream;
using std::getline;
using std::stoi;

class WeightSort
{
public:
    static string orderWeight(const string &strng);
};

string WeightSort::orderWeight(const string &strng){
    vector<vector<int>> v;
    istringstream ss(strng);
    string s;
    while(getline(ss, s, ' ')){
        vector<int> vi;
        for(const auto& ch: s){
            int num = stoi(string(1,ch));
            vi.push_back(num);
        }
        v.push_back(vi);
    }
    vector<int> sumv;
    for(vector<int> vi: v){
        int sum = 0;
        for(int i: vi){
            sum+=i;
        }
        sumv.push_back(sum);
    }
    for(int i: sumv){
        cout << i << endl;
    }
    //TODO: use a vector of pairs or hashmap to store string, and added value
    // { "103", 4 } , {"123", 6}
    return strng; 
}

int main(){
    cout << WeightSort::orderWeight("103 123 4444 99 2000");
    return 0;
}


