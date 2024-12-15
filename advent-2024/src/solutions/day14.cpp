#include "day.hpp"

// const int MAX_H = 7;
// const int MAX_W = 11;
const int MAX_H = 103;
const int MAX_W = 101;


class Robot {
    public:
        int x,y,vx,vy;
        int inx,iny,invx,invy;
        Robot(int x1,int y1,int vx1,int vy1) : inx(x1), iny(y1),invx(vx1),invy(vy1){}

        void init() {
            x = inx;
            y = iny;
            vx = invx;
            vy = invy;
        }
        void step(int second) {
            int newX  = (((x + vx * second) % MAX_W) + MAX_W) % MAX_W;
            int newY  = (((y + vy * second) % MAX_H) + MAX_H) % MAX_H;
            x = static_cast<int>(newX);
            y = static_cast<int>(newY);
        }

        int qi () {
            int bx = (MAX_W - 1) / 2;
            int by = (MAX_H - 1) / 2;
            if (x < bx && y < by) {
                return 0;
            } else if (x > bx && y < by) {
                return 1;
            } else if (x < bx && y > by) {
                return 2;
            } else if (x > bx && y > by) {
                return 3;
            }

            return -1;
        }
};


void part1 (vector<Robot> robots);
void part2 (vector<Robot> robots);

string Day14::solve(stringstream& input_buffer) {
    
    regex pattern(R"(p=(\d+),(\d+) v=(-?\d+),(-?\d+))");
    string line;
    vector<Robot> robots;

    while (getline(input_buffer, line)) {
        smatch matches;
        regex_search(line, matches, pattern);
        robots.push_back(Robot(stoi(matches[1]),stoi(matches[2]),stoi(matches[3]),stoi(matches[4])));
    }

    part1(robots);
    part2(robots);
    return "day14";
}

void part1 (vector<Robot> robots) {
    int steps = 100;
    int quadrants[4] = {0,};
    for (auto robot: robots) {
        robot.init();
        robot.step(100);
        int qi = robot.qi();
        if (qi != -1) {
            quadrants[qi]++;
        }
    }

    int total = reduce(begin(quadrants), end(quadrants), 1, multiplies<>());
    cout << "Part1: " << total << endl;
}

void part2 (vector<Robot> robots) {
    int steps = 12000;

    ofstream outFile("simulation.txt", std::ios::app);
    int score = 999999999;
    for (int step=0; step < steps; step++) {
        vector<string> tree(MAX_H, string(MAX_W, '.'));
        int q[4] = {0,};
        for (auto robot: robots) {
            robot.init();
            robot.step(step);
            tree[robot.y][robot.x] = '#';
            int qi = robot.qi();
            if (qi != -1) {
                q[qi]++;
            } 
        }
        int total = reduce(begin(q), end(q), 1, multiplies<>());
        if (total < score) {
            cout << step << endl;
            score = total;
            outFile << "Step: " << step << "\n";
                for (const auto& row : tree) {
                    outFile << row << "\n";
                }
            outFile << "\n";
        } 
    }    
    outFile.close();

}
