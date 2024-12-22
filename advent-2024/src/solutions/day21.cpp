#include "day.hpp"

using point_t = pair<int64, int64>;
using stod_t = pair<char, char>;

const map<point_t, char> NUMERIC_PAD = {
    {point_t(0,0), '7'},{point_t(0,1), '8'},{point_t(0,2), '9'},
    {point_t(1,0), '4'},{point_t(1,1), '5'},{point_t(1,2), '6'},
    {point_t(2,0), '1'},{point_t(2,1), '2'},{point_t(2,2), '3'},
                        {point_t(3,1), '0'},{point_t(3,2), 'A'},
};
const map<point_t, char> DIR_PAD = {
                        {point_t(0,1),'^'},{point_t(0,2), 'A'},
    {point_t(1,0),'<'},{point_t(1,1), 'v'},{point_t(1,2), '>'}
};
const int64 DIRECTIONS[4][2] = {{0,1},{0,-1},{1,0},{-1,0}};
const char CDIRECTIONS[4] = {'>','<','v','^'};

void part1(vector<string> codes);
void part2(vector<string> codes);

string Day21::solve(stringstream& input_buffer) {
    vector<string> codes;
    string code;
    while (getline(input_buffer, code)) {
        codes.push_back(code);
    }

    // part1(codes);
    part2(codes);

    return "day21";
}


void find_optimal_path(char ch, point_t pt, map<point_t, char> pad, map<stod_t, vector<string>>& paths) {
    auto [srow, scol] = pt;

    using Qtype = tuple<int64, int64, string>;
    map<point_t, int64> visited;

    deque<Qtype> queue;
    queue.push_back(make_tuple(srow, scol, ""));

    while (!queue.empty()) {
        auto [row, col, path] = queue.front();
        queue.pop_front();
        // cout << row << col << path << endl;
        point_t curr_pt(row, col);
        if (visited.contains(curr_pt) && path.size() > visited[curr_pt]) continue;
        visited[curr_pt] = path.size();

        paths[stod_t(ch, pad[point_t(row, col)])].push_back(path + 'A');

        for (int64 i=0; i <  ARRAY_SIZE(DIRECTIONS); i++) {
            int64 nrow = row + DIRECTIONS[i][0];
            int64 ncol = col + DIRECTIONS[i][1];
            char cdir = CDIRECTIONS[i];
            point_t next_pt(nrow, ncol);
            if (pad.contains(next_pt)) {
                queue.push_back(make_tuple(nrow, ncol, path+cdir));
            }
        }


    }
}


void fill_paths(map<stod_t, vector<string>>& num_paths, map<stod_t, vector<string>>& dir_paths) {
    for (auto [pt, ch]: NUMERIC_PAD) {
        find_optimal_path(ch, pt, NUMERIC_PAD, num_paths);
    }

    for (auto [pt, ch]: DIR_PAD) {
        find_optimal_path(ch, pt, DIR_PAD, dir_paths);
    }
}

vector<string> create_all_path(string code, map<stod_t, vector<string>> paths, char prev) {
    if (code.empty()) {
        return vector<string>(1, "");
    } 
    char curr = code[0];
    vector<string> prev_paths = create_all_path(code.substr(1), paths, curr);

    vector<string> new_paths;
    for (auto path: paths[stod_t(prev, curr)]) {
        for (auto prev_path: prev_paths) {
            new_paths.push_back(path + prev_path);
        }
    }

    return new_paths;
}


