#include "call_lib.h"
/*
Five of a kind, where all five cards have the same label: AAAAA
Four of a kind, where four cards have the same label and one card has a different label: AA8AA
Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
High card, where all cards' labels are distinct: 23456
*/
typedef struct {
    char hand[5];
    int strength;
} card_t;

card_t* parse(char* input) {
    card_t* cards =  create_list(card_t);
    int offset = 0, read;
    card_t card = {0}; 
    while (sscanf(input + offset, "%s %d\n%n", card.hand, &card.strength, &read) != EOF) {
        push_list(cards, card);
        offset += read;
    }
    return cards;
}
// A, K, Q, J, T,
char change_order(char ch) {
    switch (ch)
    {
    case 'A':
        return 'e';
    case 'K':
        return 'd';
    case 'Q':
        return 'c';
    case 'J':
        return '1';
    case 'T':
        return 'a';
    default:
        printf("not kown char");
        exit(EXIT_FAILURE);
    }
}

int compare_order(card_t* a, card_t* b) {
    int i;
    for (i=0;i<5;i++) {
        int num1 = isdigit(a->hand[i]) ? a->hand[i]: change_order(a->hand[i]);
        int num2 = isdigit(b->hand[i]) ? b->hand[i]: change_order(b->hand[i]);
        if (num1 - num2 == 0) {
            continue;
        }
        return num1 - num2;
    }   
    printf("cannot reach here!\n");
    exit(EXIT_FAILURE);
    return 0;
}

void add_count(char* hand, int bucket[]) {
    int i;
    for (i=0; i<5; i++) {
        size_t hash_key = hash_65599(&hand[i], 1);
        bucket[hash_key % 37]++;
    }
}

void use_joker(char* hand, int bucket[]) {
    int i;
    size_t j_key = hash_65599("J", 1);
    int most_freq[2] = {0, -1};

    for (i=0; i<5; i++) {
        size_t hash_key = hash_65599(&hand[i], 1);
        if (hand[i] == 'J') {
            continue;
        }
        if (most_freq[0] < bucket[hash_key % 37]) {
            most_freq[0] = bucket[hash_key % 37];
            most_freq[1] = hand[i];
        } 
    }

    for (i=0; i<5; i++) {
        if (hand[i] == 'J') {
            hand[i] = most_freq[1];
        }
    }
}

int compare_type(card_t* a, card_t* b) {
    int i, entro_a = 0, entro_b = 0;
    size_t a_idx, b_idx;
    int a_count[37] = {0};
    int b_count[37] = {0};
    char a_joker[6];
    char b_joker[6];
    memcpy(a_joker, a->hand, strlen(a->hand)+1);
    memcpy(b_joker, b->hand, strlen(b->hand)+1);

    add_count(a_joker, a_count);
    add_count(b_joker, b_count);
    
    use_joker(a_joker, a_count);
    use_joker(b_joker, b_count);

    memset(a_count, 0, sizeof(a_count));
    memset(b_count, 0, sizeof(b_count));
    add_count(a_joker, a_count);
    add_count(b_joker, b_count);

    for (i=0; i<5; i++) {
        entro_a += a_count[hash_65599(&a_joker[i], 1) % 37];
        entro_b += b_count[hash_65599(&b_joker[i], 1) % 37];
    }
    // printf("%d %d\n", entro_a, entro_b);
    return entro_a - entro_b;
}

int compare(const void* a, const void* b) {
    int type = compare_type((card_t*)a,(card_t*)b);
    int order;
    if (type != 0) {
        return type;
    }
    order = compare_order((card_t*)a,(card_t*)b);
    return order;
}

void part1(char* input) {
    int i;
    card_t* cards = parse(input);
    size_t winnings = 0;
  
    qsort(cards, list_len(cards), list_stride(cards), compare);

    for (i=0;i< list_len(cards); i++) {
        winnings += (i + 1) * cards[i].strength;
    }

    printf("%lld\n", winnings);
    destroy_list(cards);
}

void part2(char* input) {

}


SOLUTION("./inputs/q7.txt")
