#include <vector>
#include <iostream>
#include <string>
#include <regex>

using std::cout;
using std::endl;
using std::vector;
using std::string;
using std::regex;

bool check_spaces(string& s){
    return (!std::isspace(static_cast<unsigned char>(s.front())) && 
     !std::isspace(static_cast<unsigned char>(s.back())));
}
std::vector<std::string> wave(std::string y){
    vector<string> vs;
    vector<size_t> pos;
    vector<size_t>::iterator it;
    // size_t idx = 0;
    size_t count = 0;
    // string ck_str;

    // if(!check_spaces(y)){
    //     //remove trailing/leading spaces, as well as multiple whitepaces in the middle
    //     // ck_str = std::regex_replace(y, std::regex("^ +| +$|( ) +"), "$1");
    //     ck_str = std::regex_replace(y, std::regex("^ +| +$"), "$1");
    // }
    // else {
    //     ck_str = y;

    // count how many letters, and add index to pos vector
    for(size_t i = 0; i < y.length() ; i++ ){
        if(std::isalpha(y[i])) {
            pos.push_back(i);
            count++;
        }
    }

    // cout << "count: " << count << endl;
    //add each word to vector 
    for(size_t i = 1; i <= count; i++) vs.push_back(y);
    //point iterator to beginning of pos vector
    it = pos.begin();
    // cout << "pos[0]: " << *it << endl;
    // ++it;
    // cout << "pos[1]: " << *it << endl;
    // for(auto& i : pos) cout << i << endl;
    // for(auto& i : vs) cout << i << endl;
    for(size_t i = 0; i < vs.size(); i++){
        vs[i][*it] = std::toupper(vs[i][*it]);
        ++it;
    }
    
    

    // if( idx > 0){
    //     vs.erase(vs.begin() + idx);
    // }

    // if(!check_spaces(y)){
    //     for( auto& i : vs){
    //         // i.insert(+0, " ");
    //         for(size_t k = 0; k < i.size(); k++){
    //             if(k == 0 ){
    //                 i.insert(k++, " ");
    //             }
    //             // need to move pass the last index to insert space because new length was done
    //             // after inserting the first space in the beginning
    //             if( k == (i.size() - 1) ){
    //                 i.insert(++k," ");
    //             }
    //         }
    //         cout << i << endl;
    //     }
    // }
    
    return vs;
}

int main(){
    // vector<string> eq{"Hello","hEllo","heLlo","helLo","hellO"};
    vector<string> eq{" Gap ", " gAp ", " gaP "};
    // cout << (eq == wave("hello") ? "PASS" : "FAIL")  << endl;
    cout << (eq == wave(" gap ") ? "PASS" : "FAIL")  << endl;
    return 0;
}
