#include <stdio.h>

typedef struct {
    int x;
    char* y;
} test ;

test parse_polymer(void) {
    test t = {0};
    t.x = 7;
    t.y = "asdf";
    return t;
}

int main(void) {
    test t = parse_polymer();
    printf("%s", t.y);
    printf("%d", t.x);
    return 0;
}
