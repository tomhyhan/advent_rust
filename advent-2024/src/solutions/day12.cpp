#include "day.hpp"

typedef pair<int, int> Point_t;
typedef tuple<int, int, char> Seen_t;

const int DIRECTIONS[4][2] = {{0,1},{1,0},{0,-1},{-1,0}};
const int NDIRECTIONS[4][2] = {{-1,-1},{0,-1},{-1,0},{0,0}};

void part1(int row, int n_col, map<Point_t, char> garden);
void part2(int n_row, int n_col, map<Point_t, char> garden);

string Day12::solve(stringstream& input_buffer) {
    map<Point_t, char> garden;
    string line;

    int row = 0;
    int n_col;
    while (getline(input_buffer, line)) {
        for (int col=0; col < line.size(); col++) {
            garden[Point_t({row,col})] = line[col];
        }
        row++;
    }
    n_col = line.size();

    part1(row, n_col, garden);
    part2(row, n_col, garden);

    return "day12";
}

int get_area(Point_t pt, map<Point_t, char>& garden, set<Point_t>& visited) {
    int perimeter = 0;
    int area = 0;
    char curr_ch = garden[pt];

    vector<Point_t> stack;
    stack.push_back(pt);

    while (!stack.empty()) {
        Point_t curr_pt = stack.back();
        stack.pop_back();

        if (visited.contains(curr_pt)) {
            continue;
        }
        visited.insert(curr_pt);

        auto [row, col] = curr_pt;
        ++area;

        int n_cnt = 0;
        for (auto dir: DIRECTIONS) {
            int nrow = row + dir[0];
            int ncol = col + dir[1];
            Point_t next_pt = {nrow, ncol};
            if (garden.contains(next_pt) && garden[next_pt] == curr_ch) {
                stack.push_back(next_pt);
                n_cnt++;
            }
        }
        perimeter += 4 - n_cnt;
    }

    return area * perimeter;
}

void part1(int n_row, int n_col, map<Point_t, char> garden) {
    set<Point_t> visited;

    int total = 0;
    for (int row=0; row < n_row; row++) {
        for (int col=0; col < n_col; col++) {
            Point_t pt = {row, col}; 
            if (visited.contains(pt)) {
                continue;
            }
            total += get_area(pt, garden,visited);
        }
    }
    cout << "Part1: " << total << endl;
}

int area(vector<Point_t> coords) {
    return 0;
}

int get_area_discount(int row, int col, char curr_c, map<Point_t, set<char>>& extended_garden, set<Seen_t>& visited) {
    vector<Point_t> coords;

    vector<tuple<int, int, int>> stack;
    
    stack.push_back(make_tuple(row, col, -1));
    int last_r, last_c;
    cout << curr_c << endl;
    while (!stack.empty()) {
        auto [curr_row, curr_col, curr_dir] = stack.back();
        stack.pop_back();
        last_r = curr_row;
        last_c = curr_col;

        // Seen_t seen = {curr_row, curr_col, curr_c};
        // if (visited.contains(seen)) continue;
        

        // cout << curr_row << " " << curr_col << endl;
        Point_t pt = {curr_row, curr_col};
        for (int i=0; i <  ARRAY_SIZE(DIRECTIONS); i++) {
            int nrow = curr_row + DIRECTIONS[i][0];
            int ncol = curr_col + DIRECTIONS[i][1];
            Point_t next_pt = {nrow, ncol};
            Seen_t next_seen = {nrow, ncol, curr_c};
            if (visited.contains(next_seen)) {
                continue;
            }
            if (extended_garden.contains(next_pt) && extended_garden[next_pt].contains(curr_c)) {
                if (curr_dir != i) {
                    coords.push_back(pt);
                }
                visited.insert(next_seen);
                stack.push_back(make_tuple(nrow, ncol, i));
                break;
            }
        }
    }
    cout << endl;
    // coords.push_back(Point_t(last_r, last_c));
    for (auto co: coords) {
        cout << co.first << " " << co.second << endl;
    }

    return 0;
}

void part2(int n_row, int n_col, map<Point_t, char> garden) {
    map<Point_t, set<char>> extended_garden;

    for (int row=0; row < n_row+1; row++) {
        for (int col=0; col < n_col+1; col++) {
            Point_t pt = {row, col};
            for (auto dir: NDIRECTIONS) {
                int nrow = row + dir[0];
                int ncol = col + dir[1];
                Point_t npt = {nrow, ncol};
                if (garden.contains(npt)) {
                    extended_garden[pt].insert(garden[npt]);
                }
            }
        }
    }
    // for (auto c: extended_garden[Point_t(1,2)]) {
    //     cout << c << endl;
    // }

    set<Seen_t> visited;

    int total = 0;
    for (int row=0; row < n_row+1; row++) {
        for (int col=0; col < n_col+1; col++) {
            Point_t pt = {row, col};
            for (auto c: extended_garden[pt]) {
                Seen_t seen = {row, col, c};
                if (visited.contains(seen)) continue;
                total += get_area_discount(row, col, c, extended_garden, visited);
                // goto DEBUG;

            }
        }
    }
    DEBUG:
    cout << "Part2: " << total << endl;
}

