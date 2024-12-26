#include "day.hpp"

const int MAX_ROW = 7;
const int MAX_COL = 5;

string Day25::solve(stringstream& input_buffer) {
    string line;
    vector<vector<int>> locks;
    vector<vector<int>> keys;
    int row = 0;
    bool is_key = false;
    vector<int> grid(5, 0);
    
    while (getline(input_buffer, line)) {
        if (line.empty()) {
            if (is_key) {
                keys.push_back(grid);
            } else {
                locks.push_back(grid);
            }
            row=0;
            fill(grid.begin(), grid.end(), 0);
            continue;
        }

        for (int col=0; col < line.size(); col++) {
            if (line[col]== '#') grid[col]++;
        }

        if ( row == 0) {
            is_key = accumulate(grid.begin(), grid.end(), 0) == MAX_COL? false: true;
        }
        row++;
    }

    int total = 0;
    for (auto lock: locks) {
        for (auto key: keys) {
            bool is_match = true;
            for (int i=0; i < MAX_COL; i++) {
                if (lock[i] + key[i] > MAX_ROW) {
                    is_match = false;
                    break;
                }
            }
            if (is_match) total++;
        }
    }

    cout << "Part1: " << total << endl;
    return "day25";
}