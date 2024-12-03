#include "day.hpp"

// mul\(\d{1,3},\d{1,3}\)|(don't\(\))|(do\(\))
string Day3::solve(stringstream& input_buffer) {
    part1(input_buffer);
    part2(input_buffer);

    return "day3";
}

void Day3::part1(stringstream& input_buffer) {
    string line = input_buffer.str();
    regex pattern(R"(mul\((\d{1,3}),(\d{1,3})\))");
    smatch matches;

    int total = 0;
    while (regex_search(line, matches, pattern)) {
        int f = stoi(matches[1]);
        int s = stoi(matches[2]);
        total += f*s;
        line = matches.suffix();
    }
    
    cout << "Part 1: " << total << '\n';
}

void Day3::part2(stringstream& input_buffer) {
    string line = input_buffer.str();
    regex pattern(R"(mul\((\d{1,3}),(\d{1,3})\)|(don't\(\))|(do\(\)))");
    smatch matches;

    int total = 0;
    bool enabled = true;
    while (regex_search(line, matches, pattern)) {
        if (matches[3].matched) {
            enabled = false; 
        } else if (matches[4].matched) {
            enabled = true; 
        }

        if (enabled && matches[1].matched && matches[2].matched ) {
            int f = stoi(matches[1]);
            int s = stoi(matches[2]);
            total += f*s;
        }        

        line = matches.suffix();
    }
    
    cout << "Part 2: " << total << '\n';
}