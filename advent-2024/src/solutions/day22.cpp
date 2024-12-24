#include "day.hpp"

const map<char, function<int64(int64,int64)>> OPERATORS = {
    {'*', [](int64 a, int64 b) {return a*b;}},
    {'/', [](int64 a, int64 b) {return a/b;}},
};

namespace {
    using seq_t = tuple<int64,int64,int64,int64>;
    
    void part1(vector<int64> numbers);  
    void part2(vector<int64> numbers);  
}

string Day22::solve(stringstream& input_buffer) {
    vector<int64> numbers;
    string line;
    while (getline(input_buffer, line)) {
        numbers.push_back(stoll(line));
    }

    part1(numbers);
    part2(numbers);
    return "day22";
}

namespace {
    int64 mix_and_prune(int64 secret, int64 scale, char op) {
        return (secret ^ OPERATORS.at(op)(secret, scale)) % 16777216; 
    }

    int64 get_next_secret(int64 secret) {
        int64 result = mix_and_prune(mix_and_prune(mix_and_prune(secret, 64, '*'), 32, '/'), 2048, '*');
        return result; 
    }

    void part1(vector<int64> numbers) {
        int64 total = 0;
        for (auto num: numbers) {
            int64 secret = num;
            for (int i=0; i < 2000; i++) {
                secret = get_next_secret(secret);
            }
            total += secret;
        }
        cout << "Part1: " << total << endl;
    }

    void part2(vector<int64> numbers) {
        map<seq_t, vector<int64>> bananas;
        for (auto num: numbers) {
            map<seq_t, int64> curr_bananas;
            int64 secret = num;
            int64 prev_digit = secret % 10;
            deque<int64> queue;
            for (int i=0; i < 2000; i++) {
                secret = get_next_secret(secret);
                int64 digit = secret % 10;
                int64 diff = digit - prev_digit;
                queue.push_back(diff);
                if (queue.size() == 4) {
                    seq_t seq(queue[0],queue[1],queue[2],queue[3]);
                    
                    if (!curr_bananas.contains(seq)) {
                        curr_bananas[seq] = digit;
                    }
                    queue.pop_front();

                }
                prev_digit = digit;
            }

            for (auto [seq, n_banana]: curr_bananas) {
                bananas[seq].push_back(n_banana);
            }
        }
        
        int64 total = 0;
        for (auto [seq, banana]: bananas) {
            total = max(total, accumulate(banana.begin(), banana.end(), 0LL));
        }

        cout << "Part2: " << total << endl;
    }
}
