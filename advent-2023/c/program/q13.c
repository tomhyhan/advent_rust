#include "call_lib.h"

void print_grid(bool** grid) {
    size_t j;
    size_t i;
    for (i=0; i < list_len(grid); i++) {
        bool* row = grid[i];
        for (j=0; j < list_len(row); j++) {
            printf("%c", row[j]? '#': '.');
        }
        puts("");
    }
    puts("");
}

bool** create_grid(char* input) {
    bool** grid = create_list(bool*);
    int offset = 0, read;
    char line[100];
    while (sscanf(input + offset, "%s\n%n", line, &read) == 1) {
        bool* row = create_list(bool);
        char* pline = line;
        // printf("%s\n", line);
        for (; *pline != '\0'; pline++) {
            bool tile = *pline == '#'? TRUE: FALSE;
            push_list(row, tile);
        }
        push_list(grid, row);
        offset += read;
    }
    
    return grid;    
}

bool*** create_grids(char* input) {
    char* start = input;
    char* end;
    char delim[]= "\n\n";
    bool*** grids = create_list(bool**);
    bool** grid;

    while ((end = strstr(start, delim)) != NULL) {
        *end = '\0';
        printf("%s\n",  start);
        grid = create_grid(start);
        push_list(grids, grid);
        start = end + 2;
    }
    // *end = '\0';
    // printf("%s\n",  end);
    // start = end + 2;
    // grid = create_grid(start);
    // print_grid(grid);
    // push_list(grids, grid);
    
    return grids;
}

void destory_grid(bool*** grids) {
    size_t i;
    for (i=0; i < list_len(grids); i++) {
        size_t j;
        bool** grid = grids[i];
        for (j=0; j < list_len(grid); j++) {
            destroy_list(grid[j]);
        }
        destroy_list(grid);
    }
    destroy_list(grids);
}

int convert_size_t(bool* row) {
    int i;
    int irow = 0;
    for (i = 0; i < list_len(row); i++) {
        irow <<= 1;
        irow |= row[list_len(row) -1 -i]? 1 : 0; 
    }    
    return irow;
}


int bit_pattern_matches(bool* row1, bool* row2) {
    int i;
    int diff = 0;
    for (i = 0; i < list_len(row1); i++) {
        if (row1[i] != row2[i]) {
            diff++;
        }
    }    
    // printf("%d\n", diff);
    return diff;
}

size_t find_mirror(bool** grid) {
    size_t pos;
    for (pos=1; pos < list_len(grid); pos++) {
        bool found_match = TRUE;
        int left = pos - 1, right = pos;
        int diffs = 0;
        while (left >=0 && right < list_len(grid)) {
            diffs += bit_pattern_matches(grid[left], grid[right]);
            left--;
            right++;
        }
        if (diffs == 1) {
            return pos;
        }
    }
    return -1;
}


bool** rotate_90(bool** grid) {
    bool** new_grid = create_list(bool*);
    size_t row_len = list_len(grid);
    size_t col_len = list_len(grid[0]);
    size_t row, col;
    for (col=0; col < col_len; col++) {
        bool* line = create_list(bool);
        for (row=0; row < row_len; row++) {
            push_list(line, grid[row][col]);
        }
        push_list(new_grid, line);
    }
    return new_grid;  
}


void part1(char* input) {
    bool*** grids = create_grids(input);
    size_t i, j, total = 0;

    for (i=0; i < list_len(grids); i++) {
        bool** grid = grids[i];
        bool** rotate_grid;
        size_t row = find_mirror(grid);
        size_t col;
        if (row != -1) {
            total += row * 100;
            continue;
        }

        rotate_grid = rotate_90(grid);
        col = find_mirror(rotate_grid);
        total += col;
        destroy_list(rotate_grid);
    }
    printf("total: %d\n", total);
    destory_grid(grids);
}    


void part2(char* input) {

}

SOLUTION("./inputs/q13.txt")

// part1
// bool bit_pattern_matches(bool* row1, bool* row2) {
//     size_t irow1 = convert_size_t(row1);
//     size_t irow2 = convert_size_t(row2);
//     return (irow1 ^ irow2) == 0? TRUE : FALSE;
// }

// size_t find_mirror(bool** grid) {
//     size_t pos;
//     for (pos=1; pos < list_len(grid); pos++) {
//         bool found_match = TRUE;
//         int left = pos - 1, right = pos;
//         while (left >=0 && right < list_len(grid)) {
//             if (!bit_pattern_matches(grid[left], grid[right])) {
//                 goto CONTINUE;
//             }
//             left--;
//             right++;
//         }
//         return pos;
//         CONTINUE:
//             continue;
//     }
//     return -1;
// }
