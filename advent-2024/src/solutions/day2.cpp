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

bool is_safe_report(int prev, int curr, bool isincreasing) {
    int diff = abs(prev - curr);
    if (diff < 1 || diff > 3 || (isincreasing != prev < curr)) {
        return false;
    }
    return true;
}

bool produce_safe_report(vector<int> report) {
    bool isincreasing = report[0] < report[1];
    for (int i=1; i < report.size(); i++) {
        int prev = i-1;
        int curr = i;
        if (!is_safe_report(report[prev], report[curr], isincreasing)) {
            return false;
        }
    }
    return true;
} 

void Day2::part2(stringstream& input_buffer) {
    vector<vector<int>> reports;

    string line;
    while (getline(input_buffer, line)) {
        istringstream issline(line);
        vector<int> report;

        int num;
        while (issline >> num) {
            report.push_back(num);
        }

        reports.push_back(report);
    }

    int n_safe_reports = 0;
    for (auto report: reports) {
        bool found_valid = false;
        for (int k=0; k < 2; k++) {
            reverse(report.begin(), report.end());
            int prev = 0, curr = 1;

            bool isincreasing = report[prev] < report[curr];
            bool safe = true;
            for (int i=1; i < report.size(); i++) {
                prev = i-1;
                curr = i;
                if (!is_safe_report(report[prev], report[curr], isincreasing)) {
                    safe = false;
                    break;
                }
            }

            if (safe) {
                found_valid= true;
                break;
            }

            for ( auto skip: {prev, curr}) {
                vector<int> skip_report;
                for (int i=0; i < report.size(); i++) {
                    if (i==skip) continue;
                    skip_report.push_back(report[i]);
                }
                if (produce_safe_report(skip_report)) {
                    found_valid= true;
                    break;
                }
            }
        }
        if (found_valid) {
            n_safe_reports += 1;
        }
    }


    cout << "report size: " << n_safe_reports << "\n"; 
}