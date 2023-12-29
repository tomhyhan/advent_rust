#include "call_lib.h"

static int MIN_CARDS[] = {12,13,14};
enum {RED, GREEN, BLUE};

char* get_digit(char* line) {
    char* c_ptr = line;
    int num;
    for(; *c_ptr != ' '; c_ptr++) {
    }
    return c_ptr;
}

bool is_possible(char* line, int min_rgb[]) {
    char color;
    char* digit_end;
    int digit;
    bool possible = TRUE;
    for (; *line != '\0';line++) {
        if (isdigit(*line)) {
            digit_end = get_digit(line);
            digit = strtol(line, &digit_end, 10);
            line = digit_end; 
        } 
        if (*line == 'r' && *(line+2) == 'd') {
            if ( digit > MIN_CARDS[RED]) {
                possible = FALSE;
            }
            min_rgb[RED] = MAX(min_rgb[RED], digit);
        }
        if (*line == 'g' && *(line+1) == 'r') {
            if (digit > MIN_CARDS[GREEN]) {
                possible = FALSE;
            }
            min_rgb[GREEN] = MAX(min_rgb[GREEN], digit);
        }
        if (*line == 'b' && *(line+1) == 'l') {
            if (digit > MIN_CARDS[BLUE]) {
                possible = FALSE;
            }
            min_rgb[BLUE] = MAX(min_rgb[BLUE], digit);
        }
    }
    return possible;
}

void parse(char* input, char pattern[], bool part1) {
    int offset = 0, read = 0, id;
    char line[200];
    int games = 0;
    int power = 0;
    while(sscanf(input+offset, pattern, &id, line, &read) == 2) {
        char* token;
        char delim[] = ";";
        int min_rgb[] = {INT_MIN,INT_MIN,INT_MIN};
        token = strtok(line, delim);
        while (token) {
            if (!is_possible(token, min_rgb)) {
                if (part1) {
                    goto NEXT_LINE;
                }
            } 
            token = strtok(NULL, delim);
        }
        games += id;
        power += min_rgb[RED] * min_rgb[GREEN] * min_rgb[BLUE];
        NEXT_LINE:
            offset += read + 1;
    } 
    printf("games %d\n",games);
    printf("power %d\n",power);
}


void part1(char* input) {
    char pattern[] = "Game %d:%[^\n]%n";
    parse(input, pattern, TRUE);
}

void part2(char* input) {
    char pattern[] = "Game %d:%[^\n]%n";
    parse(input, pattern, FALSE);
}


SOLUTION("./inputs/q2.txt")
