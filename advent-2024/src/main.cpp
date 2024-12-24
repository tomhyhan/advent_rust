#include <iostream>
#include <fstream>
#include <string>
#include <map>
#include <functional>
#include <sstream>

#include "solutions/day.hpp"

using namespace std;

class AOCSolver {
    private:
        map<string, function<string(stringstream&)>> solutions;
    public:
        AOCSolver() {
            solutions["day1"] = [](stringstream& file) {
                Day1 day1solver;
                return day1solver.solve(file);
            };
            solutions["day2"] = [](stringstream& file) {
                Day2 day2solver;
                return day2solver.solve(file);
            };
            solutions["day3"] = [](stringstream& file) {
                Day3 day3solver;
                return day3solver.solve(file);
            };
            solutions["day4"] = [](stringstream& file) {
                Day4 day4solver;
                return day4solver.solve(file);
            };
            solutions["day5"] = [](stringstream& file) {
                Day5 day5solver;
                return day5solver.solve(file);
            };
            solutions["day6"] = [](stringstream& file) {
                Day6 day6solver;
                return day6solver.solve(file);
            };
            solutions["day7"] = [](stringstream& file) {
                Day7 day7solver;
                return day7solver.solve(file);
            };
            solutions["day8"] = [](stringstream& file) {
                Day8 day8solver;
                return day8solver.solve(file);
            };
            solutions["day9"] = [](stringstream& file) {
                Day9 day9solver;
                return day9solver.solve(file);
            };
            solutions["day10"] = [](stringstream& file) {
                Day10 day10solver;
                return day10solver.solve(file);
            };
            solutions["day11"] = [](stringstream& file) {
                Day11 day11solver;
                return day11solver.solve(file);
            };
            solutions["day12"] = [](stringstream& file) {
                Day12 day12solver;
                return day12solver.solve(file);
            };
            solutions["day13"] = [](stringstream& file) {
                Day13 day13solver;
                return day13solver.solve(file);
            };
            solutions["day14"] = [](stringstream& file) {
                Day14 day14solver;
                return day14solver.solve(file);
            };
            solutions["day15"] = [](stringstream& file) {
                Day15 day15solver;
                return day15solver.solve(file);
            };
            solutions["day16"] = [](stringstream& file) {
                Day16 day16solver;
                return day16solver.solve(file);
            };
            solutions["day17"] = [](stringstream& file) {
                Day17 day17solver;
                return day17solver.solve(file);
            };
            solutions["day18"] = [](stringstream& file) {
                Day18 day18solver;
                return day18solver.solve(file);
            };
            solutions["day19"] = [](stringstream& file) {
                Day19 day19solver;
                return day19solver.solve(file);
            };
            solutions["day20"] = [](stringstream& file) {
                Day20 day20solver;
                return day20solver.solve(file);
            };
            solutions["day21"] = [](stringstream& file) {
                Day21 day21solver;
                return day21solver.solve(file);
            };
            solutions["day22"] = [](stringstream& file) {
                Day22 day22solver;
                return day22solver.solve(file);
            };
            solutions["day23"] = [](stringstream& file) {
                Day23 day23solver;
                return day23solver.solve(file);
            };
        };

        bool solve(const string& day) {
            
            string input_file = "input/" + day + ".txt";

            ifstream file(input_file);

            if (!file.is_open()) {
                cout << "cannot open a file " << input_file << "\n";
                return false;
            };

            stringstream buffer;
            buffer << file.rdbuf();
            file.close();

            auto iter = solutions.find(day);
            if (iter == solutions.end()) {
                cout << "No Solution found for " << day << "\n";
                return false;
            };

            iter -> second(buffer);
            cout << "Solution For " << day  << "\n";

            return true;
        };
};

int main(int argc, char* argv[]) {
    if (argc < 2) {
        std::cout << "Please Provide day\n";
        return 1;
    }

    string day = argv[1];
    AOCSolver aocsolver;

    if (!aocsolver.solve(day)){
        return 1;
    }
    
    return 0;
}