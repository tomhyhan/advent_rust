#include "call_lib.h"

typedef struct {
    int winning[10];
    int hand[25];
} card_t;

// sample
// card_t* parse(char* input) {
//     int read, offset = 0;
//     card_t* cards = create_list(card_t);
//     card_t card;
//     while (sscanf(input + offset, "Card %*d: %d %d %d %d %d | %d %d %d %d %d %d %d %d\n%n", &card.winning[0], &card.winning[1], &card.winning[2],&card.winning[3],&card.winning[4], &card.hand[0], &card.hand[1],&card.hand[2],&card.hand[3],&card.hand[4],&card.hand[5],&card.hand[6],&card.hand[7], &read) == 13) {
//         push_list(cards, card);
//         offset += read;
//     }
//     return cards;
// }

card_t* parse(char* input) {
    int read, offset = 0;
    card_t* cards = create_list(card_t);
    card_t card;
    char line[200]; 
    while (sscanf(input + offset, "Card %*d: %[^\n]%n",line, &read) != EOF) {
        int i = 0;
        int line_read, line_offset = 0;
        while (sscanf(line + line_offset, "%d%n", &card.winning[i++], &line_read) == 1) {
            line_offset += line_read;
        };
        line_offset += 2;
        i = 0;
        while (sscanf(line + line_offset, "%d%n", &card.hand[i++], &line_read) == 1){
            line_offset += line_read;
        };

        push_list(cards, card);
        offset += read + 1;
    }
    return cards;
}

bool is_winning_card(int hand, int winning[], int arr_len) {
    int i;
    for (i=0; i < arr_len; i++ ) {
        if (hand == winning[i]) {
            return TRUE;
        }
    }
    return FALSE;
}

void part1(char* input) {
    int i, points = 0;
    card_t* cards = parse(input);
    for (i=0; i < list_len(cards); i++) {
        int j, score = 1;
        for (j=0; j < ARRAY_LEN(cards[i].hand); j++) {
            int hand = cards[i].hand[j];
            if (is_winning_card(hand, cards[i].winning, ARRAY_LEN(cards[i].winning))) {
                score <<= 1; 
            }
        }
        points += (score / 2);
    }
    printf("%d\n", points);
    destroy_list(cards);
}

void part2(char* input) {
    int i, points = 0;
    int card_points[255] = {0};
    
    card_t* cards = parse(input);
    for (i=0; i < list_len(cards); i++) {
        int j, matches = 0;
        for (j=0; j < ARRAY_LEN(cards[i].hand); j++) {
            int hand = cards[i].hand[j];
            if (is_winning_card(hand, cards[i].winning, ARRAY_LEN(cards[i].winning))) {
                matches += 1; 
            }
        }
        if (matches > 0) {
            card_points[i]++;
            for (j=1; j <= matches; j++) {
                card_points[i+j] += card_points[i];
            }
        }
    }

    for (i=0; i < 255; i++) {
        points += card_points[i];
    }
    
    printf("%d\n", points);
    destroy_list(cards);
}

SOLUTION("./inputs/q4.txt")
