#include "day.hpp"

typedef pair<int, int> Point_t;


class Machine {
    public:
        Point_t buttonA;
        Point_t buttonB;
        Point_t prize;
        Machine(vector<Point_t> info) {
            buttonA = info[0];
            buttonB = info[1];
            prize = info[2];
        }
};

void part1(vector<Machine> machines);

string Day13::solve(stringstream& input_buffer) {
    regex btn_pattern(R"(Button .: X\+(\d+), Y\+(\d+))");
    regex p_pattern(R"(Prize: X=(\d+), Y=(\d+))");
    
    string line;
    vector<Point_t> info;
    vector<Machine> machines;

    while (getline(input_buffer,line)) {

        smatch matches;
        if (regex_search(line, matches, btn_pattern)) {
            info.push_back(make_pair(stoi(matches[1]), stoi(matches[2])));
        } else if (regex_search(line, matches, p_pattern)) {
            info.push_back(make_pair(stoi(matches[1]), stoi(matches[2])));
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

void get_tokens(vector<pair<int, array<int,2>>>& DP, int x1, int x2, int prize) {
    int bi = 0;
    for (auto button: {x1,x2}) {
        for (int i=0; i < prize + 1; i++) {
            if (i >= button ) {
                int curr = DP[i-button].first + 1;
                if (curr < DP[i].first) {
                    DP[i].first = curr;
                    DP[i].second = DP[i-button].second;
                    DP[i].second[bi]++;
                }
            }
        }
        bi++;
    }
}

void part1(vector<Machine> machines) {
    int total = 0;
    for (auto machine: machines) {
        cout <<"asdf" << endl;
        int prize_x = machine.prize.first;
        int x2 = machine.buttonA.first;
        int x1 = machine.buttonB.first;

        int max_num = 99999;

        int prize_y = machine.prize.second;
        int y2 = machine.buttonA.second;
        int y1 = machine.buttonB.second;
        
        vector<pair<int, array<int,2>>> DPX(prize_x+1, make_pair(max_num, array<int,2>{0,0}));
        DPX[0].first = 0;
        
        vector<pair<int, array<int,2>>> DPY(prize_x+1, make_pair(max_num, array<int,2>{0,0}));
        DPY[0].first = 0;

        get_tokens(DPX, x1, x2, prize_x);
        get_tokens(DPY, y1, y2, prize_y);

        if (DPX[prize_x].first != max_num && DPY[prize_y].first != max_num) {
            total += DPX[prize_x].second[0] + DPX[prize_x].second[1] * 3; 
            cout <<  DPX[prize_x].second[0] + DPX[prize_x].second[1] * 3 << endl; 
        }
    }
    cout << "Part1: " << total << endl;
}
