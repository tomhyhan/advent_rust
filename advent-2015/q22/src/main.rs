use std::{collections::VecDeque, fs};

#[derive(Debug)]
struct Player {
    hp: i32,
    atk: i32,
    armor: i32,
    mana: i32,
    shield: i32,
    posion: i32,
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
            posion: 0,
            recharge: 0,
        }
    }
}

#[derive(Debug)]
struct State {
    skill: String,
    consume: i32,
    turns: i32,
    atk: i32,
    heal: i32,
    armor: i32,
    mana: i32,
}

enum Skills {
    missile,
    darin,
    shield,
    posion,
    recharge,
}

#[derive(Debug)]
struct Boss {
    hp: i32,
    atk: i32,
}

impl Boss {
    fn new() -> Self {
        Boss { hp: 71, atk: 10 }
    }
}
struct Game {
    player: Player,
    boss: Boss,
}
fn dfs() {
    let mut new_player = Player::new();
    let mut queue = VecDeque::from([]);
}
fn main() {
    dfs()
}
