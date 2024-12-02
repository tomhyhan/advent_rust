#pragma once
#include <string>
#include <fstream>
#include <vector>

using namespace std;

class Day {
    public:
        virtual string solve(stringstream& input_buffer) = 0; 
};

class Day1 : public Day{
    public:
        string solve(stringstream& input_buffer) override;
    protected:
        void part1(vector<int>, vector<int>);
        void part2(vector<int>, vector<int>);
};