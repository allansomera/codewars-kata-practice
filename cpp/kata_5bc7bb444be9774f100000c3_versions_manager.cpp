#include <iterator>
#include <string>
#include <string_view>
#include <vector>
#include <iostream>
#include <sstream>

using std::vector;
using std::string_view;
using std::cout;
using std::endl;
using std::istringstream;
using std::ostringstream;
using std::string;
using std::begin;
using std::end;
using std::prev;


class VersionManager
{
    public:
        VersionManager(string_view version = "0.0.1");
        VersionManager& major();
        VersionManager& minor();
        VersionManager& patch();
        VersionManager& rollback();
        string release();
        void check(string_view&);
        void set_rollback_version(string_view&);
        string_view* build_string();
        
    private:
        string_view version_;
        string_view rollback_version_;
        int MAJOR_;
        int MINOR_;
        int PATCH_;
};


VersionManager::VersionManager(string_view version) 
    // : version_(version.empty() ? "0.0.1" : version), MAJOR_(0), MINOR_(0), PATCH_(1){}
    : version_(version.empty() ? "0.0.1" : version){
        check(version_);
    }

VersionManager& VersionManager::major(){
    ++MAJOR_;
    MINOR_ = 0;
    PATCH_ = 0;
    return *this;
}

VersionManager& VersionManager::minor(){
    ++MINOR_;
    PATCH_ = 0;
    return *this;
}

VersionManager& VersionManager::patch(){
    ++PATCH_;
    return *this;
}

string VersionManager::release(){

    vector<int> str_v = {MAJOR_, MINOR_, PATCH_};
    ostringstream oss;

    // joining a vector<int> with delimeter '.'
    auto b = begin(str_v), e = end(str_v);
    if(b != e){
        std::copy(
            b, prev(e), std::ostream_iterator<int>(oss, "."));
            b = prev(e);
    }
    if(b != e){
        oss << *b;
    }
    return oss.str();
}

void VersionManager::check(string_view& str){
    // vector<string> vs;
    vector<int> vi;
    istringstream ss(string(str), std::ios_base::in);
    string temp;
    //this loop will split string with delimeter '.'
    while(std::getline(ss, temp, '.')){
        vi.push_back(std::stoi(temp));
    }
    // for(auto& i : vi) cout << i << endl;
    if(vi.size() == 1){
        MAJOR_ = vi[0];
        MINOR_ = 0;
        PATCH_ = 0;
    }
    else if(vi.size() == 2){
        MAJOR_ = vi[0];
        MINOR_ = vi[1];
        PATCH_ = 0;
    }
    else{
        MAJOR_ = vi[0];
        MINOR_ = vi[1];
        PATCH_ = vi[2];
    }
}

void VersionManager::set_rollback_version(string_view& rb_string){
    
}
// Implement methods here.
// Use preloaded VersionException class for throwing exceptions.

int main(){
    cout << VersionManager().major().release() << endl;
    cout << VersionManager().minor().release() << endl;
    cout << VersionManager().patch().release() << endl;
    cout << VersionManager("4").minor().patch().patch().release() << endl;
    cout << VersionManager("1.2.3").major().major().release() << endl;
    cout << "empty string: " << VersionManager("0.2.1").release() << endl;
    // vm.print();
    // vm.release();
    return 0;
}
