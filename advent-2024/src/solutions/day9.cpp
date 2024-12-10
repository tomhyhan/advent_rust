#include "day.hpp"

void part1(vector<int> disk);

string Day9::solve(stringstream& input_buffer) {

    string input = input_buffer.str();
    vector<int> disk;
    // 0 2 4 
    cout << '9' - '0'<< endl;
    for (int i=0; i < input.size(); i++) {
        vector<int> vec;
        if (i % 2 == 0) {
            vec.assign(input[i] - '0', i / 2);
        } else {
            vec.assign(input[i] - '0', -1);
        }
        disk.insert(disk.end(), vec.begin(), vec.end());
    }
    // vector<char> disk_vec(disk.begin(), disk.end());

    // cout << disk << endl;

    // for (int x : disk) {
    //     cout << x << " ";
    // }
    cout << endl;

    part1(disk);
    return "day9";
}

void part1(vector<int> disk) {
    int left = 0;
    int right = disk.size() - 1;
    int64 total=0;
    while (true) {
        while (disk[left] != -1) left++;
        while (disk[right] == -1) right--;
        if (left >= right) break;
        disk[left] = disk[right];
        disk[right] = -1;
    }
    
    for (int i=0; i < disk.size(); i++) {
        if (disk[i] == -1) break;
        total += i * disk[i];
    }
    // 6331928801055 X
    cout << total << endl;
}

void part2() {
    // 00...111...2...333.44.5555.6666.777.888899

}