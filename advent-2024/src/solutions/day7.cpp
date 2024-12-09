#include "day.hpp"

void part1(map<long long, vector<long long>> equations);

const vector<function<long long(long long,long long)>> OPERATERS = {
    [](long long a, long long b){return a+b;},
    [](long long a, long long b){return a*b;}
};

string Day7::solve(stringstream& input_buffer) {
    map<long long, vector<long long>> equations;
    string line;
    
    while (getline(input_buffer, line)) {
        istringstream isline(line);
        string test_val;
        getline(isline, test_val, ':');

        vector<long long> nums;
        string num;
        while (isline >> num) {
            nums.push_back(stoll(num));
        }
        equations[stoll(test_val)] = nums;
    }
    part1(equations);
    return "day7";
}

void part1(map<long long, vector<long long>> equations) {
    long long n_cal_results = 0;
    
    for (auto pair: equations) {
        vector<long long>& nums = pair.second;
        vector<set<long long>> DP(nums.size());
        DP[0].insert(nums[0]);
        for (int i=1; i < nums.size(); i++) {
            for (auto n: DP[i-1]) {
                for (auto op: OPERATERS) {
                    DP[i].insert(op(nums[i], n));
                }
            }
        }

        if (DP[DP.size()-1].contains(pair.first)) {
            n_cal_results += pair.first;
        }
        // cout << pair.first << " " << pair.second[0] << endl;
    }
    // 42283209481122 X
    
    cout << "Part1: " << n_cal_results << endl;
}