#include "day.hpp"

class Computer {
    public:
        int64 pointer = 0;
        int64 regA, regB, regC;
        string outs;

        Computer(vector<int64> regs) {
            regA = regs[0];
            regB = regs[1];
            regC = regs[2];
        }

        void reset() {
            pointer = 0;
            regA = 0;
            regB = 0;
            regC = 0;        
            outs = "";
        }

        void run_program(vector<int64> program, bool out_once=false) {
            
            while (pointer < program.size()) {
                optional<int64> output = execute(program[pointer],program[pointer+1]);
                if (output.has_value()) {
                    outs += to_string(*output);
                    if (out_once) {
                        return;
                    }
                }
                pointer += 2;
            }
        }

        optional<int64> execute(int64 opcode, int64 operand) {
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

        int64 combo(int64 operand) {
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
void part1(Computer computer, vector<int64> program);
void part2(Computer computer, vector<int64> program);

string Day17::solve(stringstream& input_buffer) {
    
    vector<int64> regs;
    string line;
    size_t pos;

    while (getline(input_buffer, line)) {
        if (line.empty()) break;
        pos = line.find(':');
        regs.push_back(stoi(line.substr(pos+2)));
    }
    getline(input_buffer, line);
    pos = line.find(':');

    vector<int64> program;
    string snum;
    istringstream isline(line.substr(pos+2));
    while (getline(isline, snum, ',')) {
        program.push_back(stoi(snum));
    }
    Computer computer(regs);
    part1(computer, program);
    part2(computer, program);
    return "day17";
}

void part1(Computer computer, vector<int64> program) {
    computer.run_program(program);
    cout << "Part1: " << computer.outs << endl;
}

// [(5), (46), (369), (2953, 2960)]
void part2(Computer computer, vector<int64> program) {

    vector<int64> reg_values;
    vector<int64> answers;
    reg_values.push_back(5);
    
    for (int64 i=program.size()-1; i >= 0; i--) {
        int64 result = program[i];
        vector<int64> new_reg_values;
        for (auto reg_v: reg_values)  {
            computer.reset();
            computer.regA = reg_v;
            computer.run_program(program, true);
            if (computer.outs == to_string(result)) {
                for (int j = 0; j < 8; j++) {
                    new_reg_values.push_back(reg_v*8+j);
                }
                if (i==0) {
                    answers.push_back(reg_v);
                }
            }
        }
        reg_values = new_reg_values;
    }
    int64 min_reg_v = *min_element(answers.begin(), answers.end());

    computer.reset();
    computer.regA = min_reg_v;
    computer.run_program(program);
    cout << "Part2: " <<  min_reg_v << endl;
    cout << "Part2: " <<  computer.outs << endl;
}
