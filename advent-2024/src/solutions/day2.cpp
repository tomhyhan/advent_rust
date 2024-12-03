#include "day.hpp"
#include <optional>

string Day2::solve(stringstream& input_buffer) {

    part1(input_buffer);
    input_buffer.clear(); 
    input_buffer.seekg(0);
    part2(input_buffer);

    return "day2";
}

void Day2::part1(stringstream& input_buffer) {
    string line;
    int valid_reports = 0;
    while (getline(input_buffer, line)) {
        istringstream issline(line);

        int level;

        optional<int> prev;
        optional<int> curr;

        bool isincreasing = false;

        bool valid_report = true;
        while (issline >> level) {
            if (!prev.has_value()) {
                prev = level;
                continue;
            }
            if (!curr.has_value()) {
                curr = level;
                isincreasing = prev.value() < curr.value()? true : false;
            } 

            curr = level;
            int diff = abs(prev.value() - curr.value());
            if (diff < 1 || diff > 3 || (isincreasing != prev.value() < curr.value())) {
                valid_report = false;
                break;
            }
            prev = curr;
        }

        if (valid_report) {
            valid_reports += 1;
        }
    }
    cout << "Part1: " << "valid_reports: "<< valid_reports << "\n";
}

void Day2::part2(stringstream& input_buffer) {
    string line;
    int valid_reports = 0;
    while (getline(input_buffer, line)) {
        istringstream issline(line);

        int level;

        optional<int> prev;
        optional<int> curr;

        bool isincreasing = false;

        bool tolerate = true;
        bool is_valid_report = true;
        while (issline >> level) {
            if (!prev.has_value()) {
                prev = level;
                continue;
            }
            if (!curr.has_value()) {
                curr = level;
                isincreasing = prev.value() < curr.value()? true : false;
            } 

            curr = level;
            int diff = abs(prev.value() - curr.value());
            if (diff < 1 || diff > 3 || (isincreasing != prev.value() < curr.value())) {
                if (tolerate) {
                    tolerate = false;
                    curr.reset();
                    continue;
                };
                is_valid_report = false;
                break;
            }
            prev = curr;
        }

        if (is_valid_report) {
            valid_reports += 1;
        }
    }
    // 511 X

    cout << "Part2: " << "valid_reports: "<< valid_reports << "\n";
}