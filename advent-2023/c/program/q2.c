#include "call_lib.h"

static int MIN_CARDS[] = {12,13,14};
enum {RED, GREEN, BLUE};

void parse(char* input, char pattern[], int result, int next) {
    int offset = 0, read = 0;
    char line[200];
    while(sscanf(input+offset, pattern, line, &read) == result) {
        char pattern[] = "%[^;]%n";
        parse1(line, pattern, 1, 1);
        offset += read + next;
    } 
}


void part1(char* input) {
    char pattern[] = "Game %*d:%[^\n]%n";
    parse(input, pattern, 1, 1);
}

void part2(char* input) {

}


SOLUTION("./inputs/q2.txt")
