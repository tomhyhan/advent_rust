#include "day.hpp"


namespace {
    void part1(map<string, vector<string>> networks);
    void part2(map<string, vector<string>> networks);
}


string Day23::solve(stringstream& input_buffer) {
    map<string, vector<string>> networks;
    string line;

    while (getline(input_buffer, line)) {
        istringstream isline(line);
        string key, value; 
        getline(isline, key, '-');
        getline(isline, value, '-');
        networks[key].push_back(value);
        networks[value].push_back(key);
    }

    // part1(networks);
    part2(networks);
    cout << networks.size() << endl;
    return "day23";
}

namespace {
    void part1(map<string, vector<string>> networks) {
        set<set<string>> found;
        for (auto [start_str, nstrs]: networks) {
            set<set<string>> seen_inside;
            vector<tuple<string, int, set<string>>> stack;
            set<string> s = {start_str};
            stack.push_back(make_tuple(start_str, 0, s));
            while (!stack.empty()) {
                auto [curr, distance, pair] = stack.back();
                stack.pop_back();

                for (auto next: networks[curr]) {
                    if (distance == 2 ) {
                        if (start_str == next) {
                            found.insert(pair);
                        }
                        continue;
                    }
                    set<string> conn = {next, curr};
                    if (seen_inside.contains(conn)) continue;
                    seen_inside.insert(conn);
                    set<string> new_pair = pair;
                    new_pair.insert(next);
                    stack.push_back(make_tuple(next, distance+1, new_pair));
                }
            }
            // break;
        }
        int total = 0;
        for (auto p: found) {
            for (auto t: p) {
                if (t[0] == 't') {
                    total++;
                    break;
                }
            }
        }
        cout << "Part1: " << total << endl;
    }

    void part2(map<string, vector<string>> networks) {
        vector<set<string>> set_networks;
        for (auto [k, v]: networks) {
            set<string> network;
            network.insert(k);
            for (auto s: v) {
                network.insert(s);
            }
            set_networks.push_back(network);
        }
        
        set<set<string>> test;
        for (auto p1: set_networks) {
            for (auto p2: set_networks) {
                if (p1 == p2) {
                    continue;
                }
                set<string> result;
                set_intersection(p1.begin(), p1.end(),p2.begin(), p2.end(), inserter(result, result.begin()));

                if (result.size() == 3) {
                    test.insert(result);
                }
            }
        }
        // too lazy to debug ;o -> python
        cout << test.size() << endl;
    }
}

// aq yn vc cg wq 
// cg de tb yn aq
// co ka ta de tc
// de cg co ta ka
// ka co tb ta de
// kh tc qp ub ta
// qp kh ub td wh
// ta co ka de kh
// tb cg ka wq vc
// tc kh wh td co
// td tc wh qp yn
// ub qp kh wq vc
// vc aq ub wq tb
// wh tc td yn qp
// wq tb ub aq vc 
// yn aq cg wh td