#include <array>
#include <iostream>
using std::array;
using std::cout;
using std::endl;

bool check_row(const array<array<int, 9>, 9> & board){
    // for(const auto& row: board){
    //     int prev = row[0];
    //     for(const auto& i: row){
    //         if(row[i] != i){
    //             prev = row[i];
    //         }
    //         else cout << "duplicate" << i << endl;
    //     }
    //     cout << endl;
    // }
    for(int row = 0; row < board.size(); row++){
        int current = board[row][0];
        for(int i = 0; i < board[row].size(); i++){
            if( i == 0 && board[row][i] == current) continue;
            if(board[row][i] != current ) current = board[row][i];
            else {
                cout << "duplicate on board[row]: " << row << endl;
                cout << "duplicate on board[row][i]: " << i << endl;
                return false; 
            }
        }
        cout << "row: " << row << " //passed" << endl;
    }
    return true;
}
bool check_column(const array<array<int, 9>, 9> & board){

    for(int col = 0; col < board[col].size(); col++){
        int current = board[0][col];
        for(int i = 0; i < 9 ; i++){
            if(i == 0 && board[i][col] == current) continue;
            if(board[i][col] != current) current = board[i][col];
            else{
                cout << "duplicate on board[i][col]: " << col << endl;
                cout << "duplicate on board[i]: " << i << endl;
                return false;
            }
        }
    }

    return true;
}
bool check_3x3(const array<array<int, 9>, 9> & board){

    //loop by outer column 
    // for(int k = 0, col_pos = 3; k < col_pos; k+=3, col_pos+=3)    {
    //     // loop by row (row) first 3 rows
    // }
    for(int i = 0; i < 3 ; i++){
        //loop by column, first 3 cols
        for (int j = 0; j < 3; j++ ){
            cout<< board[i][j];
        }
        cout << endl;
    }

    for(int i = 3; i < 6 ; i++){
        //loop by column, first 3 cols
        for (int j = 3; j < 6; j++ ){
            cout<< board[i][j];
        }
        cout << endl;
    }

    return true;
}

bool validate(const array<array<int, 9>, 9>& board) {
    // check_row(board); // if all rows do not have a duplicate
    // check_column(board); // if all columns do not have a duplicate
    check_3x3(board);
    return false;
}

int main(){
    array<array<int, 9>, 9> tb2 = 
    {{
         {1,1,1,2,2,2,7,8,9},
         {1,1,1,2,2,2,7,8,9},
         {1,1,1,2,2,2,3,3,3},
         {4,4,4,4,4,4,4,4,4},
         {5,5,5,5,5,5,5,5,5},
         {6,6,6,6,6,6,6,6,6},
         {7,7,7,7,7,7,7,7,7},
         {8,8,8,8,8,8,8,8,8},
         {9,9,9,9,9,9,9,9,9}
     }};

    validate(tb2);

}


