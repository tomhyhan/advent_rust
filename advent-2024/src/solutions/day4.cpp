#include "day.hpp"

const string XMAS = "XMAS";

string Day4::solve(stringstream& input_buffer) {
    vector<string> grid;

    string line;
    while (input_buffer >> line) {
        grid.push_back(line);
    }
    part1(grid);
    return "day4";
}

int find_xmas(int row, int col, vector<string>& grid) {
    int n_xmas = 0;
    set<pair<int, int>> visited;
    
    
    vector<tuple<int,int,int,int,int>> stack;
    for (int i=-1; i < 2; i++) {
        for (int j=-1; j < 2; j++) {
            int nrow = row + i;
            int ncol = col + j;
            
            if (nrow == row && ncol == col) continue;
            if (nrow >=0 && nrow < grid.size() && ncol >= 0 && ncol < grid[0].size() && grid[nrow][ncol] == XMAS[1]) {
                stack.push_back({1,nrow,ncol,i,j});
            }
        }
    } 

    while (!stack.empty()) {
        auto [idx, row, col, rmov, cmov] = stack.back();
        stack.pop_back();
        
        if (idx == 3 && grid[row][col] == XMAS[idx]) {
            n_xmas += 1;
            continue;
        }

        int nidx = idx + 1;
        int nrow = row + rmov;
        int ncol = col + cmov;
        if (nrow >=0 && nrow < grid.size() && ncol >= 0 && ncol < grid[0].size() && grid[nrow][ncol] == XMAS[nidx]) {
            stack.push_back({nidx,nrow,ncol,rmov,cmov});
        }
    }
    
    return n_xmas;
}

void Day4::part1(vector<string>& grid) {
    
    int n_xmas = 0;
    for (int row=0; row < grid.size(); ++row) {
        for (int col=0; col < grid[0].size(); ++col) {
            if (grid[row][col] == 'X') {
                n_xmas += find_xmas(row, col, grid);
            }
        }
    }
    
    cout << "Part1: " << n_xmas << "\n";
}

void Day4::part2(stringstream& input_buffer) {

}
// if (visited.contains({row, col}) || idx >= 3) {
//     continue;
// }
// visited.insert({row, col});
