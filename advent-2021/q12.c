#include "lib.h"

PointVector* find_pv_value(HashMap* graph, char* key) {
    return ((PointVector* )find_val(graph, key));
}

static size_t visited_id(char small[]) {
    static char visited[20][20] = {0};
    static size_t i,length = 0;

    for (i=0; i < length; i++) {
        if (strcmp(visited[i],small) == 0) {
            return i;
        }
    }
    strcpy_s(visited[length], sizeof(visited[length]), small);
    return length++;
}

HashMap *create_graph(FILE *file) {
    char from[6], to[6];
    HashMap *graph = init_hashmap(); 

    while (fscanf_s(file, "%[^-]-%s\n", from, sizeof(from), to, sizeof(to)) != EOF) {
        if (find_val(graph, from) == NULL) {
            PointVector *values = init_ptr_vector(1000);
            insert(graph, _strdup(from), values);
        }
        
        if (find_val(graph, to) == NULL) {
            PointVector *values = init_ptr_vector(1000);
            insert(graph, _strdup(to), values);
        }
        push_pv(find_pv_value(graph, _strdup(from)), _strdup(to));
        push_pv(find_pv_value(graph, _strdup(to)), _strdup(from));
    }

    return graph;
}

int64_t traverse(HashMap* graph, bool visited[], char node[], bool visited_twice) {
    int64_t paths = 0, i;
    PointVector *current_pt;
    size_t vid = visited_id(node);

    if (strcmp(node, "end") == 0) {
        return 1;
    }
    if (node[0] >= 'a' && visited[vid]) {
        if (visited_twice) {
            return 0;
        }
        visited_twice = TRUE;
    } 
    
    if (node[0] >= 'a') {
        visited[vid] = TRUE;
    }

    current_pt = find_pv_value(graph, node); 
    for (i=0; i < current_pt->size; i++) {
        bool new_visited[20];
        char* value = (char *)current_pt->array[i];
        if (strcmp(value, "start") == 0) continue;
        memcpy(new_visited, visited, 20 * sizeof(bool));
        paths += traverse(graph, new_visited, value, visited_twice);
    }

    return paths;
}


void solution(FILE *file) {
    HashMap *graph = create_graph(file);
    bool visited[20] = {FALSE};
    int64_t paths; 
    paths = traverse(graph, visited, "start", FALSE);
    printf("paths: %lld\n", paths);
    freeHashMap(graph);
}

AOC_MAIN_ONE("./inputs/q12.txt")
