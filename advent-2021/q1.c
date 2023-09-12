#include <stdio.h>
#include <stdlib.h>

#define N 2000

int main(void)
{
    FILE *file;
    errno_t err;
    
    char line[N];
    int nums[N];
    int num;
    int length = sizeof(nums) / sizeof(nums[0]);
    int count = 0, matches = 0;
    int i, previous;

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

    previous = nums[0];
    for (i = 1; i < length; i++) {
        if (nums[i] > previous) {
            matches++;
        }
        previous = nums[i];
    }
    printf("%d", matches);

    fclose(file);

    return 0;
}
