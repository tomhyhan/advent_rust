#include "day.hpp"

typedef pair<int, int> Point_t;

const map<char, Point_t> DIRECTIONS = {
    {'<', Point_t({0,-1})},
    {'^', Point_t({-1,0})},
    {'>', Point_t({0,1})},
    {'v', Point_t({1,0})},
};


void part1(Point_t start, vector<string> grid, string moves);

string Day15::solve(stringstream& input_buffer) {
    string line;
    string moves;
    vector<string> grid;
    Point_t start;

    int row = 0;
    while (getline(input_buffer, line)) {
        for (int col=0; col < line.size(); col++) {
            if (line[col] == '@') {
                start = {row, col};
            }
        }        

        if (line.empty()) {
            break;
        } 

        grid.push_back(line);
        row++;
    }

    getline(input_buffer, moves, '\0');
    part1(start, grid, moves);
    return "day15";
}

void part1(Point_t start, vector<string> grid, string moves) {
    auto [row,col] = start;

    for (auto move: moves) {
        if (move == '\n') continue;
        vector<Point_t> stack;
        auto [mrow, mcol] = DIRECTIONS.at(move); 
        int curr_row = row;
        int curr_col = col;
        while (grid[curr_row][curr_col] != '#' && grid[curr_row][curr_col] != '.') {
            stack.push_back(Point_t(curr_row, curr_col));
            curr_row += mrow;
            curr_col += mcol;
        }

        if (grid[curr_row][curr_col] == '#') {
            continue;
        }

        while (!stack.empty()) {
            auto [prev_row, prev_col] = stack.back();
            stack.pop_back();

            grid[curr_row][curr_col] = grid[prev_row][prev_col];
            curr_row = prev_row;
            curr_col = prev_col;
        }
        grid[curr_row][curr_col] = '.';
        row += mrow;
        col += mcol;
    }

    // for (auto s: grid) {
    //     cout << s << endl;
    // }
    int total = 0;
    for (int i = 0; i < grid.size(); i++) {
        for (int j = 0; j < grid[0].size(); j++) {
            if (grid[i][j] == 'O') {
                total += i*100 + j;
            }
        }
    }
    // 1472588 X
    cout << "Part 1: " << total << endl;
}