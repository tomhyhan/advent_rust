#include "lib.h"

HashMap *create_graph(FILE *file) {
    char from[6], to[6], line[100];
    HashMap *graph = init_hashmap(); 
    PointVector* temp;
    char* ch;
    while (fscanf_s(file, "%[^-]-%s\n", from, sizeof(from), to, sizeof(to)) != EOF) {
        if (find_val(graph, from) == NULL) {
            PointVector *values = init_ptr_vector(1000);
            printf("here\n");
            insert(graph, from, values);
        }
        
        if (find_val(graph, to) == NULL) {
            PointVector *values = init_ptr_vector(1000);
            printf("here\n");
            insert(graph, to, values);
        }
        temp = find_val(graph, from);
        // temp = find_val(graph, from);
        push_pv(temp, to);
        printf("%d\n", temp->size);
        // push_pv(((PointVector* )find_val(graph, to)), from);
        printf("here\n");
        printf("%s", (char* )temp->array[0]);
        break;
        }
    return graph;
}


void solution(FILE *file) {
    HashMap *graph = create_graph(file);

}

AOC_MAIN_ONE("./inputs/q12.txt")
