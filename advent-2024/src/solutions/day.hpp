#pragma once
#include <iostream>
#include <sstream>
#include <fstream>

#include <string>
#include <vector>

#include <algorithm>
#include <unordered_map>
#include <ranges>

using namespace std;

class Day {
    public:
        virtual string solve(stringstream& input_buffer) = 0; 
};

class Day1 : public Day {
    public:
        string solve(stringstream& input_buffer) override;
    protected:
        void part1(vector<int>, vector<int>);
        void part2(vector<int>, vector<int>);
};

class Day2 : public Day {
    public:
        string solve(stringstream& input_buffer) override;
    protected:
        void part1(stringstream& input_buffer);
        void part2(stringstream& input_buffer);
};