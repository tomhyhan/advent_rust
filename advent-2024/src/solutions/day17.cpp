#include "day.hpp"


class Computer {
    public:
        int pointer = 0;
        int regA, regB, regC;
        
        Computer(vector<int> regs) {
            regA = regs[0];
            regB = regs[1];
            regC = regs[2];
        }

        void run_program(vector<int> program) {
            string outs;
            while (pointer < program.size()) {
                optional<int> output = execute(program[pointer],program[pointer+1]);

                if (output.has_value()) {
                    // cout << *output << endl;
                    outs+= to_string(*output) + ',';
                }
                pointer += 2;
            }

            cout << outs << endl;
        }

        optional<int> execute(int opcode, int operand) {
            switch (opcode) {
                case 0:
                    regA = regA / pow(2, combo(operand));
                    break;
                case 1:
                    regB ^=  operand;
                    break;
                case 2:
                    regB = combo(operand) % 8;
                    break;
                case 3:
                    if (regA != 0) {
                        pointer = operand - 2;
                    }
                    break;
                case 4:
                    regB ^= regC;
                    break;
                case 5:
                    return combo(operand) % 8;
                case 6:
                    regB = regA / pow(2, combo(operand));
                    break;
                case 7:
                    regC = regA / pow(2, combo(operand));
                    break;
                default:
                    throw std::invalid_argument("Unsupported Opcode");
            }
            return nullopt; 
        }

        int combo(int operand) {
            switch (operand) {
                case 0:
                case 1:
                case 2:
                case 3:
                    return operand;
                case 4:
                    return regA;
                case 5:
                    return regB;
                case 6:
                    return regC;
                case 7:
                default:
                    throw invalid_argument("Unsupported Operand");
            }
        }
};
void part1(Computer computer, vector<int> program);


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
    istringstream isline(line.substr(pos+2));
    while (getline(isline, snum, ',')) {
        program.push_back(stoi(snum));
    }
    Computer computer(regs);
    part1(computer, program);
    return "day17";
}

void part1(Computer computer, vector<int> program) {
    computer.run_program(program);
    cout << computer.regB << endl;

}

