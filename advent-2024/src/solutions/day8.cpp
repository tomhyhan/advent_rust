#include "day.hpp"

typedef pair<int, int> Point_t;

void part1(map<char, vector<Point_t>> locations, int n_row, int n_col);
void part2(map<char, vector<Point_t>> locations, int n_row, int n_col);

string Day8::solve(stringstream& input_buffer) {
    map<char, vector<Point_t>> locations;

    string line;
    int row=0;
    int n_col;
    while (getline(input_buffer, line)) {
        int n_line = line.size();
        for (int col=0; col < n_line; col++) {
            if (line[col] != '.') {
                locations[line[col]].push_back(Point_t{row,col});
            }
        }
        n_col = n_line;
        row++;
    }
    // cout << row << n_col << endl;
    part1(locations, row, n_col);
    part2(locations, row, n_col);

    return "day8";
}


void part1(map<char, vector<Point_t>> locations, int n_row, int n_col) {
    set<Point_t> unique_locations;

    for (auto location: locations) {
        for (int i=0; i < location.second.size(); i++) {
            int irow = location.second[i].first;
            int icol = location.second[i].second;
            for (int j=0; j < location.second.size(); j++) {
                if (i==j) continue;
                int jrow = location.second[j].first;
                int jcol = location.second[j].second;
                int row = irow + irow - jrow;
                int col = icol + icol - jcol;
                if (0 <= row && row < n_row && 0 <= col && col < n_col ) {
                    unique_locations.insert(Point_t{row,col});
                }
            }
        }
    } 
    cout << "Part1: " << unique_locations.size() << endl;
}

void part2(map<char, vector<Point_t>> locations, int n_row, int n_col) {
    set<Point_t> unique_locations;

    for (const auto& location: locations) {
        for (int i=0; i < location.second.size(); i++) {
            for (int j=0; j < location.second.size(); j++) {
                if (i==j) continue;
                int irow = location.second[i].first;
                int icol = location.second[i].second;
                int jrow = location.second[j].first;
                int jcol = location.second[j].second;
                int drow = irow - jrow;
                int dcol = icol - jcol;
                while (0 <= irow && irow < n_row && 0 <= icol && icol < n_col ) {
                    unique_locations.insert(Point_t{irow,icol});
                    irow += drow;
                    icol += dcol;
                }
            }
        }
    } 
    cout << "Part2: " << unique_locations.size() << endl;
}