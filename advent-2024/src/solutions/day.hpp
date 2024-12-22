#pragma once
#include <iostream>
#include <sstream>
#include <fstream>
#include <regex>
#include <typeinfo>
#include <functional>
#include <chrono>
#include <thread>
#include <limits>
#include <optional>

#include <queue>
#include <deque>
#include <string>
#include <vector>
#include <set>
#include <map>
#include <array>

#include <algorithm>
#include <cmath>
#include <unordered_map>
#include <ranges>
#include <numeric>

#define ARRAY_SIZE(arr) (sizeof(arr) / sizeof(arr[0]))

typedef long long int64;

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

class Day6 : public Day {
    public:
        string solve(stringstream& input_buffer) override;
};

class Day7 : public Day {
    public:
        string solve(stringstream& input_buffer) override;
};

class Day8 : public Day {
    public:
        string solve(stringstream& input_buffer) override;
};

class Day9 : public Day {
    public:
        string solve(stringstream& input_buffer) override;
};

class Day10 : public Day {
    public:
        string solve(stringstream& input_buffer) override;
};

class Day11 : public Day {
    public:
        string solve(stringstream& input_buffer) override;
};

class Day12 : public Day {
    public:
        string solve(stringstream& input_buffer) override;
};

class Day13 : public Day {
    public:
        string solve(stringstream& input_buffer) override;
};

class Day14: public Day {
    public:
        string solve(stringstream& input_buffer) override;
};

class Day15: public Day {
    public:
        string solve(stringstream& input_buffer) override;
};

class Day16: public Day {
    public:
        string solve(stringstream& input_buffer) override;
};

class Day17: public Day {
    public:
        string solve(stringstream& input_buffer) override;
};

class Day18: public Day {
    public:
        string solve(stringstream& input_buffer) override;
};

class Day19: public Day {
    public:
        string solve(stringstream& input_buffer) override;
};

class Day20: public Day {
    public:
        string solve(stringstream& input_buffer) override;
};

class Day21: public Day {
    public:
        string solve(stringstream& input_buffer) override;
};