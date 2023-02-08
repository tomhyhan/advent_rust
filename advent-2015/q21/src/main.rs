use itertools::{iproduct, Itertools};
use std::{collections::HashMap, fs};

#[derive(Debug)]
struct Info {
    atk: i32,
    def: i32,
    cost: i32,
}

#[derive(Debug)]
struct Store {
    weapons: HashMap<String, Info>,
    armors: HashMap<String, Info>,
    rings: HashMap<String, Info>,
}

impl From<String> for Store {
    fn from(s: String) -> Self {
        let mut t = s.split("\n\n");
        let weapon_info = t.next().unwrap();
        let armor_info = t.next().unwrap();
        let ring_info = t.next().unwrap();
        let mut weapons = HashMap::new();
        let mut armors = HashMap::new();
        let mut rings = HashMap::new();

        let mut w = weapon_info.lines();
        w.next();
        w.for_each(|line| {
            let s: Vec<_> = line
                .split(" ")
                .map(|w| w.trim())
                .filter(|&p| p != "")
                .collect();
            println!("{s:?}");
            weapons.insert(
                s[0].to_string(),
                Info {
                    cost: s[1].parse::<i32>().unwrap(),
                    atk: s[2].parse::<i32>().unwrap(),
                    def: s[3].parse::<i32>().unwrap(),
                },
            );
        });

        let mut a = armor_info.lines();
        a.next();
        a.for_each(|line| {
            let s: Vec<_> = line
                .split(" ")
                .map(|w| w.trim())
                .filter(|&p| p != "")
                .collect();
            println!("{s:?}");
            armors.insert(
                s[0].to_string(),
                Info {
                    cost: s[1].parse::<i32>().unwrap(),
                    atk: s[2].parse::<i32>().unwrap(),
                    def: s[3].parse::<i32>().unwrap(),
                },
            );
        });

        armors.insert(
            "nothing".to_string(),
            Info {
                cost: 0,
                atk: 0,
                def: 0,
            },
        );

        let mut r = ring_info.lines();
        r.next();
        r.for_each(|line| {
            let s: Vec<_> = line
                .split(" ")
                .map(|w| w.trim())
                .filter(|&p| p != "")
                .collect();
            rings.insert(
                s[0].to_string(),
                Info {
                    cost: s[1].parse::<i32>().unwrap(),
                    atk: s[2].parse::<i32>().unwrap(),
                    def: s[3].parse::<i32>().unwrap(),
                },
            );
        });

        rings.insert(
            "nothing".to_string(),
            Info {
                cost: 0,
                atk: 0,
                def: 0,
            },
        );

        Store {
            weapons,
            armors,
            rings,
        }
    }
}

#[derive(Debug)]
struct Player {
    HP: i32,
    atk: i32,
    armor: i32,
    cost: i32,
}

impl Player {
    fn new() -> Self {
        Player {
            HP: 100,
            atk: 0,
            armor: 0,
            cost: 0,
        }
    }

    fn buy_e(&mut self, w: &Info, a: &Info, r1: &Info, r2: &Info) {
        self.atk += w.atk + r1.atk + r2.atk;
        self.armor += a.def + r1.def + r2.def;
        self.cost = w.cost + a.cost + r1.cost + r2.cost;
    }

    fn can_defeat(&self, boss: &Boss) -> bool {
        let my_turns = get_turns(boss.HP, get_atk(self.atk, boss.armor));
        let boss_turns = get_turns(self.HP, get_atk(boss.atk, self.armor));
        my_turns <= boss_turns
    }
}

fn get_atk(atk: i32, armor: i32) -> i32 {
    if atk > armor {
        atk - armor
    } else {
        1
    }
}

fn get_turns(hp: i32, atk: i32) -> i32 {
    hp / atk + i32::from(hp % atk != 0)
}

#[derive(Debug)]
struct Boss {
    HP: i32,
    atk: i32,
    armor: i32,
}

impl Boss {
    fn new() -> Self {
        Boss {
            HP: 103,
            atk: 9,
            armor: 2,
        }
    }
}

fn play_game(store: Store) {
    println!("{:?}", store.weapons);
}

// fn get_power(rings: &HashMap<String, Info>) -> Vec<Vec<&String>> {
//     // println!("{rings:?}")
//     for n in 1..3 {}
//     let mut comb: Vec<_> = rings.keys().combinations(1).collect();
//     let mut comb2: Vec<_> = rings
//         .keys()
//         .combinations(2)
//         .filter(|p| !p.contains(&&"nothing".to_string()))
//         .collect();
//     comb.append(&mut comb2);
//     println!("{:?}", comb);
//     // for i in 0..2 {}
//     comb
// }

fn main() {
    let path = "q21.txt";
    let content = fs::read_to_string(path).expect("fail to read a file");

    let store: Store = content.into();
    // println!("{:?}", store);

    // play_game(store);
    // println!("{:?}", ring_info);
    let boss = Boss::new();
    let wa: Vec<_> = store
        .weapons
        .keys()
        .cartesian_product(store.armors.keys())
        .collect();

    let war = iproduct!(
        store.weapons.keys(),
        store.armors.keys(),
        store.rings.keys(),
        store.rings.keys()
    )
    .filter(|(_, _, r1, r2)| r1 != r2 || r1 == &&"nothing".to_string())
    .map(|(w, a, r1, r2)| {
        let mut player = Player::new();
        player.buy_e(
            &store.weapons.get(w).unwrap(),
            &store.armors.get(a).unwrap(),
            &store.rings.get(r1).unwrap(),
            &store.rings.get(r2).unwrap(),
        );
        player
    })
    .filter(|p| !p.can_defeat(&boss))
    .max_by(|x, y| x.cost.cmp(&y.cost))
    .unwrap();

    println!("{war:?}");

    // let r_power = get_power(&store.rings);

    // // .iter()
    // // .combinations_with_replacement(2)
    // // .collect();
    // let war: Vec<_> = wa.iter().cartesian_product(r_power).collect();
    // println!("{war:?}")
}
