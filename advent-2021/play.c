#include <stdio.h>
#include <stdlib.h>

typedef struct {
    int num;
} fib_t;

int fib(fib_t* fib_num) {
    fib_t* prev;
    fib_t* pprev;
    int r;
    if (fib_num->num <= 1) {
        return fib_num->num;
    }

    prev = malloc(sizeof(fib_t));
    pprev = malloc(sizeof(fib_t));
    prev->num = fib_num->num - 1;
    pprev->num = fib_num->num - 2;

    r =  fib(prev) + fib(pprev); 
    free(pprev);
    free(prev);
    return r;
}

int main(void) {
    // fib_t fib_num = {5};
    int r;
    fib_t* fib_num = malloc(sizeof(fib_t));
    fib_num->num = 5;
    r = fib(fib_num);
    printf("%d\n", r);
    free(fib_num);
    return 0;
}
