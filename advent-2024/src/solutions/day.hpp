#pragma once
#include <iostream>
#include <sstream>
#include <fstream>
#include <regex>

#include <string>
#include <vector>
#include <set>
#include <map>

#include <algorithm>
#include <unordered_map>
#include <ranges>

#define ARRAY_SIZE(arr) (sizeof(arr) / sizeof(arr[0]))

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

class Day3 : public Day {
    public:
        string solve(stringstream& input_buffer) override;
    protected:
        void part1(stringstream& input_buffer);
        void part2(stringstream& input_buffer);
};
class Day4 : public Day {
    public:
        string solve(stringstream& input_buffer) override;
    protected:
        void part1(vector<string>& grid);
        void part2(vector<string>& grid);
};

class Day5 : public Day {
    public:
        string solve(stringstream& input_buffer) override;
    protected:
        void part1(map<int, set<int>> graph,vector<vector<int>> pages);
        void part2(map<int, set<int>> graph,vector<vector<int>> pages);
};