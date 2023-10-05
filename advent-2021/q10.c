#include "lib.h"

HashMap *init_matching_brackets(void) {
    HashMap *hashmap = init_hashmap();
    insert(hashmap, ")", "(");
    insert(hashmap, "]", "[");
    insert(hashmap, ">", "<");
    insert(hashmap, "}", "{");
    insert(hashmap, "(", ")");
    insert(hashmap, "[", "]");
    insert(hashmap, "<", ">");
    insert(hashmap, "{", "}");
    return hashmap;
}

int64_t scores(char bracket) {
    switch (bracket) {
        case ')':
            return 3;
        case ']':
            return 57;
        case '}':
            return 1197;
        case '>':
            return 25137;
        default:
            exit(1);
    }
}

int64_t scores_complete(char bracket) {
    switch (bracket) {
        case ')':
            return 1;
        case ']':
            return 2;
        case '}':
            return 3;
        case '>':
            return 4;
        default:
            exit(1);
    }
}

char *find_match(HashMap *matching_brackets, char braket) {
    char *match;
    char current[2];
    current[0] = braket;
    current[1] = '\0';
    match = find_val(matching_brackets, current);
    return match;
}

int64_t find_corrupted(char line[]) {
    size_t i;
    int64_t total = 0;
    PointVector *stack = init_ptr_vector(1000);
    HashMap *matching_brackets = init_matching_brackets(); 

    for (i=0; *(line+i) != '\0'; i++) {
        char current = line[i];
        if (strchr("([<{", current) != NULL) {
            push_pv(stack, &line[i]);
        } else {
            char *poped_char = pop_pv(stack); 
            char *match = find_match(matching_brackets, current);
            
            if (*match != *poped_char)  {
                /* return scores(line[i]); part 1 */
                return -1;
            }
        }
    }
    while (stack->size != 0) {
        char *poped_char = pop_pv(stack);
        char *match = find_match(matching_brackets, *poped_char);
        
        total = total * 5 + scores_complete(match[0]); 
    }
    free_ptr_vector(stack);
    freeHashMap(matching_brackets);
    return total;
}

int cmpfunc(const void *a, const void *b) {
    int64_t x = *(const int64_t *)a;
    int64_t y = *(const int64_t *)b;
    if (x < y) {
        return -1;
    } else if (x > y) {
        return 1;
    } else {
        return 0;
    }
}

void solution(FILE *file) {
    char line[1000];
    Vector *points = init_vector(1000);
    /*uint64_t error_pts = 0; part 1*/
    
    while (fscanf_s(file, "%s",line, sizeof(line)) != EOF) {
        int64_t point = find_corrupted(line); 
        if (point == -1) {
            continue;
        }
        push(points, point);
    };
    qsort(points->array, points->size, sizeof(int64_t), cmpfunc);
    print_vector(points);
    printf("middle point: %lld", points->array[points->size/2]);

    free_vector(points);
}

AOC_MAIN_ONE("./inputs/q10.txt")

