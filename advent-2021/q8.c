#include "lib.h"

#define N 1000



int is_unique(char pat[]) {
    switch (strlen(pat))
    {
    case 2:case 4:case 3:case 7:
        return TRUE;
    default:
        return FALSE;
    }
}

void part1(FILE *file) {
    char line[N];
    uint32_t unique_numbers = 0;

    while (fgets(line, N, file) != NULL) {
        size_t i;
        char pat[20];
        uint32_t read = 0, offset = 0, found = 0;
        
        while (TRUE) {
            found = sscanf_s(line + offset, "%s %n", pat, sizeof(pat), &read);
            offset += read;
            if (strcmp(pat, "|")==0) {
                break;
            }
        }
        while (TRUE) {
            found = sscanf_s(line + offset, "%s %n", pat, sizeof(pat), &read);
            if (found == -1) {
                break;
            }
            if (is_unique(pat)) unique_numbers++;
            offset += read;
        }
    }
    printf("part 1 unique_numbers - %d\n", unique_numbers);
}

typedef struct {
    char in[10][10];
    char out[4][10];
} bounds_t;

PointVector* parse_input(FILE *file) {
    PointVector* segments = init_ptr_vector(1000);

    bounds_t *bounds = malloc(sizeof(bounds_t));
    while(fscanf(file, "%s %s %s %s %s %s %s %s %s %s | %s %s %s %s\n", bounds->in[0], bounds->in[1], bounds->in[2], bounds->in[3], bounds->in[4], bounds->in[5], bounds->in[6], bounds->in[7], bounds->in[8], bounds->in[9], bounds->out[0], bounds->out[1], bounds->out[2], bounds->out[3]) != EOF) {
        push_pv(segments, bounds);
        bounds = malloc(sizeof(bounds_t));
    };
    free(bounds);
    
    return segments;
}

char* find_str_with_len(bounds_t* b, size_t len) {
    size_t i;
    for (i=0; i < 10; i++) {
        if (strlen(b->in[i]) == len) {
            return b->in[i];
        }
    }
    printf("should not reach here!\n");
    exit(1);
}

size_t find_similarities(char* out, char* in) {
    size_t cnt = 0;
    char* o;
    char* i;
    
    for (o = out; *o != '\0'; o++) {
        for (i = in; *i != '\0'; i++) {
            cnt += *o == *i;
        }
    }
    return cnt;

}

void eval_total(PointVector* segments) {
    size_t i;
    int64_t total = 0; 

    for (i=0; i < segments->size; i++) {
        bounds_t* b = segments->array[i];
        size_t j;

        char value[5];
        for (j=0; j < 4; j ++) {
            char* seven = find_str_with_len(b, 3);
            char* four = find_str_with_len(b, 4);
            size_t s_cnt = find_similarities(b->out[j], seven);
            size_t f_cnt = find_similarities(b->out[j], four);
            size_t len = strlen(b->out[j]);

            switch (len) {
                case 2:
                    value[j] = '1'; 
                    break;
                case 3:
                    value[j] = '7'; 
                    break;
                case 4:
                    value[j] = '4'; 
                    break;
                case 5:
                    if (s_cnt == 3) {
                        value[j] = '3';
                    } else {
                        value[j] = f_cnt == 3? '5': '2';
                    }
                    // 2 3 5
                    break;
                case 6:
                    if (s_cnt == 2) {
                        value[j] = '6';
                    } else {
                        value[j] = f_cnt == 4? '9': '0';
                    }
                    break;
                case 7:
                    value[j] = '8'; 
                    break;
                default:
                    exit(1);
            }            
        }
        total += atoi(value);
    }
    printf("%lld\n", total);
}
// 1 -> 2, 4 -> 4, 7 -> 3, 8 -> 7
// 2 -> 5, 3 -> 5, 5 -> 5, 0 -> 6, 6 -> 6, 9 -> 6
//   0:      1:      2:      3:      4:
//  aaaa    ....    aaaa    aaaa    ....
// b    c  .    c  .    c  .    c  b    c
// b    c  .    c  .    c  .    c  b    c
//  ....    ....    dddd    dddd    dddd
// e    f  .    f  e    .  .    f  .    f
// e    f  .    f  e    .  .    f  .    f
//  gggg    ....    gggg    gggg    ....

//   5:      6:      7:      8:      9:
//  aaaa    aaaa    aaaa    aaaa    aaaa
// b    .  b    .  .    c  b    c  b    c
// b    .  b    .  .    c  b    c  b    c
//  dddd    dddd    ....    dddd    dddd
// .    f  e    f  .    f  e    f  .    f
// .    f  e    f  .    f  e    f  .    f
//  gggg    gggg    ....    gggg    gggg


void part2(FILE *file) {
    PointVector *segments = parse_input(file);
    eval_total(segments);
    free_ptr_vector(segments);
    // size_t cnt = find_similarities("acdfg", "acf");
    // printf("%lld\n", cnt);
}
 

AOC_MAIN("./inputs/q8.txt")

