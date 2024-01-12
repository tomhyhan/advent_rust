#include "call_lib.h"

typedef struct {
    int dir, steps;
    char rgb[10];
} plan_t;

plan_t* parse(char* input) {
    int offset = 0, read;
    plan_t* plans = create_list(plan_t);
    plan_t plan = {0};
    char d;
    while (sscanf(input + offset, "%c %d (%[^)])\n%n", &d, &plan.steps, plan.rgb, &read) == 3) {
        switch (d) {
            case 'R':
                plan.dir = 0; 
                break;
            case 'L':
                plan.dir = 2;
                break;
            case 'U':
                plan.dir = 3;
                break;
            case 'D':
                plan.dir = 1;
                break;
            default:
                printf("wrong\n");
                exit(1);
                break;
        }
        push_list(plans, plan);
        offset += read;
    }
    return plans;
}

void part1(char* input) {
    plan_t* plans = parse(input);
    int directions[4][2] = {{0,1},{1,0},{0,-1},{-1,0}};
    size_t i;
    int64_t pos[2] = {0,0};
    size_t area = 0;
    size_t outer = 0;
    for (i=0; i < list_len(plans); i++) {
        plan_t p = plans[i];
        int64_t next_pos[2] = {0};
        next_pos[0] = pos[0] + directions[p.dir][0] * p.steps;
        next_pos[1] = pos[1] + directions[p.dir][1] * p.steps;
        area += pos[1] * next_pos[0] - pos[0] * next_pos[1];
        outer += p.steps;
        memcpy(pos, next_pos, sizeof(pos));
    }
    // a = i + b / 2 - 1;
    // 
    printf("area: %lld\n", area / 2);
    printf("outter: %lld\n", outer);
    printf("inner: %lld\n", (area / 2 + 1 - outer / 2) + outer);
    destroy_list(plans);
}

void part2(char* input) {
    plan_t* plans = parse(input);
    int directions[4][2] = {{0,1},{1,0},{0,-1},{-1,0}};
    size_t i;
    int64_t pos[2] = {0,0};
    size_t area = 0;
    size_t outer = 0;
    for (i=0; i < list_len(plans); i++) {
        plan_t p = plans[i];
        char hex[6];
        int dir;
        char* end;
        int64_t steps;
        int64_t next_pos[2] = {0};

        sscanf(p.rgb, "#%5s%1d", hex, &dir);
        steps = strtoll(hex, &end, 16);
        
        next_pos[0] = pos[0] + directions[dir][0] * steps;
        next_pos[1] = pos[1] + directions[dir][1] * steps;
        area += pos[1] * next_pos[0] - pos[0] * next_pos[1];
        outer += steps;
        memcpy(pos, next_pos, sizeof(pos));
    }
    
    // a = i + b / 2 - 1;
    printf("area: %lld\n", area / 2);
    printf("outter: %lld\n", outer);
    printf("inner: %lld\n", (area / 2 + 1 - outer / 2) + outer);
    destroy_list(plans);
}

SOLUTION("./inputs/q18.txt")
