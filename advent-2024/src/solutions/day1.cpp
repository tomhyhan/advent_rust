#include "day.hpp"

string Day1::solve(stringstream& input_buffer) {
    int result = 0;

    vector<int> num_list1;
    vector<int> num_list2;
    int num1, num2;
    while (input_buffer >> num1 >> num2) {
        num_list1.push_back(num1);
        num_list2.push_back(num2);
    }

    part1(num_list1, num_list2);
    part2(num_list1, num_list2);

    return "day1";
}

void Day1::part1(vector<int> num_list1, vector<int> num_list2) {
    sort(num_list1.begin(), num_list1.end());
    sort(num_list2.begin(), num_list2.end());

    int result = 0;
    for (int i=0; i < num_list1.size(); i++) {
        result += abs(num_list1[i] - num_list2[i]);
    };

    cout << "part1: " << result << "\n";
}

void Day1::part2(vector<int> num_list1, vector<int> num_list2) {
    unordered_map<int, int> num_counts;

    for (const int num : num_list2) {
        num_counts[num]++;
    }
    
    int result = 0;
    for (const int num : num_list1) {
        result += num * num_counts[num];
    }

    cout << "Part2: " << result << "\n";
}