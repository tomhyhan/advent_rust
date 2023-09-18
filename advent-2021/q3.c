#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "lib.h"
#include "vector.h"

#define INPUT_BITS 12
#define BIT_LENGTH INPUT_BITS +1
#define TRUE 1
#define FALSE 0

int main(void) {
    FILE *file = read_file_data("./inputs/q3.txt");
    char line[16], gamma[BIT_LENGTH] = "", epsilon[BIT_LENGTH] = "";
    unsigned int *data, gamma_bit, eclp_bit, gam, eps;
    unsigned int i, sc, bit, nv, n1, filter_size, count = 0;
    const char *one = "1";
    const char *zero = "0";
    Vector *search = init_vector(500);

    while (fgets(line, sizeof(line), file) != NULL ) {
        count++;
    }

    data = calloc(count, sizeof(unsigned int));

    fseek(file, 0, SEEK_SET);
    for(i = 0; i < count; i++) {
        data[i] = strtol(fgets(line, sizeof(line), file), NULL, 2);
    }

    fclose(file);
    sc = count;
    for (bit = 0; bit < INPUT_BITS; bit++) {
       for(i = nv = n1 = 0; i < sc; i++) {
            if(data[i] & (1 << (INPUT_BITS - 1 - bit)))
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
    printf("result: %ld\n", strtol(gamma, NULL, 2) * strtol(epsilon, NULL, 2));

    memcpy(search->array, data, sizeof(int) * (count));
    search->size = count;

    for (bit = 0; bit < INPUT_BITS; bit++) { 
        filter_size = 0;
        for(i = nv = n1 = 0; i < search->size; i++) {
            if(search->array[i] & (1 << (INPUT_BITS - 1 - bit)))
                n1++;
            nv++;
        }
        
        gamma_bit = nv - n1 <= n1 ? 1: 0;
        for (i=0; i < search->size; i++) {
            if (((search->array[i] >> (INPUT_BITS - 1 - bit)) & 1) == gamma_bit) {
                search->array[filter_size++] = search->array[i];
            }
        }
        search->size = filter_size;
    }
    gam = search->array[0];
    printf("%d\n", search->array[0]);
    
    memcpy(search->array, data, sizeof(int) * (count));
    search->size = count;

    for (bit = 0; bit < INPUT_BITS; bit++) { 
        filter_size = 0;
        for(i = nv = n1 = 0; i < search->size; i++) {
            if(search->array[i] & (1 << (INPUT_BITS - 1 - bit)))
                n1++;
            nv++;
        }
        
        eclp_bit = nv - n1 <= n1 ? 0: 1;
        for (i=0; i < search->size; i++) {
            if (((search->array[i] >> (INPUT_BITS - 1 - bit)) & 1) == eclp_bit) {
                search->array[filter_size++] = search->array[i];
            }
        }
        search->size = filter_size;
    }
    eps =  search->array[0];
    printf("%d\n", search->array[0]);
    printf("%d\n", gam*eps);

    return 0;
}



