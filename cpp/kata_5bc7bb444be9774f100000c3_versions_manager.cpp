#include <exception>
#include <iterator>
#include <string>
#include <string_view>
#include <vector>
#include <iostream>
#include <sstream>
#include <stack>
#include <cctype>

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
using std::stack;
using std::isalpha;

class VersionException : public std::exception{
    private:
        char * message_;

    public:
        VersionException(char * msg) : message_(msg){}
        char * what () {
            return message_;
        }
};


class VersionManager
{
    public:
        VersionManager(string_view version = "0.0.1");
        VersionManager& major();
        VersionManager& minor();
        VersionManager& patch();
        VersionManager& rollback();
        string release();
        void check(string_view);
        void set_rb_version();
        string_view* build_string();
        void join(vector<int>&, ostringstream&);
        
    private:
        stack<vector<int>> svi_old_ver_;
        string_view version_;
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
    set_rb_version();
    ++MAJOR_;
    MINOR_ = 0;
    PATCH_ = 0;
    return *this;
}

VersionManager& VersionManager::minor(){
    set_rb_version();
    ++MINOR_;
    PATCH_ = 0;
    return *this;
}

VersionManager& VersionManager::patch(){
    set_rb_version();
    ++PATCH_;
    return *this;
}

VersionManager& VersionManager::rollback(){
    if(svi_old_ver_.empty()){
        throw VersionException("Cannot rollback!");
        return *this;
    }
    vector<int> vi = svi_old_ver_.top();
    ostringstream oss;
    join(vi, oss);
    check(oss.str());
    version_ = oss.str();
    svi_old_ver_.pop();
    return *this;
}

string VersionManager::release(){

    vector<int> str_v = {MAJOR_, MINOR_, PATCH_};
    ostringstream oss;
    join(str_v, oss);
    return oss.str();
}

void VersionManager::join(vector<int>& vi, ostringstream& oss){
    // joining a vector<int> with delimeter '.'
    auto b = begin(vi), e = end(vi);
    if(b != e){
        std::copy(
            b, prev(e), std::ostream_iterator<int>(oss, "."));
            b = prev(e);
    }
    if(b != e){
        oss << *b;
    }
}

void VersionManager::check(string_view str){
    // vector<string> vs;
    vector<int> vi;
    istringstream ss(string(str), std::ios_base::in);
    string temp;
    //this loop will split string with delimeter '.'
    while(std::getline(ss, temp, '.')){
        if(vi.size() >= 3) break;
            if(isalpha(temp[0])){
                // cout << "character is a letter: " << temp << endl;
                throw VersionException("Error occured while parsing version!");
                break;
            } 
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
        MINOR_ = vi[1]; PATCH_ = vi[2];
    }
}

void VersionManager::set_rb_version(){
    vector<int> vi = {MAJOR_, MINOR_, PATCH_};
    ostringstream oss;
    join(vi, oss);
    // cout << "rollback version: " << oss.str() << endl;
    svi_old_ver_.push(vi);
}

// Implement methods here.
// Use preloaded VersionException class for throwing exceptions.

int main(){
    // cout << VersionManager("a.b.c").release() << endl;
    // cout << VersionManager().minor().release() << endl;
    // cout << VersionManager().patch().release() << endl;
    // cout << VersionManager("4").minor().patch().patch().release() << endl;
    // cout << VersionManager("1.2.3").major().major().release() << endl;
    // cout << "rollback: VersionManager(1): " << VersionManager("1").major().minor().rollback().release() << endl;
    // cout << "rollback: VersionManager(): " << VersionManager().major().rollback().release() << endl;
    // cout << "empty string: " << VersionManager("0.2.1").release() << endl;
    // cout << VersionManager().major().patch().rollback().release() << endl;
    // cout << VersionManager().major().patch().rollback().major().rollback().release() << endl;
    cout << "VersionManager.major().patch().release() " << endl;
    cout << VersionManager().major().patch().release() << endl;

    cout << "rollback()" << endl << VersionManager().major().patch().rollback().release() << endl;
    // cout << VersionManager().major().patch().rollback().rollback().release() << endl;

    // vm.print();
    // vm.release();
    return 0;
}
