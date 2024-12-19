#include "day.hpp"


// const int GRID_SIZE = 7;
const int BYTES = 1024;
const int GRID_SIZE = 71;
const int DIRECTIONS[4][2] = {{0,1},{0,-1},{1,0},{-1,0}};
using point_t = pair<int, int> ;


void part1(vector<string> grid, vector<point_t> pts);
void part2(vector<string> grid, vector<point_t> pts);

string Day18::solve(stringstream& input_buffer) {
    vector<string> grid(GRID_SIZE, string(GRID_SIZE, '.'));
    vector<point_t> pts;
    string line;
    while(getline(input_buffer, line)) {
        int pos = line.find(',');
        int x = stoi(line.substr(0,pos));
        int y = stoi(line.substr(pos+1));
        point_t pt(y,x);
        pts.push_back(pt);
    }

    part1(grid, pts);
    part2(grid, pts);

    return "day18";
}

optional<int> djikstra (vector<string> grid) {
    int max_num = numeric_limits<int>::max();
    using QueueType = tuple<int, int, int>;
    priority_queue<QueueType, vector<QueueType>, greater<QueueType>> queue;
    
    vector<vector<int>> visited(GRID_SIZE, vector<int>(GRID_SIZE, max_num));
    queue.push(make_tuple(0,0,0));

    while (!queue.empty()) {
        auto [distance, row, col] = queue.top();
        queue.pop();

        if (distance >= visited[row][col]) {
            continue;
        }
        visited[row][col] = distance;
        
        if (row == GRID_SIZE -1 && col == GRID_SIZE - 1) {
            return distance;
        }

        for (auto dir: DIRECTIONS) {
            int nrow = row + dir[0];
            int ncol = col + dir[1];
            if (nrow >= 0 && nrow < GRID_SIZE && ncol >= 0 && ncol < GRID_SIZE && grid[nrow][ncol] != '#') {
                queue.push(make_tuple(distance + 1, nrow, ncol));
            }
        }
    }
    return nullopt;
}

void part1(vector<string> grid, vector<point_t> pts) {
    for (int i=0; i < BYTES; i++) {
        point_t pt = pts[i];
        auto [r,c] = pt;
        grid[r][c]  = '#';
    }

    optional<int> out = djikstra(grid);
    cout << "Part1: " <<  *out << endl; 
}

void part2(vector<string> grid, vector<point_t> pts) {
    for (int i=0; i < BYTES; i++) {
        point_t pt = pts[i];
        auto [r,c] = pt;
        grid[r][c]  = '#';
    }

    for (int i=BYTES; i < pts.size(); i++) {
        point_t pt = pts[i];
        auto [r,c] = pt;
        grid[r][c]  = '#';

        optional<int> out = djikstra(grid);
        if (!out.has_value()) {
            cout << "Part2: " <<  c << "," << r << endl; 
            break;
        }
    }
}
