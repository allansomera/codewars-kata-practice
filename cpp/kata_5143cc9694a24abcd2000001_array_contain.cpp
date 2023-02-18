#include <vector>
#include <unordered_map>
#include <iostream>

using std::vector;
using std::unordered_map;
using std::cout;
using std::endl;

bool contains_all(const std::vector<int>& arr, const std::vector<int>& target) {
    unordered_map<int, int> um;
    
    for(vector<int>::const_iterator it = arr.begin() ; it != arr.end(); it++){
        if(target.size() == 1){
            cout << "arr *it: " << *it << " target[0]: " << target[0] << endl;
            if(*it == target[0]){
                return true;
                // ++um[target.front()];
            } else { um[target[0]];}
        }else{
            for(vector<int>::const_iterator pt = target.begin(); pt != target.end(); pt++){
                // um[*pt] = 0;
                um[*pt]; // initialize to 0, new to me haha didnt know you can do this
                if(*pt == *it) ++um[*pt];
                // cout << "*pt: " << *pt << " um[*pt]: " << um[*pt] << endl;
            }
        }
    }

    for(auto& [key, value] : um ){
        // cout << "key: " << key << " value: " << value << endl;
        if (value == 0){
            return false;
        }
    }

    return true;
}

int main(){
    vector<int> nums{1, 2, 3, 4, 5, 6, 7, 8, 9};
    cout << contains_all(nums, vector<int>{1,2,6}) << endl;
    cout << contains_all(nums, vector<int>{1}) << endl;
    cout << contains_all(nums, vector<int>{9}) << endl;
    cout << contains_all(nums, vector<int>{2,4,6,8}) << endl;
    // cout << contains_all(nums, vector<int>{0}) << endl;
    // cout << contains_all(nums, vector<int>{1,15,6}) << endl;
    return 0;

}
