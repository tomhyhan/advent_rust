#include "day.hpp"

typedef pair<int, int> Point_t;

function<int(char)> ctoi = [](char a) {return a - '0';};

const int DIRECTIONS[4][2] = {{0,1},{0,-1},{1,0},{-1,0}};

void part1(vector<vector<int>> grid);


string Day10::solve(stringstream& input_buffer) {
    vector<vector<int>> grid;
    string line;

    while (getline(input_buffer, line)) {
        vector<int> nums;
        for (auto c: line) {
            nums.push_back(ctoi(c));
        }
        grid.push_back(nums);
    }

    part1(grid);
    
    return "day10";
}

int count_routes(int row, int col, vector<vector<int>> grid, map<Point_t, int> scores) {
    Point_t key = {row,col};
    auto score = scores.find(key);

    if (score != scores.end()) {
        return score->second;
    } else if (grid[row][col] == 9) {
        return 1;
    }

    int total_scores = 0;

    for (auto dir: DIRECTIONS) {
        int nrow = row + dir[0];
        int ncol = col + dir[1];
        if (0 <= nrow && nrow < grid.size() && 0 <= ncol && ncol < grid[0].size() && grid[row][col] + 1 == grid[nrow][ncol]) {
            int s = count_routes(nrow, ncol, grid, scores);
            if (s > 0) total_scores++;
        }
    }

    scores[key] = total_scores;
    return total_scores;
}

void part1(vector<vector<int>> grid) {

    int total_score = 0;
    map<Point_t, int> scores;

    for (int row=0; row < grid.size(); row++) {
        for (int col=0; col < grid[0].size(); col++) {
            if (grid[row][col] == 0) {
                total_score += count_routes(row, col, grid, scores);
            }
        }
    }

    cout << "part1: " << total_score << endl;
}