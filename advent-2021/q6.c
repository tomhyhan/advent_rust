#include <stdio.h>
#include "ptr_vector.h"

void prinf_vector(PointVector *vector)
{
    printf("%d\n", *(int *)vector->array[0]);
    *(int *)vector->array[0] = 5;
    printf("%d\n", *(int *)vector->array[0]);
}

int main(void)
{
    PointVector *vector = init_ptr_vector(10);
    int value = 1;
    push(vector, &value);
    printf("%d\n", *(int *)vector->array[0]);
    value = 2;
    prinf_vector(vector);
    return 1;
}
