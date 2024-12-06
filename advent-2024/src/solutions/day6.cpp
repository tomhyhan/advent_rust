#include "day.hpp"

typedef pair<int, int> Point_t;
typedef tuple<int, int, int> Position_t;

const int DIRECTIONS[4][2] = {{-1,0},{0,1},{1,0},{0,-1}};

set<Point_t> part1(map<Point_t, char>& layout, Position_t start) {
    set<Point_t> visited;

    vector<Position_t> stack;
    stack.push_back(start);

    while (!stack.empty()) {
        auto [row, col, dir] = stack.back();
        stack.pop_back();

        Point_t curr_pt = Point_t{row, col};
        
        visited.insert(curr_pt);

        int nrow = row + DIRECTIONS[dir][0];
        int ncol = col + DIRECTIONS[dir][1];
        Point_t next_pt = Point_t{nrow, ncol};
        auto next_layout = layout.find(next_pt);
        if (next_layout == layout.end()) {
            break;
        }
        if (next_layout->second == '#') {
            dir = (dir + 1) % 4;
            nrow = row + DIRECTIONS[dir][0];
            ncol = col + DIRECTIONS[dir][1];
        }

        stack.push_back(Position_t{nrow, ncol, dir});
    }
    cout << "Part1: " << visited.size() << endl;
    return visited;
}

bool is_loop(map<Point_t, char>& layout, Position_t start) {
    set<Position_t> visited;

    vector<Position_t> stack;
    stack.push_back(start);

    while (!stack.empty()) {
        Position_t curr_pos = stack.back();
        stack.pop_back();

        auto [row, col, dir] = curr_pos;

        if (visited.contains(curr_pos)) {
            return true;
        }
        visited.insert(curr_pos);

        int nrow = row + DIRECTIONS[dir][0];
        int ncol = col + DIRECTIONS[dir][1];

        Point_t next_pt = Point_t{nrow, ncol};
        auto next_layout = layout.find(next_pt);
        if (next_layout == layout.end()) {
            return false;
        }
        if (next_layout->second == '#') {
            dir = (dir + 1) % 4;
            nrow = row;
            ncol = col;
        }

        stack.push_back(Position_t{nrow, ncol, dir});
    }
    cout << "??" << endl;
}

void part2(map<Point_t, char>& layout, Position_t start, set<Point_t> paths) {
    int n_total_loops = 0;
    for (auto& pair : paths) {
        if (pair.first == get<0>(start) && pair.second == get<1>(start)) continue;
        layout[pair] = '#';
        n_total_loops += is_loop(layout, start);
        layout[pair] = '.';
    }
    // 1563 X 
    cout << "Part2: " << n_total_loops << endl;
}


string Day6::solve(stringstream& input_buffer) {
    map<Point_t, char> layout;
    Position_t start;
    string line;
    int i = 0;

    while (input_buffer >> line) {
        for (int col=0; col < line.size(); col++) {
            if (line[col] == '^') {
                start = {i, col, 0};
                layout[Point_t {i, col}] = '.';
                continue;
            }
            layout[Point_t {i, col}] = line[col];
        }
        i++;
    }
    set<Point_t> visited = part1(layout, start);
    part2(layout, start, visited);
    return "day6";
}




