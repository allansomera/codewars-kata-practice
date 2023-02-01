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
using std::pair;
using std::sort;
using std::stable_sort;

class WeightSort
{
public:
    static string orderWeight(const string &strng);
    static string join(const vector<pair<string, int>> &, const string &);
};

string WeightSort::join(const vector<pair<string, int>> &v, const string &delim = " "){
    string out;
    // if (auto i = v.begin(), e = v.end(); i != e){
    //     out += i->first;
    // }
    for(auto i = v.begin(); i != v.end(); ++i) out.append(i->first).append(delim);

    out.erase(std::find_if(out.rbegin(), out.rend(), std::bind1st(std::not_equal_to<char>(), ' ')).base(), out.end());
    return out;
}

string WeightSort::orderWeight(const string &strng){
    vector<pair<string,int>> vp;
    istringstream ss(strng);
    string s;
    while(getline(ss, s, ' ')){
        int sum = 0;
        for(const auto& ch: s){
            int num = stoi(string(1,ch));
            sum += num;
        }
        vp.push_back(pair<string, int>(s, sum));
    }
    for(auto i = vp.begin(); i != vp.end(); i++){
        cout << "s: " << i->first << " sum: " << i->second << endl;
    }
    sort(vp.begin(), vp.end(), [=](pair<string, int>  &a, pair<string,int>& b){
            // if a.second and b.second are equal, compare with the string instead
            // otherwise keep comparing with the ints
            return a.second == b.second ? a.first < b.first : a.second < b.second;
    });
    cout << "\n";
    for(auto i = vp.begin(); i != vp.end(); i++){
        cout << "s: " << i->first << " sum: " << i->second << endl;
    }
    string final = join(vp, " ");
    return final; 
}

int main(){
    cout << WeightSort::orderWeight("103 123 4444 99 2000");
    return 0;
}


