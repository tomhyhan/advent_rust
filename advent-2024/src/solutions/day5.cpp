#include "day.hpp"
// 75,47,61,53,29
string Day5::solve(stringstream& input_buffer) {
    map<int, vector<int>> graph;
    vector<vector<int>> pages;

    string line;
    bool create_map = true;
    while (getline(input_buffer, line)) {
        istringstream isline(line);
        string num;
        if (line.empty()) {
            create_map = false;
            continue;
        }

        if (create_map) {
            getline(isline, num, '|');
            int key = stoi(num);
            getline(isline, num, '|');
            int value = stoi(num);
            graph[key].push_back(value);
        } else {
            vector<int> page; 
            while (getline(isline, num, ',')) {
                page.push_back(stoi(num));
            }
            pages.push_back(page);
        }
    }
    cout << pages.size() << endl;

    return "day5";
}

void Day5::part1() {}
void Day5::part2() {}