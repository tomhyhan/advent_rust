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