#include "day.hpp"

class Computer {
    private:
        int pointer = 0;
        int regA, regB, regC;
    public:
        void set_reister(int rA, int rB, int rC) {
            regA = rA;
            regA = rB;
            regA = rC;
        }

        void run_program(vector<int> program) {

        }

        void get_ins(int opcode) {
            switch (opcode) {
                case 0:
                    break;
                case 1:
                    break;
                case 2:
                    break;
                case 3:
                    break;
                case 4:
                    break;
                case 5:
                    break;
                case 6:
                    break;
                case 7:
                    break;
                default:
                    throw std::invalid_argument("Unsupported Opcode");
            } 
        }

        
};

string Day17::solve(stringstream& input_buffer) {
    
    vector<int> regs;
    string line;
    size_t pos;

    while (getline(input_buffer, line)) {
        if (line.empty()) break;
        pos = line.find(':');
        regs.push_back(stoi(line.substr(pos+2)));
    }
    getline(input_buffer, line);
    pos = line.find(':');

    vector<int> program;
    string snum;
    istringstream isline(line);
    while (getline(isline, snum, ',')) {
        program.push_back(stoi(snum));
    }
    part1()
    return "day17";
}