#include "call_lib.h"

enum { BUCKET_SIZE = 1333 };
// sample
// enum { BUCKET_SIZE = 23 };

typedef struct node node_t;
bool (*compare_p)(int a, int b);

bool less_then(int a, int b) {
    return a < b? TRUE: FALSE;
}

bool bigger_than(int a, int b) {
    return a > b? TRUE: FALSE;
}

typedef struct {
    char flow[10][20];
    size_t size;
} workflow_t;

typedef struct node {
    char key[10];
    workflow_t items;
} node_t;

typedef struct  {
    int x,m,a,s;
} part_t;

typedef struct  {
    int64_t x[2],m[2],a[2],s[2];
} pr_t;

typedef struct {
    node_t* bucket[BUCKET_SIZE];
    // extra parts info
    part_t parts[200];
    size_t p_size;
} hash_t;

size_t get_bucket_idx(char* key) {
    size_t hash_key = hash_65599(key, strlen(key));
    return hash_key % BUCKET_SIZE;
}

void insert_hash(hash_t* hashmap, node_t* node) {    
    size_t bucket_idx = get_bucket_idx(node->key);
    size_t start = bucket_idx;
    do {
        if (hashmap->bucket[start] == NULL) {
            hashmap->bucket[start] = node;
            return;
        }
        start = (start + 1) % BUCKET_SIZE;
    } while (start != bucket_idx);
}

node_t* search_hash(hash_t* hashmap, char* key) {
    size_t bucket_idx = get_bucket_idx(key);
    size_t start = bucket_idx;

    do {
        if (hashmap->bucket[start] == NULL) {
            return NULL;
        }
        if (strcmp(hashmap->bucket[start]->key, key) ==0) {
            return hashmap->bucket[start];
        }
        start = (start + 1) % BUCKET_SIZE;
    } while (start != bucket_idx);

    return NULL;
}

void init_hashmap(hash_t* hash) {
    size_t i;
    for (i=0; i < BUCKET_SIZE; i++) {
        hash->bucket[i] = NULL;
    }
}

hash_t parse(char* input) {
    int offset = 0, read;
    char key[10];
    char value[100];
    int x,m,a,s;
    hash_t hashmap  = {0};
    init_hashmap(&hashmap);
    while(sscanf(input + offset, "%[^{]{%[^}]}\n%n", key, value, &read) == 2) {
        node_t* node = calloc(1, sizeof(node_t));
        char* token;
        char delim[] = ",";
        memcpy(node->key, key, sizeof(key));
        token = strtok(value, delim);
        while (token) {
            memcpy(node->items.flow[node->items.size++], token, (strlen(token) + 1) * sizeof(char));
            token = strtok(NULL, delim);
        }
        insert_hash(&hashmap, node);
        offset += read;
    }


    while (sscanf(input + offset, "{x=%d,m=%d,a=%d,s=%d}\n%n", &x,&m,&a,&s, &read) == 4) {
        part_t part = {0};
        part.x = x;
        part.m = m;
        part.a = a;
        part.s = s;
        hashmap.parts[hashmap.p_size++] = part;
        offset += read;
    }
    return hashmap;
}

void destroy_hash(hash_t* hashmap) {
    size_t i;

    for (i=0; i < BUCKET_SIZE; i++) {
        free(hashmap->bucket[i]);
    }
}

bool compare(char part_num, char cmp, int bound, part_t part) {
    int part_value;
    switch (part_num)
    {
        case 'x':
            part_value = part.x;
            break;
        case 'm':
            part_value = part.m;
            break;
        case 'a':
            part_value = part.a;
            break;
        case 's':
            part_value = part.s;
            break;
        default:
            exit(1);
            break;
    }
    return cmp == '<'? 
        less_then(part_value, bound): 
        bigger_than(part_value, bound);
}

