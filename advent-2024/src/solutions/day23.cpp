#include "day.hpp"


namespace {
    void part1(map<string, vector<string>> networks);
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

    part1(networks);

    return "day23";
}

namespace {
    void part1(map<string, vector<string>> networks) {
        set<string> seen;
        int total = 0;
        for (auto [k, v]: networks) {
            // if (seen.contains(k)) continue;
            // v.push_back(k);
            // int nt = 0;
            cout << k << " ";
            for (auto s: v) {
                // seen.insert(s);
                // if (s[0] == 't') nt++;
                cout << s << " ";
            }
            cout << endl;
            
            // int n = v.size();
            // int r = 3;
            // int nott = n - nt;
            // total = tgamma(n+1) / (tgamma(r+1) * tgamma(n-r+1)) - tgamma(nott+1) / (tgamma(r+1) * tgamma(nott-r+1));
        }

        cout << total << endl;
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