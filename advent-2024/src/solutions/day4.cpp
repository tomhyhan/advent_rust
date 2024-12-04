#include "day.hpp"

const string XMAS = "XMAS";
const string X_MAS = "MAS";

const int DIRECTIONS[4][2] = {{0,2},{2,0},{0,-2},{-2,0}};
const int MOVES[4][2][2][2] = {{{{1,1},{1,-1}},{{-1,1},{-1,-1}}}, {{{1,1},{-1,1}},{{1,-1},{-1,-1}}},{{{1,-1},{1,1}},{{-1,-1},{-1,1}}},{{{-1,1},{1,1}},{{-1,-1},{1,-1}}}};

string Day4::solve(stringstream& input_buffer) {
    vector<string> grid;

    string line;
    while (input_buffer >> line) {
        grid.push_back(line);
    }
    part1(grid);
    part2(grid);
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

bool check_condition(int row, int col, int idx, vector<string>& grid) {
    if (row >=0 && row < grid.size() && col >= 0 && col < grid[0].size() && grid[row][col] == X_MAS[idx]) {
        return true;
    }
    return false;
}

// 1. instant match using brute force
// 2. sperately create and match diagnol "MAS" pairs
void Day4::part2(vector<string>& grid) {

    int n_xmas = 0;
    for (int row=0; row < grid.size(); ++row) {
        for (int col=0; col < grid[0].size(); ++col) {
            if (grid[row][col] == 'M') {
                for (int i=0; i < ARRAY_SIZE(DIRECTIONS); i++) {
                    auto [rmov, cmov] = DIRECTIONS[i];
                    int crow = row;
                    int ccol = col;
                    int nrow = crow + rmov;
                    int ncol = ccol + cmov;
 
                    for (int j=0; j < 2; j++) {
                        const auto [first_pair, second_pair] = MOVES[i][j];
                        const auto [drmov, dcmov] = first_pair;
                        const auto [dnrmov, dncmov] = second_pair;
 
                        for (int idx = 0; idx < 3; idx++) {
                            int r = crow + drmov * idx;
                            int c = ccol + dcmov * idx;
                            int nr = nrow + dnrmov * idx;
                            int nc = ncol + dncmov * idx;
                            if (!(check_condition(r, c, idx, grid) && check_condition(nr, nc, idx, grid))) {
                                break;
                            }
                            if (idx == 2) {
                                n_xmas += 1;
                            }
                        }
                    }                    
                }
            }
        }
    }
    // 1096 X
    cout << "Part2: " << (n_xmas / 2) << "\n";
}
