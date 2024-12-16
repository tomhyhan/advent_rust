#include "day.hpp"

typedef pair<int, int> Point_t;

const map<char, Point_t> DIRECTIONS = {
    {'<', Point_t({0,-1})},
    {'^', Point_t({-1,0})},
    {'>', Point_t({0,1})},
    {'v', Point_t({1,0})},
};


void part1(Point_t start, vector<string> grid, string moves);
void part2(Point_t start, vector<string> grid, string moves);

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
    part2(start, grid, moves);
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

vector<string> extend_grid(vector<string> grid) {
    vector<string> egrid;

    for (auto line: grid) {
        string nline;
        for (int i=0; i < line.size(); i++) {
            if (line[i] == 'O') {
                nline.push_back('[');
                nline.push_back(']');
            } else {
                nline.push_back(line[i]);
                if (line[i] != '@') {
                    nline.push_back(line[i]);
                } else {
                    nline.push_back('.');
                }
            }
        }
        egrid.push_back(nline);
    }
    return egrid;
}

bool recursive_stack(int mrow, int mcol, set<Point_t>pts, vector<string>& egrid) {

    if (pts.empty()) {
        return true;
    }
    set<Point_t> new_pts;

    for (auto pt: pts) {
        auto [row,col] = pt;
        if (egrid[row][col] == '@' || egrid[row][col] == '[' || egrid[row][col] == ']') {
            int nrow = row + mrow;
            int ncol = col + mcol;
            if (egrid[nrow][ncol] == '#') {
                return false;
            }
            if (egrid[nrow][ncol] == ']' ||egrid[nrow][ncol] == '[') {
                Point_t new_pt(nrow, ncol);
                new_pts.insert(new_pt);
                if (egrid[row][col] != egrid[nrow][ncol]) {
                    Point_t new_pt_next;
                    if (egrid[nrow][ncol] == ']') {
                        new_pt_next = {row+mrow, col+mcol-1};
                    } else {
                        new_pt_next = {row+mrow, col+mcol+1};
                    }
                    new_pts.insert(new_pt_next);
                } 
            }

        } 
    }
    
    bool can_move = recursive_stack(mrow, mcol, new_pts, egrid);
    if (can_move) {
        for (auto pt: pts) {
            auto [row,col] = pt;
            egrid[row+mrow][col+mcol] = egrid[row][col];
            egrid[row][col] = '.';
        }
    }

    return can_move;
}

void part2(Point_t start, vector<string> grid, string moves) {
    vector<string> egrid = extend_grid(grid);

    auto [row,col] = start;
    col *= 2;

    for (auto move: moves) {
        if (move == '\n') continue;
        
        // horizontal
        int curr_row = row;
        int curr_col = col;
        auto [mrow, mcol] = DIRECTIONS.at(move); 
        
        if (move == '>' || move == '<') {
            
            vector<Point_t> stack;
            while (egrid[curr_row][curr_col] != '#' && egrid[curr_row][curr_col] != '.') {
                stack.push_back(Point_t(curr_row, curr_col));
                curr_row += mrow;
                curr_col += mcol;
            }

            if (egrid[curr_row][curr_col] == '#') {
                continue;
            }

            while (!stack.empty()) {
                auto [prev_row, prev_col] = stack.back();
                stack.pop_back();

                egrid[curr_row][curr_col] = egrid[prev_row][prev_col];
                curr_row = prev_row;
                curr_col = prev_col;
            }
            egrid[curr_row][curr_col] = '.';
            row += mrow;
            col += mcol;
        } else {
            // vertical
            Point_t pt(curr_row, curr_col); 
            set<Point_t> list;
            list.insert(pt);

            bool can_move = recursive_stack(mrow, mcol, list, egrid);
            if (can_move) {
                row += mrow;
                col += mcol;
            }
        }

    }
    for (auto s: egrid) {
        cout << s << endl;
    }

    int total = 0;
    for (int i = 0; i < egrid.size(); i++) {
        for (int j = 0; j < egrid[0].size(); j++) {
            if (egrid[i][j] == '[') {
                total += i*100 + j;
            }
        }
    }
    // 1452690 X
    cout << "Part2 : " << total << endl;
}