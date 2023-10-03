#include "lib.h"

#define N 1000

int is_unique(char pat[]) {
    switch (strlen(pat))
    {
    case 2:case 4:case 3:case 7:
        return TRUE;
    default:
        return FALSE;
    }
}

void part1(FILE *file) {
    char line[N];
    uint32_t unique_numbers = 0;

    while (fgets(line, N, file) != NULL) {
        size_t i;
        char pat[20];
        uint32_t read = 0, offset = 0, found = 0;
        
        while (TRUE) {
            found = sscanf_s(line + offset, "%s %n", pat, sizeof(pat), &read);
            offset += read;
            if (strcmp(pat, "|")==0) {
                break;
            }
        }
        while (TRUE) {
            found = sscanf_s(line + offset, "%s %n", pat, sizeof(pat), &read);
            if (found == -1) {
                break;
            }
            if (is_unique(pat)) unique_numbers++;
            offset += read;
        }
    }
    printf("part 1 unique_numbers - %d\n", unique_numbers);
}
// 2 - 1,4 - 4,3 - 7,7 - 8, 
// 6 - 0,6,9
// 5 - 2,3,5
void part2(FILE *file) {

}
 

AOC_MAIN("./inputs/q8.txt")

