#include <stdio.h>
#include <stdlib.h>

#define length 3 

int main(void) {
    char buffer[length];
    char line[length];

    while (1) {
        if (fgets(buffer, length, stdin) == NULL) {
            clearerr(stdin);
            break;
        }

        if (sscanf(buffer, "%s", line) == 1) {
            printf("scanned: %s\n", line);
        }

    }

    return 0;
}
