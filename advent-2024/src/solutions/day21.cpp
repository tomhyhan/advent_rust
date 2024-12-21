#include "day.hpp"

using point_t = pair<int, int>;
const map<char, point_t> numeric_pad = {
    {'7', point_t(0,0)},{'8', point_t(0,1)},{'9', point_t(0,2)},
    {'4', point_t(1,0)},{'5', point_t(1,1)},{'6', point_t(1,2)},
    {'1', point_t(2,0)},{'2', point_t(2,1)},{'3', point_t(2,2)},
                        {'0', point_t(3,1)},{'A', point_t(3,2)},
};
const map<char, point_t> dir_pad = {
                        {'^', point_t(0,1)},{'A', point_t(0,2)},
    {'<', point_t(1,0)},{'v', point_t(1,1)},{'>', point_t(1,2)}
};

void part1(vector<string> codes);

string Day21::solve(stringstream& input_buffer) {
    vector<string> codes;
    string code;
    while (getline(input_buffer, code)) {
        codes.push_back(code);
    }

    part1(codes);

    return "day21";
}

void part1(vector<string> codes) {

}

/* 
+---+---+---+
| 7 | 8 | 9 |
+---+---+---+
| 4 | 5 | 6 |
+---+---+---+
| 1 | 2 | 3 |
+---+---+---+
    | 0 | A |
    +---+---+

   +---+---+
    | ^ | A |
+---+---+---+
| < | v | > |
+---+---+---+

   +---+---+
    | ^ | A |
+---+---+---+
| < | v | > |
+---+---+---+

*/