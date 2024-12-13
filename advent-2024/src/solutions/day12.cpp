#include "day.hpp"

typedef pair<int, int> Point_t;
typedef tuple<int, int, char> Seen_t;

const int DIRECTIONS[4][2] = {{0,1},{1,0},{0,-1},{-1,0}};
const int NDIRECTIONS[4][2] = {{0,0},{0,1},{1,0},{1,1}};

void part1(int row, int n_col, map<Point_t, char> garden);
void part2(int n_row, int n_col, map<Point_t, char> garden);

string Day12::solve(stringstream& input_buffer) {
    map<Point_t, char> garden;
    string line;

    int row = 0;
    int n_col;
    while (getline(input_buffer, line)) {
        for (int col=0; col < line.size(); col++) {
            garden[Point_t({row,col})] = line[col];
        }
        row++;
    }
    n_col = line.size();

    part1(row, n_col, garden);
    part2(row, n_col, garden);

    return "day12";
}

int get_area(Point_t pt, map<Point_t, char>& garden, set<Point_t>& visited) {
    int perimeter = 0;
    int area = 0;
    char curr_ch = garden[pt];

    vector<Point_t> stack;
    stack.push_back(pt);

    while (!stack.empty()) {
        Point_t curr_pt = stack.back();
        stack.pop_back();

        if (visited.contains(curr_pt)) {
            continue;
        }
        visited.insert(curr_pt);

        auto [row, col] = curr_pt;
        ++area;

        int n_cnt = 0;
        for (auto dir: DIRECTIONS) {
            int nrow = row + dir[0];
            int ncol = col + dir[1];
            Point_t next_pt = {nrow, ncol};
            if (garden.contains(next_pt) && garden[next_pt] == curr_ch) {
                stack.push_back(next_pt);
                n_cnt++;
            }
        }
        perimeter += 4 - n_cnt;
    }

    return area * perimeter;
}

set<Point_t> get_area_pts(Point_t pt, map<Point_t, char>& garden, set<Point_t>& visited) {
    set<Point_t> area_pts;
    
    char curr_ch = garden[pt];

    vector<Point_t> stack;
    stack.push_back(pt);

    while (!stack.empty()) {
        Point_t curr_pt = stack.back();
        stack.pop_back();

        if (visited.contains(curr_pt)) {
            continue;
        }
        visited.insert(curr_pt);

        auto [row, col] = curr_pt;
        area_pts.insert(curr_pt);

        for (auto dir: DIRECTIONS) {
            int nrow = row + dir[0];
            int ncol = col + dir[1];
            Point_t next_pt = {nrow, ncol};
            if (garden.contains(next_pt) && garden[next_pt] == curr_ch) {
                stack.push_back(next_pt);
            }
        }
    }

    return area_pts;
}

void part1(int n_row, int n_col, map<Point_t, char> garden) {
    set<Point_t> visited;

    int total = 0;
    for (int row=0; row < n_row; row++) {
        for (int col=0; col < n_col; col++) {
            Point_t pt = {row, col}; 
            if (visited.contains(pt)) {
                continue;
            }
            total += get_area(pt, garden,visited);
        }
    }
    cout << "Part1: " << total << endl;
}

int countDiagonalPoints(int r, int c, set<Point_t>& p) {
    return p.count({r-1,c-1}) + p.count({r-1,c+1}) + 
           p.count({r+1,c-1}) + p.count({r+1,c+1});
}

int get_perimeter(Point_t start, set<Point_t>& perimeters) {
    int corners = 0;
    set<Point_t> visited;

    vector<Point_t> stack;
    stack.push_back(start);


    while (!stack.empty()) {
        auto [row, col] = stack.back();
        stack.pop_back();
        
        Point_t pt = {row, col};
        if (visited.contains(pt)) continue;
        visited.insert(pt);

        int count_n = 0;
        vector<int> dir_diff;
        for (int i=0; i <  ARRAY_SIZE(DIRECTIONS); i++) {
            int nrow = row + DIRECTIONS[i][0];
            int ncol = col + DIRECTIONS[i][1];
            Point_t next_pt = {nrow, ncol};

            if (perimeters.contains(next_pt)) {
                stack.push_back(next_pt);
                count_n++;
                dir_diff.push_back(i); 
            }
        }

        if ((count_n == 2 && (abs(dir_diff[0] - dir_diff[1])  == 1 || abs(dir_diff[0] - dir_diff[1])  == 3)) || (count_n == 4 && countDiagonalPoints(row ,col, perimeters) == 3)) {
            corners++;
        } 
    }

    return corners;
}

void part2(int n_row, int n_col, map<Point_t, char> garden) {
        set<Point_t> visited;

    int total = 0;
    for (int row=0; row < n_row; row++) {
        for (int col=0; col < n_col; col++) {
            Point_t pt = {row, col}; 
            if (visited.contains(pt)) {
                continue;
            }
            set<Point_t> area_pts =  get_area_pts(pt, garden,visited);
            int area = area_pts.size(); 
            int min_row = numeric_limits<int>::max();
            int min_col = numeric_limits<int>::max();
            int max_row = numeric_limits<int>::min(); 
            int max_col = numeric_limits<int>::min();

            for (const auto& pt : area_pts) {
                min_row = min(min_row, pt.first);
                max_row = max(max_row, pt.first);
                min_col = min(min_col, pt.second);
                max_col = max(max_col, pt.second);
            }            
            set<Point_t> perimeters;

            int i = 0;
            for (int mrow=min_row; mrow <= max_row; mrow++) {
                int j=0;
                for (int mcol=min_col; mcol <= max_col; mcol++) {
                    if (area_pts.contains(Point_t(mrow, mcol))) {
                        for (auto dir: NDIRECTIONS) {
                            int nprow = mrow + dir[0] + i;
                            int npcol = mcol + dir[1] + j;
                            perimeters.insert(Point_t(nprow, npcol));
                        }
                    }
                    j++;
                }
                i++;
            }
            int perimeter = get_perimeter(*perimeters.begin(), perimeters);
            total += area * perimeter;
        }
    }
    cout << "Part2: " << total << endl;
}

