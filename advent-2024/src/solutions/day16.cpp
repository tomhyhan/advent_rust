// with backtracking
#include "day.hpp"

typedef pair<int, int> point_t;
typedef tuple<int,int,int> key_t;
const int DIRECTIONS[4][2] = {{-1,0},{0,1},{1,0},{0,-1}};

template<typename K, typename V>
class DefaultDict : public std::map<K, V> {
    V default_val;
public:
    DefaultDict(V default_value = V{}) : default_val(default_value) {}
    
    V& operator[](const K& key) {
        if (this->count(key) == 0) {
            this->std::map<K, V>::operator[](key) = default_val;
        }
        return this->std::map<K, V>::operator[](key);
    }
};

void part1_simple(point_t start, point_t end, vector<string> grid);
void part2_simple(point_t start, point_t end, vector<string> grid);
void part2_astar(point_t start, point_t end, vector<string> grid);

string Day16::solve(stringstream& input_buffer) {
    vector<string> grid;
    string line;

    int row = 0;
    point_t start, end;
    while (getline(input_buffer, line)) {
        for (int col=0; col < line.size(); col++){
            if (line[col] == 'S') {
                start = point_t(row, col);
                line[col] = '.';
            } else if (line[col] == 'E') {
                end = point_t(row, col);
                line[col] = '.';
            }
        }
        grid.push_back(line);
        row++;
    }
    part1_simple(start, end, grid);
    part2_simple(start, end, grid);
    return "day16";
}

void part1_simple(point_t start, point_t end, vector<string> grid) {
    DefaultDict<key_t, int> visited(numeric_limits<int>::max());

    auto [srow, scol] = start;
    auto [erow, ecol] = end;
    int sdir = 1;

    priority_queue<tuple<int, int, int, int>, vector<tuple<int, int, int, int>>, greater<tuple<int, int, int, int>>> queue;
    
    queue.push(make_tuple(0, srow, scol, sdir));

    int min_score = numeric_limits<int>::max();
    while (!queue.empty()) {
        auto [score, row, col, dir] = queue.top();
        queue.pop();

        if (row==erow && col==ecol) {
            cout << "Part 1: " << score << endl;
            break;
        }

        for (int ndir = 0; ndir < ARRAY_SIZE(DIRECTIONS); ndir++) {
            int nrow = row + DIRECTIONS[ndir][0];
            int ncol = col + DIRECTIONS[ndir][1];
            key_t key = make_tuple(nrow, ncol, ndir);
            int nscore = dir == ndir? score + 1:score + 1 + 1000; 
            if (grid[nrow][ncol] != '#' && nscore < visited[key]) {
                visited[key] = nscore;
                queue.push(make_tuple(nscore, nrow, ncol, ndir));
            }
        }
    }
}



void part2_simple(point_t start, point_t end, const vector<string> grid) {

}
