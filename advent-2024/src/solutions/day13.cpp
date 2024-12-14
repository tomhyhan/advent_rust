#include "day.hpp"

class Machine {
    public:
        long double x1, x2, y1, y2, c1, c2;
        Machine(vector<long double> info) {
            x1 = info[0];
            x2 = info[1];
            y1 = info[2];
            y2 = info[3];
            c1 = info[4] + 10000000000000;
            c2 = info[5] + 10000000000000;
        }

        void print() {
            cout << x1 << "x1" << " + " << y1 << "y1" << " = " << c1 << endl; 
            cout << x2 << "x2" << " + " << y2 << "y2" << " = " << c2 << endl; 
        }
        
        long double apply_cramer() {
            long double det = x1*y2 - x2*y1;
            long double area_A = c1*y2 - c2*y1;
            long double area_B = x1*c2 - x2*c1;
            long double A = (area_A) / det;
            long double B = (area_B) / det;
            if ((int64)A == A && (int64)B == B) {
                // cout << A << " " << B << endl;
                return A *3 +  B;
            }
            return 0;
        }
};

void part1(vector<Machine> machines);

string Day13::solve(stringstream& input_buffer) {
    regex btn_pattern(R"(Button .: X\+(\d+), Y\+(\d+))");
    regex p_pattern(R"(Prize: X=(\d+), Y=(\d+))");
    
    string line;
    vector<long double> info;
    vector<Machine> machines;

    while (getline(input_buffer,line)) {

        smatch matches;
        if (regex_search(line, matches, btn_pattern)) {
            info.push_back(stold(matches[1]));
            info.push_back(stold(matches[2]));
        } else if (regex_search(line, matches, p_pattern)) {
            info.push_back(stold(matches[1]));
            info.push_back(stold(matches[2]));
        }

        if (line.empty()) {
            Machine machine(info);
            machines.push_back(machine);
            info.clear();
        }
    }
        

    part1(machines);
    return "day13";
}

void part1(vector<Machine> machines) {
    long double total = 0;

    for (auto machine: machines) {
        total += machine.apply_cramer();
    }
    cout << "Part1: " << (int64)total << endl;
}
