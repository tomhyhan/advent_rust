#include "lib.h"

typedef struct {
    int32_t depth;
    int32_t value;
} num_t;

static PointVector* parse_snailfishes(FILE* file) {
    PointVector *lines = init_ptr_vector(1000);
    static char line[1000];
    while (fscanf_s(file, "%s\n", line, sizeof(line)) != EOF) {
        push_pv(lines, _strdup(line));
    }
    return lines;
}

bool explode(PointVector* snailfishes) {
    size_t i;
    // printf("explode\n");
    for (i=1; i < snailfishes->size; i++ ) {
        num_t* num1 = snailfishes->array[i-1];
        num_t* num2 = snailfishes->array[i];
        if (num1->depth != 5 || num2->depth != 5) {
            continue;
        }
        if (i > 1) {
            num_t* left = snailfishes->array[i-2];
            left->value = left->value + num1->value;
        } 

        if (i < (snailfishes->size)-1) {
            num_t* right = snailfishes->array[i+1];
            right->value = right->value + num2->value;
        } 

        num1->value = 0;
        num1->depth -= 1;

        remove_pv(snailfishes, i);
        
        return TRUE;
    }
    return FALSE;
}

bool split(PointVector* snailfishes) {
    size_t i;
    // printf("split\n");

    for (i=0; i < snailfishes->size; i++) {
        num_t* num = snailfishes->array[i];
        num_t* num1;
        int32_t val;
        if (num->value < 10) continue; 

        val = num->value;
        insert_pv(snailfishes, i);

        snailfishes->array[i] = (num_t*)malloc(sizeof(num_t));
        
        num = snailfishes->array[i]; 
        num1 = snailfishes->array[i+1];

        num->value = val / 2;
        num->depth = num1->depth + 1;

        num1->value = ceil((float)val/2);
        num1->depth += 1;

        return TRUE;
    } 
    
    return FALSE;
}

void add_snailfishes(PointVector* snailfishes, PointVector* lines) {
    size_t i;
    for (i=0; i < lines->size; i++) {
        size_t j;
        int32_t depth = 0;
        char* current_line;
        for (j=0; j < snailfishes->size; j++) {
            ((num_t*)snailfishes->array[j])->depth++;
        }
        if (i > 0) depth ++;
        current_line = (char*)lines->array[i]; 
        while (*current_line != '\0') {
            num_t* num;
            switch (*current_line) {
                case ',':
                case ']':
                case '[':
                    if (*current_line == '[') {
                        depth++;
                    } else if (*current_line == ']') {
                        depth--;
                    }
                    current_line++;
                    continue;
                default:
                    num = (num_t*)malloc(sizeof(num_t));
                    num->depth = depth;
                    num->value = *current_line - '0';
                    push_pv(snailfishes, num);
                    current_line++;
            }
            
        }
        while (explode(snailfishes) || split(snailfishes)){
        }
        // while (split(snailfishes));
        // explode(snailfishes);
        // split(snailfishes);
    }
    // printf("\n");
    // for (i=0; i < snailfishes->size; i++) {
    //     num_t* n = snailfishes->array[i];
    //     printf("%d %d\n", n->value, n->depth);
    // }
}

int32_t calc_magnitude(PointVector* snailfishes) {
    while (snailfishes->size != 1) {
        size_t i;
        for (i=1; i< snailfishes->size; i++) {
            num_t* num1 = snailfishes->array[i-1];
            num_t* num2 = snailfishes->array[i];

            if (num1->depth != num2->depth) continue; 

            num1->value = num1->value * 3 + num2->value * 2;
            num1->depth--;

            remove_pv(snailfishes, i);
            break;
        }
    }
    return ((num_t*)snailfishes->array[0])->value;
}

int32_t find_largest_magnitude(PointVector* lines) {
    size_t i, j;
    int32_t max_magnitude = 0;
    for (i = 0; i < lines->size; i++) {
        for (j = 0; j < lines->size; j++) {
            PointVector* new_line; 
            PointVector* snailfishes;
            int32_t magnitude;
            char* copy_i = NULL; 
            char* copy_j = NULL;
            printf("asdf");
            if (i==j) continue;

            new_line = init_ptr_vector(1000);
            snailfishes = init_ptr_vector(10000);
            
            copy_i = _strdup((char*)lines->array[i]);
            copy_j = _strdup((char*)lines->array[j]);
            
            push_pv(new_line, copy_i);
            push_pv(new_line, copy_j);

            add_snailfishes(snailfishes, new_line);
            magnitude = calc_magnitude(snailfishes);
            printf("two: %d\n",magnitude);

            max_magnitude = MAX(max_magnitude, magnitude);
            free_ptr_vector(new_line);
            free_ptr_vector(snailfishes);
        }
    }
    return max_magnitude;
}

void solution(FILE* file) {
    PointVector* snailfishes = init_ptr_vector(10000);
    PointVector* lines = parse_snailfishes(file);
    int32_t magnitude, max_magnitude;
    
    add_snailfishes(snailfishes, lines);
    magnitude = calc_magnitude(snailfishes);

    max_magnitude = find_largest_magnitude(lines);
    
    printf("magnitude: %d\n", magnitude);
    printf("max_magnitude: %d\n", max_magnitude);

    free_ptr_vector(lines);
    free_ptr_vector(snailfishes);
}


AOC_MAIN_ONE("./inputs/q18.txt")