void part1(vector<string> codes) {
    map<stod_t, vector<string>> num_paths;
    map<stod_t, vector<string>> dir_paths;

    fill_paths(num_paths, dir_paths);

    int64 total = 0;
    for (auto code: codes) {
        int64 i_part = stoi(code.substr(0,3));
        char prev = 'A';
        vector<string> num_codes;
        num_codes = create_all_path(code, num_paths, 'A');

        vector<string> robot_codes;
        for (auto nc: num_codes) {
            vector<string> new_codes = create_all_path(nc, dir_paths, 'A');
            robot_codes.insert(robot_codes.end(), new_codes.begin(), new_codes.end());
        }

        vector<string> human_codes;
        for (auto rc: robot_codes) {
            vector<string> new_codes = create_all_path(rc, dir_paths, 'A');
            human_codes.insert(human_codes.end(), new_codes.begin(), new_codes.end());
        }

        auto min_code = *min_element(human_codes.begin(), human_codes.end(), [](const string& a, const string& b){ return a.length() < b.length();});

        cout << min_code << endl;
        cout << i_part << endl;
        cout << min_code.size() << endl;
        total += min_code.size() * i_part;
    }
    cout << "Part1: " << total << endl;
}


string find_min_by_len(vector<string> codes) {
    return *min_element(codes.begin(), codes.end(), [](const string& a, const string& b){ return a.length() < b.length();});
}

void fill_shotest_path_by_rate_of_chage(map<stod_t, vector<string>> dir_paths, map<stod_t, string>& optimal_dir_paths) {

    for (auto [pt1, ch1]: DIR_PAD) {
        for (auto [pt2, ch2]: DIR_PAD) {
            int min_path_len = numeric_limits<int>::max();
            string min_path;
            for (auto path: dir_paths[stod_t(ch1, ch2)]) {
                // v<<A <v<A
                string curr_path = "";
                char prev = 'A';
                vector<string> all_paths = create_all_path(path, dir_paths, prev);
                // cout << ch1 << " " << ch2 << " "<< path << endl;
                // for (auto p: all_paths) {
                //     cout << p << endl;
                // }
                // cout << endl;
                string curr_min_path = find_min_by_len(all_paths);
                if (curr_min_path.size() < min_path_len) {
                    min_path_len = curr_min_path.size();
                    min_path = path;
                }
            }
            optimal_dir_paths[stod_t(ch1, ch2)] = min_path;
        }
    }
}

// <vA<AA>>^A
// v<<A
// <

// <vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A
// v<<A>>^A<A>AvA<^AA>A<vAAA>^A
// <A^A>^^AvvvA

int64 create_all_path(int depth, char source, char dest, map<stod_t, vector<string>> paths, map<tuple<int, char, char>, int64>& memo) {
    tuple<int, char, char> key = make_tuple(depth, source, dest);
    if (depth >= 26) {
        return 1;
    } else if (memo.contains(key)) {
        return memo[key];
    }

    int64 min_new_path = numeric_limits<int64>::max();
    for (auto path: paths[stod_t(source, dest)]) {
        int64 new_path = 0; 
        char prev = 'A';
        for (auto c: path) {
            new_path += create_all_path(depth+1, prev, c, paths, memo);
            prev = c;
        }
        min_new_path = min(min_new_path, new_path);
    }

    memo[key] = min_new_path; 
    return min_new_path;
}

void part2(vector<string> codes) {
    map<stod_t, vector<string>> num_paths;
    map<stod_t, vector<string>> dir_paths;
    map<stod_t, string> optimal_dir_paths;

    fill_paths(num_paths, dir_paths);
    // fill_shotest_path_by_rate_of_chage(dir_paths, optimal_dir_paths);
    // cout << optimal_dir_paths[stod_t('A', '<')] << endl;

    int64 total = 0;
    map<tuple<int, char, char>, int64> memo;

    for (auto code: codes) {
        int64 i_part = stoi(code.substr(0,3));
        vector<string> num_codes;
        num_codes = create_all_path(code, num_paths, 'A');
        int64 min_len = numeric_limits<int64>::max();

        for (auto num_code: num_codes) {
            char prev = 'A';
            int64 new_code = 0;
            for (auto curr: num_code) {
                new_code += create_all_path(0, prev, curr, dir_paths, memo);
                prev = curr; 
            }
            min_len = min(min_len, new_code);
        }

        cout << i_part << endl;
        cout << min_len << endl;
        total += min_len * i_part;
    }

    // 704172293042864
    cout << "Part2: " << total << endl;
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