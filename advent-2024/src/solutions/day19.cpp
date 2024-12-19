#include "day.hpp"

void part1(vector<string> designs, vector<string> towels);
void part2(vector<string> designs, vector<string> towels);

string Day19::solve(stringstream& input_buffer) {
    regex pattern(R"(\w+)");
    smatch matches;

    vector<string> towels;
    
    string line;
    getline(input_buffer, line);
    
    while (regex_search(line, matches, pattern)) {
        towels.push_back(matches[0]);
        line = matches.suffix();
    }

    vector<string> designs;

    while (getline(input_buffer, line)) {
        if (line.empty()) continue;
        designs.push_back(line);
    }

    part1(designs, towels);
    part2(designs, towels);
    return "day19";
}

bool is_possible(string design, vector<string>& towels, map<string, bool>& memo) {
    if (design.empty()) {
        return true;
    } else if (memo.contains(design)) {
        return memo[design];
    }

    bool possible = false;
    for (auto towel: towels) {
        int n = towel.size();
        if (design.substr(0,n) == towel) {
            possible = possible || is_possible(design.substr(n), towels, memo);
        }
    }

    memo[design] = possible;
    return possible;
}

int64 is_possible_pt2(string design, vector<string>& towels, map<string, int64>& memo) {
    if (design.empty()) {
        return 1;
    } else if (memo.contains(design)) {
        return memo[design];
    }

    int64 possible = 0;
    for (auto towel: towels) {
        int n = towel.size();
        if (design.substr(0,n) == towel) {
            possible += is_possible_pt2(design.substr(n), towels, memo);
        }
    }

    memo[design] = possible;
    return possible;
}

void part1(vector<string> designs, vector<string> towels) {
    int n_possibles = 0;
    map<string, bool> memo;
    for (auto design: designs) {
        n_possibles += is_possible(design, towels, memo);
    }

    cout << "Part1: " << n_possibles << endl;
}

void part2(vector<string> designs, vector<string> towels) {
    int64 n_possibles = 0;
    map<string, int64> memo;
    for (auto design: designs) {
        n_possibles += is_possible_pt2(design, towels, memo);
    }

    cout << "Part2: " << n_possibles << endl;
}