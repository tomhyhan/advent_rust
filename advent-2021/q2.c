#include <stdio.h>


int main(void) {
    FILE *file;
    errno_t err;

    int num, result;
    char line[10], ins[20];
    int coord[] = {0,0};

    err = fopen_s(&file, "./inputs/q2.txt", "r");

    if (err != 0) {
        printf("Failed to open the file. Error code: %d\n", err);
        return 1;
    }
    
    while (fgets(line, sizeof(line), file) != NULL ) {
        result = sscanf_s(line, "%s %d", ins,sizeof(ins), &num); 
        if (result == 2) {
            switch (ins[0])
            {
            case 'f':
                coord[0] += num;
                break;
            case 'd':
                coord[1] += num;
                break;
            case 'u':
                coord[1] -= num;
                break;
            default:
                return 1;
                break;
            } 
        }
    }
    printf("result : %d\n", coord[0] * coord[1]);
    
    return 0;
}
