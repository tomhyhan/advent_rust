#include <stdio.h>


int main(void) {
    char test[2][2] = {{'1','2'},{'3','4'}};
    char* t[2];
    t[0] = test[0];
    t[1] = test[1];
    printf("%s\n", t[0]);
    return 0;
}
