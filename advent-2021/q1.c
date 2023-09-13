#include <stdio.h>
#include <stdlib.h>

#define N 2000

int main(void)
{
    FILE *file;
    errno_t err;
    
    char line[N];
    int nums[N], num, i, previous, current;
    int count = 0, matches = 0;

    err = fopen_s(&file, "./inputs/q1.txt", "r");
    if (err != 0) {
        printf("Failed to open the file. Error code: %d\n", err);
        return 1;
    }

    while (fgets(line, sizeof(line), file) != NULL) {
        if (sscanf_s(line, "%d", &num) == 1) {
            nums[count] = num;
            count++;
        } else {
            printf("Invalid input: %s", line);
        }
    }

    previous = nums[0] + nums[1] + nums[2];
    for (i = 1; i < count-2; i++) {
        current = nums[i] + nums[i+1] + nums[i+2]; 
        if (current > previous) {
            matches++;
        }
        previous = current;
    }
    printf("matches - %d", matches);

    fclose(file);

    return 0;
}
