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

void add_count(card_t* card, int bucket[]) {
    int i;
    for (i=0; i<5; i++) {
        size_t hash_key = hash_65599(&card->hand[i], 1);
        bucket[hash_key % 37]++;
    }
}

int use_joker(card_t* card, int bucket[]) {
    int i;
    int most_freq[2] = {0,0};
    size_t j_key = hash_65599("J", 1);
    for (i=0; i<5; i++) {
        size_t hash_key = hash_65599(&card->hand[i], 1);
        if (most_freq[0] < bucket[hash_key % 37]) {
            most_freq[0] = bucket[hash_key % 37];
            most_freq[1] = hash_key % 37;
        } else if (hash_key != j_key && most_freq[0] <= bucket[hash_key % 37]) {
            most_freq[0] = bucket[hash_key % 37];
            most_freq[1] = hash_key % 37;
        }
    }

    if (j_key == most_freq[1]) {
        return most_freq[1];
    }

    for (i=0; i<5; i++) {
        if (card->hand[i] == 'J') {
            // size_t hash_key = hash_65599(&card->hand[i], 1);
            // size_t hash_key = hash_65599("J", 1);
            bucket[most_freq[1]] += 1;
            bucket[j_key] -= 1;
        }
    }
    return most_freq[1];
}

int compare_type(card_t* a, card_t* b) {
    int i, entro_a = 0, entro_b = 0;
    size_t a_idx, b_idx;
    int a_count[37] = {0};
    int b_count[37] = {0};
    add_count(a, a_count);
    add_count(b, b_count);

    a_idx = use_joker(a, a_count);
    b_idx = use_joker(b, b_count);

    // printf("%d\n", b_count[hash_65599("2", 1) % 37]);
    // printf("%d\n", a_count[hash_65599("2", 1) % 37]);

    printf("%d\n", a_count[a_idx]);
    printf("%d\n", b_count[b_idx]);
    printf("%d\n", b_idx);
    printf("%d\n", a_idx);
    for (i=0; i<5; i++) {
        entro_a += a->hand[i] == 'J'? a_count[a_idx] : a_count[hash_65599(&a->hand[i], 1) % 37];
        entro_b += b->hand[i] == 'J'? b_count[b_idx]: b_count[hash_65599(&b->hand[i], 1) % 37];
    }
    // printf("entro %d %d\n", entro_a, entro_b);
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
    char test[13] = "23456789JTKAQ";
    for (i=0; i < 13; i++ ) {
        printf("%lld\n", hash_65599(&test[i], 1) %37);
    }
    puts("");    
    qsort(cards, list_len(cards), list_stride(cards), compare);

    for (i=0;i< list_len(cards); i++) {
        // printf("%s\n", cards[i].hand);
        winnings += (i + 1) * cards[i].strength;
    }

    printf("%lld\n", winnings);
    destroy_list(cards);
}

void part2(char* input) {

}


SOLUTION("./inputs/q7.txt")
