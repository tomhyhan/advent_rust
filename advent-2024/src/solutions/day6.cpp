#include "day.hpp"

string Day6::solve(stringstream& input_buffer) {
    vector<string> map;
    tuple<size_t, size_t> start;
    string line;
    int i = 0;

    while (input_buffer >> line) {
        
        size_t pos = line.find('^');
        if (pos != string::npos) {
            line[pos] = '.';
            start = {i, pos};
            cout << line << endl;
            cout << i << pos << endl;
        }

        map.push_back(line);
        
        i++;
    }
    part1(map, start);
    return "day6";
}

void Day6::part1(vector<string> map, tuple<size_t, size_t> start) {
    const size_t n_row = map.size();
    const size_t n_col = map[0].size();
    bool visited[n_row][n_col] = false;

}

void Day6::part2(vector<string> map, tuple<size_t, size_t> start) {

}

