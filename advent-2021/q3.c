#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "lib.h"

#define INPUT_BITS 12
#define BIT_LENGTH INPUT_BITS +1

int main(void) {
    FILE *file = read_file_data("./inputs/q3.txt");
    char line[16], gamma[BIT_LENGTH] = "", epsilon[BIT_LENGTH] = "";
    unsigned int *data, *search;
    unsigned int i, sc, bit, nv, n1, count = 0;
    const char *one = "1";
    const char *zero = "0";

    while (fgets(line, sizeof(line), file) != NULL ) {
        count++;
    }
    data = calloc(count, sizeof(unsigned int));
    search = calloc(count, sizeof(unsigned int));

    fseek(file, 0, SEEK_SET);
    for(i = 0; i < count; i++) {
        data[i] = strtol(fgets(line, sizeof(line), file), NULL, 2);
    }

    fclose(file);

    memcpy(search, data, sizeof(unsigned int) * (sc = count));
    for (bit = 0; bit < INPUT_BITS; bit++) {
       for(i = nv = n1 = 0; i < sc; i++) {
            if(search[i] & (1 << (INPUT_BITS - 1 - bit)))
                n1++;
            nv++;
        }
        if (nv - n1 <= n1) {
            strcat_s(gamma, BIT_LENGTH, one);
            strcat_s(epsilon, BIT_LENGTH, zero);
        } else {
            strcat_s(gamma, BIT_LENGTH, zero);
            strcat_s(epsilon, BIT_LENGTH, one);
        }
    }
    printf("%s\n", gamma);
    printf("%s\n", epsilon);
    printf("result: %ld", strtol(gamma, NULL, 2) * strtol(epsilon, NULL, 2));

    free(data);
    free(search);

    return 0;
}
