#include "lib.h"

static int64_t IDX = 0;

static char* convert_packet(char packet[]) {
    static char binary[10000] = "";
    int64_t i,j;
    for (i=0; *packet != '\0'; i++) {
        int8_t hex_num = *packet >= 'A'? *packet - 'A' + 10: *packet - '0'; 
        char* hex = binary + i * 4;
        for (j=3; j >= 0; j--) {
            char bit = ((hex_num >> j) & 1) + '0';
            *hex = bit;
            hex++;
        }
        packet++;
    }
    return binary;
}

/*
VVVTTTAAAAABBBBBCCCCC
version/typeID/
id=4 literal, mul of 4bits
id=^4  operator, length_type: 1bit
0 -> 15bits:length of bits / 1 -> 11bits: num of sub-packets
*/

int64_t read_packet(char* bpacket, int64_t end) {
    char* packet = (char*)calloc(end+1, sizeof(char));
    int64_t start;
    int64_t label;
    for (start = 0; start < end; start++) {
        int64_t len = strlen(packet);
        packet[len] = *bpacket;
        bpacket++;
    }

    packet[end] = '\0';
    label = strtol(packet, NULL, 2);
    IDX += end;
    free(packet);
    return label;
}

int64_t decode_values(Vector* values, int64_t type_id) {
        int64_t i;
    switch (type_id) {
    case 0:
        int64_t sum = 0;
        for (i=0; i < values->size; i++) {
            sum += values->array[i];
        }
        return sum;
    case 1:
        int64_t product = 1;
        for (i=0; i < values->size; i++) {
            product *= values->array[i];
        }
        return product;
    case 2:
        int64_t min = values->array[0];
        for (i=0; i < values->size; i++) {
            min = MIN(min,values->array[i]);
        }
        return min;
    case 3:
        int64_t max = values->array[0];
        for (i=0; i < values->size; i++) {
            max = MAX(max,values->array[i]);
        }
        return max;
    case 5:
        return values->array[0] > values->array[1] ? 1 : 0;
    case 6:
        return values->array[0] < values->array[1] ? 1 : 0;
    case 7:
        return values->array[0] == values->array[1] ? 1 : 0;
    default:
        printf("wrong type id\n");
        exit(1);
        break;
    }

}

int64_t decode(char* bpacket, int64_t* total_version) {
    int64_t version = read_packet(bpacket + IDX, 3);
    int64_t type_id = read_packet(bpacket + IDX, 3);
    bool length_type_id;
    int64_t num_of_packets;

    *total_version += version;

    if (type_id == 4) {
        bool next = TRUE;
        int64_t value = 0;
        while (next) {
            next = read_packet(bpacket + IDX, 1) & 1;
            value = value << 4 | read_packet(bpacket + IDX, 4);     
        }
        return value;
    } 

    length_type_id = read_packet(bpacket + IDX, 1);

    if (length_type_id == 0) {
        int64_t length = read_packet(bpacket + IDX, 15);
        Vector *values = init_vector(100);
        int64_t result;
        while (length > 0) {
            int64_t before = IDX;
            int64_t value = decode(bpacket, total_version);
            push(values, value);
            length =  length - (IDX - before);
        }
        result = decode_values(values, type_id);
        free_vector(values);
        return result; 
    } else {
        Vector *values = init_vector(100);
        int64_t result;

        num_of_packets = read_packet(bpacket + IDX, 11);
        while (num_of_packets > 0) {
            int64_t value = decode(bpacket, total_version);
            push(values, value);
            num_of_packets--;
        }

        result = decode_values(values, type_id);
        free_vector(values);
        return result; 
    }
}

void solution(FILE *file) {
    char packet[10000];
    char* bpacket; 
    int64_t total_version;
    int64_t value;
    fscanf_s(file, "%s\n", packet, sizeof(packet));
    bpacket = convert_packet(packet);
    
    value = decode(bpacket, &total_version);
    printf("value: %lld\n",value);
    printf("%c\n", 1 + '0');
} 

AOC_MAIN_ONE("./inputs/q16.txt")
