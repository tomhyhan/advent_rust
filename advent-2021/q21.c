#include "lib.h"

typedef struct {
    int32_t value;
    int32_t rolled;
    int32_t pos;
} player_t;

typedef struct {
    player_t players[5];
    bool turn;
} players_t;

static int16_t get_next_dice(void) {
    static int16_t dice = 1;
    if (dice == 101) dice = 1;
    return dice++;
}

void parse_input(FILE* file, player_t* player1, player_t* player2) {
    fscanf_s(file, "Player 1 starting position: %d\n", &player1->pos);
    fscanf_s(file, "Player 2 starting position: %d\n", &player2->pos);
}

int32_t roll_dice(void) {
    return get_next_dice() + get_next_dice() + get_next_dice();
}

void play(player_t* player) {
    int32_t dice_value = roll_dice();
    int32_t next_pos = (player->pos + dice_value) % 10;
    
    player->pos = next_pos == 0 ? 10 : next_pos;
    player->value += player->pos;
    player->rolled += 3;
}

void play_universe(player_t player1, player_t player2) {
    PointVector* stack = init_ptr_vector(100);
    int64_t p1_universe = 0;
    int64_t p2_universe = 0;
    players_t game = {0};
    game.players[0] = player1;
    game.players[1] = player2;
    game.turn = TRUE;

    push_pv(stack, &game);
    while (stack->size != 0) {
        players_t* cgame = pop_pv(stack);
        printf("pos: %d\n", cgame->players[0].pos);
        if (cgame->turn == TRUE) {
            players_t* cp = malloc(sizeof(players_t));
            printf("start\n");
            memcpy(cp, cgame, sizeof(players_t));
            printf(":asdf\n");
            printf("cp: %d\n", cp->players[0].pos);
            cp->turn = FALSE;
            cp->players[0].pos = 9;
            push_pv(stack, cp);
        }
    }

    // player_t cp = {0}; 
    // memcpy(&cp, &player1, sizeof(player_t));
    // printf("cp: %d\n", cp.value);
    free_ptr_vector(stack);
}

void solution(FILE* file) {    
    player_t player1 = {0};
    player_t player2 = {0};
    bool play1_turn = TRUE;

    parse_input(file, &player1, &player2);    

    // part 1 
    // while (player1.value < 1000 && player2.value < 1000) {
    //     if (play1_turn) {
    //         play(&player1);
    //     } else {
    //         play(&player2);
    //     }
    //     play1_turn = !play1_turn;
    // }
    // printf("%d\n", player1.value);
    // printf("%d\n", player1.value * (player1.rolled + player2.rolled));

    play_universe(player1, player2);
}

AOC_MAIN_ONE("./inputs/q21.txt")
