#include "lib.h"

#define MAX_GAMES 11
#define WINNING_SCORE 21

typedef struct {
    int32_t value;
    int32_t rolled;
    int32_t pos;
} player_t;

typedef struct {
  uint64_t wins[MAX_GAMES];
  uint64_t loss[MAX_GAMES];
} game_results_t;


void parse_input(FILE* file, player_t* player1, player_t* player2) {
    fscanf_s(file, "Player 1 starting position: %d\n", &player1->pos);
    fscanf_s(file, "Player 2 starting position: %d\n", &player2->pos);
}

static game_results_t *quantum_game_results(uint32_t initial_pos) {
    const uint8_t ROLL_PROBABILITY[] = {1, 3, 6, 7, 6, 3, 1};
    uint8_t player_a, player_b, score, roll;
    uint64_t table[MAX_GAMES][MAX_GAMES][22] = {0};
    game_results_t *results;
    
    table[0][initial_pos][0] = 1;

    for (player_a = 1; player_a < MAX_GAMES; player_a++) {
        for (player_b = 1; player_b < MAX_GAMES; player_b++) {
            for (score = 0; score < WINNING_SCORE; score++) {
                for (roll = 0; roll < 7; roll++) {
                    uint64_t q = ((player_b + roll + 2) % 10) + 1;
                    uint64_t v = MIN(q + score, WINNING_SCORE);
                    // printf("p %d %d %d %d\n", player_a, player_b, score, roll);
                    // printf("p %d %d \n", q, v);
                    // printf("score %d\n", ROLL_PROBABILITY[roll] * table[player_a - 1][player_b][score]);
                    // q = 4 v = 4 a = 1
                    // 0 1 0
                    // when roll the dice when a 0 b 5 s
                    table[player_a][q][v] += ROLL_PROBABILITY[roll] * table[player_a - 1][player_b][score];
                    // printf("score %d\n", table[player_a][q][v]);
                }
                // break;
            }
            // break;
        }
        // break;
    }

    results = calloc(1, sizeof(game_results_t));

    for (player_a = 0; player_a < MAX_GAMES; player_a++) {
        for (player_b = 0; player_b < MAX_GAMES; player_b++) {
            for (score = 0; score < WINNING_SCORE; score++) {
                results->loss[player_a] += table[player_a][player_b][score];
            }
            results->wins[player_a] += table[player_a][player_b][WINNING_SCORE];
            printf("results: %lld\n",results->wins[player_a]);
        }
    }

    return results;
}

void solution(FILE* file) {    
    player_t player1 = {0};
    player_t player2 = {0};
    game_results_t *p1_results;
    game_results_t *p2_results;
    uint64_t p1_score = 0, p2_score = 0;
    uint8_t i;
    
    parse_input(file, &player1, &player2);    

    p1_results = quantum_game_results(player1.pos);
    p2_results = quantum_game_results(player2.pos);

  
    printf("%lld\n", p1_results->wins[3]);
    printf("%lld\n", p2_results->loss[2]);
    for (i = 1; i < MAX_GAMES; i++) {
        p1_score += p1_results->wins[i] * p2_results->loss[i - 1];
        p2_score += p2_results->wins[i - 1] * p1_results->loss[i - 1];
    }

    free(p1_results);
    free(p2_results);

    printf("%lld %lld", p1_score, p2_score);
}

AOC_MAIN_ONE("./inputs/q21.txt")
