#include <array>
#include <iostream>
#include <vector>
#include <utility>
#include <set>
using std::array;
using std::vector;
using std::cout;
using std::endl;
using std::pair;
using std::set;

typedef vector<pair<int,vector<int>>> vecpair_iv; //vecpair_iv is a synonym for the dataype 
typedef vector<int> v_i;

bool check_row(const array<array<int, 9>, 9> & board){
    v_i v;
    for(int row = 0; row < board.size(); row++){
        set<int> set_row(board[row].begin(), board[row].end());
        if(set_row.size() != board[row].size()) return false;
    }
    return true;
}

bool check_column(const array<array<int, 9>, 9> & board){
    v_i v; 

    for(int col = 0; col < board[col].size(); col++){
        // int current = board[0][col];
        for(int i = 0; i < 9 ; i++){
            v.push_back(board[i][col]);
        }
        set<int> set_col(v.begin(), v.end());
        if(set_col.size() != v.size()){
            //duplicates in columns
            cout << "duplicates in col: " << col << endl;
            return false;
        }
        else {v.clear();}
    }

    return true;
}

void make_3x3(v_i &v1, vecpair_iv &v2, const array<array<int,9>, 9> &board, int start, int end, int &count){
    for(int i = 0; i < 9; i++){
        for(int j = start ; j < end; j++){
            // cout << board[i][j];
            v1.push_back(board[i][j]);
        }
        if(i == 2 || i == 5 || i == 8){
            v2.push_back(pair<int,v_i>{count,v1});
            v1.clear(); 
            count++;
        }
        // cout << endl;
    }
}

bool check_3x3(const array<array<int, 9>, 9> & board){
    v_i v1;
    vecpair_iv v2;
    int count = 1;
    bool pass = false;
    for(int i = 0, v = 0; i < 3; i++, v+=3){
        make_3x3(v1, v2, board, v, v+3, count);
    }
    for ( vecpair_iv::const_iterator it = v2.begin(); it != v2.end(); it++){
        cout << "checking 3x3 grid: " << it->first << " ";
        for(auto& vv : it->second){
            if( vv == 0){
                cout << " has zero " << endl;
                return false;
            }
            cout << vv;
        }
        cout << endl;
        set<int> setNum(it->second.begin(), it->second.end());
        if(setNum.size() == v2.size()){
            pass = true;
            cout << " //passed" << endl;
        }
        else{
            pass = false;
            break;
        }
    }

    if(pass) return true;

    // cout << endl << "duplicates in 3x3 grid" << endl;
    return false;
}

bool validate(const array<array<int, 9>, 9>& board) {
    // check_row(board); // if all rows do not have a duplicate
    // check_column(board); // if all columns do not have a duplicate
    // check_3x3(board);
    if((check_row(board) && check_column(board) && check_3x3(board)))
        return true;
    cout << "check failed" << endl;
    return false;
}

int main(){

    array<array<int, 9>, 9> tb3 = 
    {{
        {1,3,2,5,7,9,4,6,8},
        {4,9,8,2,6,1,3,7,5},
        {7,5,6,3,8,4,2,1,9},
        {6,4,3,1,5,8,7,9,2},
        {5,2,1,7,9,3,8,4,6},
        {9,8,7,4,2,6,5,3,1},
        {2,1,4,9,3,5,6,8,7},
        {3,6,5,8,1,7,9,2,4},
        {8,7,9,6,4,2,1,3,5}
     }};

    validate(tb3);
}


