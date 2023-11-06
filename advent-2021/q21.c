#include "lib.h"

typedef struct {
    int32_t value;
    int32_t rolled;
    int32_t pos;
} player_t;

typedef struct {
    int64_t universe1;
    int64_t universe2;
} universes_t;

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

void roll_universe_dice(player_t* player, size_t roll) {
    int32_t next_pos = (player->pos + roll) % 10;
    
    player->pos = next_pos == 0 ? 10 : next_pos;
    player->value += player->pos;
    player->rolled += 3;
}

universes_t play_universe(players_t* game, int32_t memo[]) {
    universes_t unis = {0};
    // size_t key = game->players[0].value
    if (game->players[0].value >= 21) {
        universes_t unis = {0};
        unis.universe1++;
        return unis;
    }
    
    if (game->players[1].value >= 21) {
        universes_t unis = {0};
        unis.universe2++;
        return unis;
    }

    
    if (game->turn) {
        size_t roll;
        for (roll=1; roll <= 3; roll++) {
            players_t* cp = malloc(sizeof(players_t));
            universes_t uni1;
            memcpy(cp, game, sizeof(players_t));
            cp->turn = FALSE;
            roll_universe_dice(&cp->players[0], roll);
            uni1 = play_universe(cp, memo);
            unis.universe1 += uni1.universe1;
            unis.universe2 += uni1.universe2;
            free(cp);
        }
    } else if (!game->turn)  {
        size_t roll;
        for (roll=1; roll <= 3; roll++) {
            players_t* cp = malloc(sizeof(players_t));
            universes_t uni2;
            memcpy(cp, game, sizeof(players_t));
            cp->turn = TRUE;
            roll_universe_dice(&cp->players[1], roll);
            play_universe(cp, memo);
            uni2 = play_universe(cp, memo);
            unis.universe1 += uni2.universe1;
            unis.universe2 += uni2.universe2;
            free(cp);
        }
    }
    // memo
    
    return unis;
}

void solution(FILE* file) {    
    player_t player1 = {0};
    player_t player2 = {0};
    bool play1_turn = TRUE;
    players_t *game;
    int64_t universe = 0;
    int32_t memo[3600] = {0};
    universes_t unis;

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
    game = malloc(sizeof(players_t));
    game->players[0] = player1;
    game->players[1] = player2;
    game->turn = TRUE;
    unis = play_universe(game, memo);
    free(game);
    printf("universes: %lld\n", unis.universe1);
}

AOC_MAIN_ONE("./inputs/q21.txt")