bool valid_part(hash_t hashmap, size_t part_idx) {
    part_t part = hashmap.parts[part_idx];
    char* key = "in";

    while (!(strcmp(key, "R") == 0 || strcmp(key, "A") == 0)) {
        size_t i;
        node_t* node = search_hash(&hashmap, key); 
        assert(node);
        for (i=0; i < node->items.size - 1; i++) {
            char* flow = node->items.flow[i];
            char part_num, cmp;
            char dkey[10];
            int bound;
            sscanf(flow, "%c%c%d:%s", &part_num, &cmp, &bound, dkey);
            if (compare(part_num, cmp,bound, part)) {
                key = dkey;
                goto CONTINUE;
            }
        }
        key = node->items.flow[node->items.size - 1];
        CONTINUE:
            continue;
    }
    return strcmp(key, "R") == 0? FALSE: TRUE;
}

void part1(char* input) {
    hash_t hashmap = parse(input);
    size_t i;
    size_t total = 0;
    
    for (i=0; i < hashmap.p_size;i++) {
        if (valid_part(hashmap, i)) {
            part_t part = hashmap.parts[i];
            total += part.x + part.m + part.a + part.s;
        }
    }
    printf("total: %lld\n", total);
    destroy_hash(&hashmap);
}

void cmp_range(char part_num, char cmp, int bound, pr_t* pt, pr_t* opt) {
    int part_value;
    switch (part_num)
    {
        case 'x':
            if (cmp == '>') {
                pt->x[0] = MAX(pt->x[0], bound + 1);
                opt->x[1] = MIN(opt->x[1], bound);
            } else {
                pt->x[1] = MIN(pt->x[1], bound - 1);
                opt->x[0] = MAX(opt->x[0], bound);
            }
            break;
        case 'm':
            if (cmp == '>') {
                pt->m[0] = MAX(pt->m[0], bound + 1);
                opt->m[1] = MIN(opt->m[1], bound);

            } else {
                pt->m[1] = MIN(pt->m[1], bound - 1);
                opt->m[0] = MAX(opt->m[0], bound);
            }            
            break;
        case 'a':
            if (cmp == '>') {
                pt->a[0] = MAX(pt->a[0], bound + 1);
                opt->a[1] = MIN(opt->a[1], bound);

            } else {
                pt->a[1] = MIN(pt->a[1], bound - 1);
                opt->a[0] = MAX(opt->a[0], bound);
            }            
            break;
        case 's':
            if (cmp == '>') {
                pt->s[0] = MAX(pt->s[0], bound + 1);
                opt->s[1] = MIN(opt->s[1], bound);
            } else {
                pt->s[1] = MIN(pt->s[1], bound - 1);
                opt->s[0] = MAX(opt->s[0], bound);
            }            
            break;
        default:
            exit(1);
            break;
    }
}

void find_valid_parts(char* src, pr_t pt, hash_t* hashmap, int64_t* total) {
    size_t i;
    node_t* node;
    if (strcmp(src, "A") == 0) {
        *total += (pt.x[1] - pt.x[0] + 1) * (pt.m[1] - pt.m[0] + 1) * (pt.a[1] - pt.a[0] + 1) * (pt.s[1] - pt.s[0] + 1);
        return;
    };
    if (strcmp(src, "R") == 0) {
        return;
    };
    node = search_hash(hashmap, src);
    for (i=0; i < node->items.size - 1; i++) {
        char* flow = node->items.flow[i];
        char part_num, cmp;
        char dkey[10];
        int bound;
        pr_t new_pt = pt;
        sscanf(flow, "%c%c%d:%s", &part_num, &cmp, &bound, dkey);
        cmp_range(part_num, cmp, bound, &new_pt, &pt);
        find_valid_parts(dkey, new_pt, hashmap, total);
    }
    find_valid_parts(node->items.flow[node->items.size - 1], pt, hashmap, total);
}

void part2(char* input) {
    hash_t hashmap = parse(input);
    size_t i;
    int64_t total = 0;
    pr_t pt = {0};
    pt.x[0] = 1;
    pt.x[1] = 4000;
    pt.m[0] = 1;
    pt.m[1] = 4000;
    pt.a[0] = 1;
    pt.a[1] = 4000;
    pt.s[0] = 1;
    pt.s[1] = 4000;

    find_valid_parts("in", pt, &hashmap, &total);

    printf("total: %lld\n", total);
    destroy_hash(&hashmap);
}

SOLUTION("./inputs/q19.txt")
