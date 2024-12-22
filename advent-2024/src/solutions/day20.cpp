#include "day.hpp"

using point_t = pair<int, int>;
const int DIRECTIONS[4][2] = {{0,1},{0,-1},{1,0},{-1,0}};


void part1(map<point_t, char> grid, point_t start, point_t end);
void part2(map<point_t, char> grid, point_t start, point_t end);

string Day20::solve(stringstream& input_buffer) {
    map<point_t, char> grid;
    
    string line;
    point_t start, end;

    int row = 0;
    while (getline(input_buffer, line)) {
        for (int col=0; col < line.size(); col++ ) {
            if (line[col] == 'S') {
                start = point_t(row,col);
                grid[point_t(row,col)] = '.';
            } else if (line[col] == 'E') {
                end = point_t(row,col);
                grid[point_t(row,col)] = '.';
            } else {
                grid[point_t(row,col)] = line[col];
            }
        }
        row++;
    }
    part1(grid, start, end);
    part2(grid, start, end);
    return "day20";
}

void fill_distances(map<point_t, int>& distances, map<point_t, char> grid, point_t start, point_t end) {
    auto [srow, scol] = start;
    auto [erow, ecol] = end;

    vector<tuple<int, int, int>> stack;
    stack.push_back(make_tuple(0, srow, scol));

    while (!stack.empty()) {
        auto [distance, row, col] = stack.back();
        stack.pop_back();

        point_t curr(row, col);
        if (distances.contains(curr)) continue;
        distances[curr] = distance;

        if (row == erow && col == ecol) {
            break;
        }
        
        for (auto dir: DIRECTIONS) {
            int nrow = row + dir[0];
            int ncol = col + dir[1];
            point_t next(nrow, ncol);
            if (grid[next] == '.') {
                stack.push_back(make_tuple(distance + 1, nrow, ncol));
            }
        }
    }
}

void part1(map<point_t, char> grid, point_t start, point_t end) {
    map<point_t, int> distances;
    fill_distances(distances, grid, start, end);

    map<int, int> counter;
    int least100 = 0;
    for (const auto& [pt, curr_d]: distances) {
        auto [row, col] = pt;

        for (auto dir: DIRECTIONS) {
            int nrow = row + dir[0];
            int ncol = col + dir[1];
            int nnrow = nrow + dir[0];
            int nncol = ncol + dir[1];
            point_t npt(nrow, ncol);
            point_t nnpt(nnrow, nncol);
            if (grid.contains(nnpt) && grid[npt] == '#' && grid[nnpt] == '.') {
                if (distances[nnpt] > curr_d) {
                    int saved =  distances[nnpt] - curr_d - 2;
                    counter[saved]++;
                    if (saved >= 100) {
                        least100++;
                    }
                }
            }
        }
    }

    cout << "Part1: " << least100 << endl;
}

void part2(map<point_t, char> grid, point_t start, point_t end) {
    map<point_t, int> distances;
    fill_distances(distances, grid, start, end);
    int least100 = 0;
    map<int, int> counter;
    for (const auto& [pt, curr_d]: distances) {
        auto [row, col] = pt;

        set<point_t> visited;
        using QType = tuple<int, int, int>;
        priority_queue<QType, vector<QType>, greater<QType>> queue;

        queue.push(make_tuple(0,row,col));
        while (!queue.empty()) {
            auto [distance, curr_r, curr_c] = queue.top();
            queue.pop();

            if (distance > 20) continue;

            point_t curr_pt(curr_r, curr_c);
            if (visited.contains(curr_pt)) {
                continue;
            }
            visited.insert(curr_pt);

            if (curr_pt != pt && grid[curr_pt] == '.') {
                if (distances[curr_pt] > curr_d) {
                    int saved =  distances[curr_pt] - curr_d - distance;
                    if (saved >= 100) {
                        least100++;
                        counter[saved]++;
                    }
                }
            }

            for (auto dir: DIRECTIONS) {
                int nrow = curr_r + dir[0];
                int ncol = curr_c + dir[1];
                point_t npt(nrow, ncol);
                if (grid.contains(npt)) {
                    queue.push(make_tuple(distance+1, nrow, ncol));
                }
            }
        }
    }
    cout << "Part2: " << least100 << endl;
}
