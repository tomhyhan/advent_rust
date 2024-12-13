#include "day.hpp"

const int MAX_BLINK=75;

void part1(vector<int64> stones);
void part2(vector<int64> stones);

string Day11::solve(stringstream& input_buffer) {
    vector<int64> stones;
    string snum;

    while (input_buffer >> snum) {
        stones.push_back(stoll(snum));
    } 
    
    part1(stones);
    part2(stones);
    return "day11";
}

void apply_rule(int64 num, vector<int64>& stones) {
    if (num == 0) {
        stones.push_back(1);
    } else if ((int)(floor(log10(num)) + 1) % 2 == 0) {
        string snum = to_string(num);
        stones.push_back(stoll(snum.substr(0, snum.size() / 2)));
        stones.push_back(stoll(snum.substr(snum.size() / 2)));

    } else {
        stones.push_back(num*2024);
    }
}

void part1(vector<int64> stones) {
    for (int blink=0; blink < 25; blink++ ) {
        vector<int64> new_stones;
        for (int i=0; i < stones.size(); i++) {
            apply_rule(stones[i], new_stones);
        }
        stones = new_stones;
    }

    cout << "part1: " << stones.size() << endl;
}

int64 count_stones(int64 num, int64 blink, map <pair<int64, int64>, int64>& memo) {
    pair<int64, int64> key = {num, blink};
    auto stone = memo.find(key);
    int64 stone_digits = (int)(floor(log10(num)) + 1);
    
    if (blink == MAX_BLINK) {
        return 1;
    } else if (stone != memo.end()) {
        return stone->second;
    }

    int64 total_stones = 0;
    if (num == 0) {
        total_stones += count_stones(1, blink+1, memo);
    } else if (stone_digits % 2 == 0) {
        // naive
        // string snum = to_string(num);
        // total_stones += count_stones(stoll(snum.substr(0, snum.size() / 2)), blink+1, memo);
        // total_stones += count_stones(stoll(snum.substr(snum.size() / 2)), blink+1, memo);
        
        // better!
        total_stones += count_stones(num / pow(10, stone_digits/2), blink+1, memo);
        total_stones += count_stones(num % (int64)pow(10, stone_digits/2), blink+1, memo);
    } else {
        total_stones += count_stones(num*2024, blink+1, memo);
    }

    memo[key] = total_stones;
    return total_stones;
}

void part2(vector<int64> stones) {

    int64 total_stones = 0;
    map <pair<int64, int64>, int64> memo;

    for (int i=0; i < stones.size(); i++) {
        total_stones += count_stones(stones[i], 0, memo);
    }
    cout << "part2: " << total_stones << endl;
}