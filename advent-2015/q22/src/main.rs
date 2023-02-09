use core::panic;
use std::cmp;
use std::{collections::VecDeque, fs};

#[derive(Debug, Clone)]
struct Player {
    hp: i32,
    atk: i32,
    armor: i32,
    mana: i32,
    shield: i32,
    poison: i32,
    recharge: i32,
}

impl Player {
    fn new() -> Self {
        Player {
            hp: 50,
            atk: 0,
            armor: 0,
            mana: 500,
            shield: 0,
            poison: 0,
            recharge: 0,
        }
    }
}

// #[derive(Debug)]
// struct State {
//     skill: String,
//     consume: i32,
//     turns: i32,
//     atk: i32,
//     heal: i32,
//     armor: i32,
//     mana: i32,
// }

#[derive(Debug, Clone)]
enum Skills {
    missile,
    drain,
    shield,
    poison,
    recharge,
}

#[derive(Debug, Clone)]
struct Boss {
    hp: i32,
    atk: i32,
}

impl Boss {
    fn new() -> Self {
        Boss { hp: 71, atk: 10 }
    }
}
#[derive(Debug, Clone)]
struct Game {
    player: Player,
    boss: Boss,
    next: Skills,
    mana_total: i32,
}
fn dfs() {
    // let mut new_player = Player::new();
    let mut queue = VecDeque::new();

    queue.push_front(Game {
        player: Player::new(),
        boss: Boss::new(),
        next: Skills::missile,
        mana_total: 0,
    });
    queue.push_front(Game {
        player: Player::new(),
        boss: Boss::new(),
        next: Skills::drain,
        mana_total: 0,
    });
    queue.push_front(Game {
        player: Player::new(),
        boss: Boss::new(),
        next: Skills::poison,
        mana_total: 0,
    });
    queue.push_front(Game {
        player: Player::new(),
        boss: Boss::new(),
        next: Skills::shield,
        mana_total: 0,
    });
    queue.push_front(Game {
        player: Player::new(),
        boss: Boss::new(),
        next: Skills::recharge,
        mana_total: 0,
    });

    let mut s_mana = i32::MAX;
    while let Some(mut game) = queue.pop_front() {
        // player turn

        game.player.hp -= 1;
        if game.player.hp <= 0 {
            continue;
        }
        // cast effects
        // println!("{game:?}");
        game.player.armor = 0;
        if game.player.shield >= 1 {
            game.player.armor = 7;
            game.player.shield -= 1;
        }
        if game.player.poison >= 1 {
            game.boss.hp -= 3;
            game.player.poison -= 1;
        }
        if game.player.recharge >= 1 {
            game.player.mana += 101;
            game.player.recharge -= 1;
        }

        if game.boss.hp <= 0 {
            s_mana = s_mana.min(game.mana_total);
            // s_mana = cmp::min(s_mana, game.mana_total);
            continue;
        }

        // cast a skill
        let mana_spent;
        match game.next {
            Skills::missile => {
                game.boss.hp -= 4;
                mana_spent = 53;
            }
            Skills::drain => {
                game.player.hp += 2;
                game.boss.hp -= 2;
                mana_spent = 73
            }
            Skills::shield => {
                game.player.shield = 6;
                mana_spent = 113
            }
            Skills::poison => {
                game.player.poison = 6;
                mana_spent = 173
            }
            Skills::recharge => {
                game.player.recharge = 5;
                mana_spent = 229
            }
            _ => panic!("unknown skills!"),
        }

        game.player.mana -= mana_spent;
        game.mana_total += mana_spent;

        if game.boss.hp <= 0 {
            s_mana = s_mana.min(game.mana_total);
            continue;
        }
        // boss turn

        game.player.armor = 0;
        if game.player.shield >= 1 {
            game.player.armor = 7;
            game.player.shield -= 1;
        }
        if game.player.poison >= 1 {
            game.boss.hp -= 3;
            game.player.poison -= 1;
        }
        if game.player.recharge >= 1 {
            game.player.mana += 101;
            game.player.recharge -= 1;
        }

        if game.boss.hp <= 0 {
            s_mana = s_mana.min(game.mana_total);
            // s_mana = cmp::min(s_mana, game.mana_total);
            continue;
        }

        game.player.hp -= game.boss.atk - game.player.armor;
        if game.player.hp <= 0 {
            continue;
        }

        if game.player.mana >= 53 {
            queue.push_front(Game {
                next: Skills::missile,
                ..game.clone()
            })
        };
        if game.player.mana >= 73 {
            queue.push_front(Game {
                next: Skills::drain,
                ..game.clone()
            })
        }
        if game.player.mana >= 113 && game.player.shield <= 1 {
            queue.push_front(Game {
                next: Skills::shield,
                ..game.clone()
            })
        }
        if game.player.mana >= 173 && game.player.poison <= 1 {
            queue.push_front(Game {
                next: Skills::poison,
                ..game.clone()
            })
        }
        if game.player.mana >= 229 && game.player.recharge <= 1 {
            queue.push_front(Game {
                next: Skills::recharge,
                ..game.clone()
            })
        }
    }
    // 1176 too low
    println!("{s_mana:?}");
}
fn main() {
    dfs()
}
