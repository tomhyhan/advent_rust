#include "call_lib.h"

static int MIN_CARDS[] = {12,13,14};
enum {RED, GREEN, BLUE};

void part1(char* input) {
    int offset = 0, read = 0;
    char line[150];
    while(sscanf(input+offset, "Game %*d:%[^\n]%n", line, &read) != EOF) {
        char delim[] = ";";
        char* token = strtok(line, delim);
        while (token) {
            char* cards = strtok(token, ",");
            while (cards) {
                printf("cards: %s\n", cards);
                cards = strtok(NULL, ",");
            }
            token = strtok(NULL, delim);
        }
        offset += read;
    } 
    
    TEST:
        printf("asdf\n");
}

void part2(char* input) {

}


SOLUTION("./inputs/q2.txt")
