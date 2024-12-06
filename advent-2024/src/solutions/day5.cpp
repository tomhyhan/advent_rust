#include "day.hpp"

string Day5::solve(stringstream& input_buffer) {
    map<int, set<int>> graph;
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
            graph[key].insert(value);
        } else {
            vector<int> page; 
            while (getline(isline, num, ',')) {
                page.push_back(stoi(num));
            }
            pages.push_back(page);
        }
    }

    part1(graph, pages);
    part2(graph, pages);

    return "day5";
}

bool is_correct_order(const vector<int>& page, map<int, int>& degree, map<int, set<int>>& graph) {
    for (const int num: page) {
        if (degree[num] !=0) {
            return false;
        }
        for (auto n: graph[num]) {
            degree[n]--;
        }
    } 
    return true;
}
void Day5::part1(map<int, set<int>> graph,vector<vector<int>> pages) {
    // 1. define degree
    // 2. insert previous num to set, and check if the curr num has previous
    int mid_page_num = 0;

    // method 2
    // for (const auto& page: pages) {
    //     set<int> prevs;
    //     bool does_contain = false;
    //     for (const int num: page) {
    //         for (auto n: graph[num]) {
    //             if (prevs.contains(n)) {
    //                 does_contain = true;
    //                 break;
    //             }
    //         }
    //         if (does_contain) {
    //             break;
    //         }
    //         prevs.insert(num);
    //     }
    //     if (!does_contain) {
    //         mid_page_num += page[page.size()/2];
    //     }
    // }

    // method 1
    for (const auto& page: pages) {
        map<int, int> degree;
        for (const int num: page) {
            for (auto n: graph[num]) {
                degree[n]++;
            }
        }
    
        if (is_correct_order(page, degree, graph)) {
            mid_page_num += page[page.size()/2];        
        }
    }

    cout << "Part1: " << mid_page_num << endl;

}

int find_first(vector<int>& order_prevs, set<int>& values) {
    for (int i=0; i < order_prevs.size(); i++) {
        if (values.contains(order_prevs[i])) {
            return i;
        }
    }
    out_of_range("Cannot reach here!");
}

void Day5::part2(map<int, set<int>> graph,vector<vector<int>> pages) {
    int mid_page_num = 0;

    for (const auto& page: pages) {
        map<int, int> degree;
        map<int, int> rev_degree;
        for (const int num: page) {
            degree[num] = 0;
        }
        for (const int num: page) {
            for (auto n: graph[num]) {
                if (find(page.begin(), page.end(), n) != page.end()) {
                    degree[n]++;
                }
            }
        }
        for (auto [k, v]: degree) {
            rev_degree[v] = k;
        }
        if (!is_correct_order(page, degree, graph)) {
            mid_page_num += rev_degree[page.size()/2];
        }
    }

    
    cout << "Part2: " << mid_page_num << endl;
}