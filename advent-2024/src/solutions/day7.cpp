#include "day.hpp"

void part1(vector<vector<int64>> equations);

const vector<function<int64(int64,int64)>> OPERATERS = {
    [](int64 a, int64 b){return a+b;},
    [](int64 a, int64 b){return a*b;},
    // part 2
    [](int64 a, int64 b){
        return stoll(to_string(a) + to_string(b));
    },
};

string Day7::solve(stringstream& input_buffer) {
    vector<vector<int64>> equations;
    string line;

    while (getline(input_buffer, line)) {
        istringstream isline(line);
        vector<int64> equation;
        
        string test_val;
        getline(isline, test_val, ':');

        equation.push_back(stoll(test_val));
        
        string num;
        while (isline >> num) {
            equation.push_back(stoll(num));
        }
        equations.push_back(equation);
    }
    part1(equations);
    return "day7";
}

void part1(vector<vector<int64>> equations) {
    int64 n_cal_results = 0;
    
    
    for (auto equation: equations) {
        int64 test_val = equation[0];
        vector<set<int64>> DP(equation.size()-1);
        DP[0].insert(equation[1]);
        for (int i=1; i < DP.size(); i++) {
            for (auto n: DP[i-1]) {
                for (auto op: OPERATERS) {
                    DP[i].insert(op(n, equation[i+1]));
                }
            }
        }

        if (DP[DP.size()-1].contains(test_val)) {
            n_cal_results += test_val;
        }
    }
    
    cout << "Part1: " << n_cal_results << endl;
}