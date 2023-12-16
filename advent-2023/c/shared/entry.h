#define _CRT_SECURE_NO_WARNINGS

#ifndef ENTRY_H
#define ENTRY_H

#include <stdio.h>
#include <stdlib.h>

#define TRUE 1
#define FALSE 0
#define bool int

char *read_input(char filename[])
{
    FILE* file;
    char* buffer;
    int length;

    file = fopen(filename, "r");
    if (!file) {
        printf("file cannot be open!");
        exit(EXIT_FAILURE);
    }
    
    fseek(file, 0, SEEK_END);
    length = ftell(file);
    fseek(file, 0, SEEK_SET);

    buffer = malloc((length+1) * sizeof(char));
    fread(buffer, sizeof(char), length, file);

    buffer[length] = '\0';
    
    fclose(file);

    return buffer;
}


#define SOLUTION(filename)                      \
    int main(void)                              \
    {                                           \
        char* input = read_input(filename);      \
        part1(input);                                \
        part2(input);                           \
        free(input);                            \
        return 0;                               \
    }

#endif

