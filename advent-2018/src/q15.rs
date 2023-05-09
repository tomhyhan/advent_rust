use std::collections::{HashMap, HashSet, VecDeque};

use advent_2018::{get_file, Runner};
use regex::internal::Char;

pub struct Q15 {}

pub struct Game {
    map: HashMap<(i32, i32), char>,
    elfs: Vec<Elf>,
    goblins: Vec<Goblin>,
    min_row: i32,
    max_row: i32,
    min_col: i32,
    max_col: i32,
}

impl Game {
    fn new(elf_attack: i32) -> Self {
        let contents = get_file("src/input/q15.txt").unwrap();
        let mut map = HashMap::new();
        let mut elfs = Vec::new();
        let mut goblins = Vec::new();
        let mut min_row = i32::MAX;
        let mut max_row = i32::MIN;
        let mut min_col = i32::MAX;
        let mut max_col = i32::MIN;

        contents.lines().enumerate().for_each(|(row, line)| {
            line.chars().enumerate().for_each(|(col, c)| {
                let row = row as i32;
                let col = col as i32;
                match c {
                    '#' => {
                        map.insert((row, col), '#');
                    }
                    '.' => {
                        map.insert((row, col), '.');
                    }
                    'G' => {
                        goblins.push(Goblin::new(row, col));
                        map.insert((row, col), '.');
                    }
                    'E' => {
                        elfs.push(Elf::new(row, col, elf_attack));
                        map.insert((row, col), '.');
                    }
                    _ => panic!("not a member of a map"),
                };
                min_row = min_row.min(row);
                max_row = max_row.max(row);
                min_col = min_col.min(col);
                max_col = max_col.max(col);
            })
        });
        Self {
            map,
            elfs,
            goblins,
            max_col,
            max_row,
            min_col,
            min_row,
        }
    }

    fn combat(&mut self) -> bool {
        let mut rounds = 0;
        let num_of_elfs = self.elfs.len();
        loop {
            rounds += 1;
            let mut current_map = self.map.clone();
            self.add_characters(&mut current_map);
            let mut seen = HashSet::new();

            for row in self.min_row..=self.max_row {
                for col in self.min_col..=self.max_col {
                    if seen.contains(&(row, col)) {
                        continue;
                    }
                    let current = current_map.get(&(row, col)).unwrap();
                    match current {
                        'E' => {
                            if self.can_move(row, col, true, &current_map) {
                                let (new_row, new_col) = self.move_character(
                                    row,
                                    col,
                                    true,
                                    &mut current_map,
                                    &mut seen,
                                );
                                self.try_attack(new_row, new_col, true, &mut current_map);
                            } else {
                                self.try_attack(row, col, true, &mut current_map);
                            }
                        }
                        'G' => {
                            if self.can_move(row, col, false, &current_map) {
                                let (new_row, new_col) = self.move_character(
                                    row,
                                    col,
                                    false,
                                    &mut current_map,
                                    &mut seen,
                                );
                                self.try_attack(new_row, new_col, false, &mut current_map);
                            } else {
                                self.try_attack(row, col, false, &mut current_map);
                            }
                        }
                        '.' | '#' => continue,
                        _ => panic!("unknonw characters!"),
                    }
                }
            }

            if self.elfs.len() != num_of_elfs {
                return false;
            }
            if self.elfs.is_empty() || self.goblins.is_empty() {
                break;
            }
        }

        let gob_hp: i32 = self.goblins.iter().map(|g| g.hp).sum();
        let elf_hp: i32 = self.elfs.iter().map(|g| g.hp).sum();

        println!("result");
        println!("{:?}", self.elfs);
        println!("{:?}", self.goblins);
        println!("gob {:?}", gob_hp);
        println!("elf {:?}", elf_hp);
        println!("{:?}", rounds);
        println!("{:?}", gob_hp * (rounds - 1));
        println!("{:?}", gob_hp * (rounds - 2));
        println!("{:?}", elf_hp * (rounds - 1));

        true
    }

    // 17 23 -> 17 24 || 16 23 14 24 15 24
    // 17 23 -> 17 22
    fn try_attack(
        &mut self,
        row: i32,
        col: i32,
        is_elf: bool,
        map: &mut HashMap<(i32, i32), char>,
    ) {
        let end = if is_elf { 'G' } else { 'E' };
        let mut enemies = Vec::new();
        for dir in [(-1, 0), (0, -1), (0, 1), (1, 0)] {
            let new_row = row + dir.0;
            let new_col = col + dir.1;
            let current = map.get(&(new_row, new_col)).unwrap();
            if *current == end {
                enemies.push((new_row, new_col))
            }
        }

        if enemies.is_empty() {
            return;
        }

        if is_elf {
            let lowesthp = self
                .goblins
                .iter_mut()
                .filter(|g| enemies.contains(&(g.row, g.col)))
                .min_by_key(|g| g.hp)
                .unwrap()
                .hp;
            let mut gobs: Vec<_> = self
                .goblins
                .iter_mut()
                .filter(|g| enemies.contains(&(g.row, g.col)) && g.hp == lowesthp)
                .collect();
            gobs.sort_by(|p1, p2| (p1.row, p1.col).cmp(&(p2.row, p2.col)));
            let gob = &mut gobs[0];
            let elf_atk = self.elfs.iter().nth(0).unwrap().attack;
            gob.hp -= elf_atk;
            if gob.hp <= 0 {
                map.insert((gob.row, gob.col), '.');
            }
        } else {
            let lowesthp = self
                .elfs
                .iter_mut()
                .filter(|e| enemies.contains(&(e.row, e.col)))
                .min_by_key(|e| e.hp)
                .unwrap()
                .hp;
            let mut elfs: Vec<_> = self
                .elfs
                .iter_mut()
                .filter(|e| enemies.contains(&(e.row, e.col)) && e.hp == lowesthp)
                .collect();
            elfs.sort_by(|p1, p2| (p1.row, p1.col).cmp(&(p2.row, p2.col)));
            let elf = &mut elfs[0];
            elf.hp -= 3;
            if elf.hp <= 0 {
                map.insert((elf.row, elf.col), '.');
            }
        }

        self.elfs = self.elfs.iter().cloned().filter(|e| e.hp > 0).collect();
        self.goblins = self.goblins.iter().cloned().filter(|g| g.hp > 0).collect();
    }

    fn move_character(
        &mut self,
        row: i32,
        col: i32,
        is_elf: bool,
        map: &mut HashMap<(i32, i32), char>,
        seen: &mut HashSet<(i32, i32)>,
    ) -> (i32, i32) {
        let open_spots = self.find_open_spots(row, col, map);
        // should be ([start pos], [current_location], [])
        // if row == 17 && col == 23 {
        //     println!("openspots {:?}", open_spots);
        // }
        let mut queue = VecDeque::from(open_spots);
        let mut min_distance = i32::MAX;
        let end = if is_elf { 'G' } else { 'E' };
        let mut result = Vec::new();
        let mut visited = HashSet::new();
        while let Some((start_pos, (c_row, c_col), distance)) = queue.pop_front() {
            if !visited.insert((c_row, c_col, start_pos.0, start_pos.1)) {
                continue;
            }

            for dir in [(-1, 0), (0, -1), (0, 1), (1, 0)] {
                let new_row = c_row + dir.0;
                let new_col = c_col + dir.1;
                let current = map.get(&(new_row, new_col)).unwrap();

                if *current == end {
                    if distance <= min_distance {
                        result.push((start_pos, (c_row, c_col), distance));
                        min_distance = distance;
                    }
                } else if *current == '.' {
                    queue.push_back((start_pos, (new_row, new_col), distance + 1))
                }
            }
        }

        result = result
            .into_iter()
            .filter(|pos| pos.2 == min_distance)
            .collect();
        result.sort_by(|p1, p2| {
            (p1.1 .0, p1.1 .1)
                .cmp(&(p2.1 .0, p2.1 .1))
                .then((p1.0 .0, p1.0 .1).cmp(&(p2.0 .0, p2.0 .1)))
        });

        // if row == 17 && col == 23 {
        //     println!("result {:?}", result);
        //     println!("min_distance - {min_distance}");
        //     for i in 15..20 {
        //         let mut s = String::new();
        //         for j in 18..30 {
        //             let c = map.get(&(i, j)).unwrap();
        //             s.push(c.clone());
        //         }
        //         println!("{:?}", s)
        //     }
        // }
        // println!("{:?}", map);
        if result.is_empty() {
            return (row, col);
        }

        let move_to = result[0].0;
        // println!(" move_to - {:?}", move_to);
        // println!("");

        if is_elf {
            let mut elf = self
                .elfs
                .iter_mut()
                .find(|elf| elf.row == row && elf.col == col)
                .unwrap();
            map.insert((elf.row, elf.col), '.');
            elf.row = move_to.0;
            elf.col = move_to.1;
            map.insert((elf.row, elf.col), 'E');
            seen.insert((elf.row, elf.col));
            (elf.row, elf.col)
        } else {
            let mut goblin = self
                .goblins
                .iter_mut()
                .find(|goblin| goblin.row == row && goblin.col == col)
                .unwrap();
            //  when moving remove already moved ones
            map.insert((goblin.row, goblin.col), '.');
            goblin.row = move_to.0;
            goblin.col = move_to.1;
            map.insert((goblin.row, goblin.col), 'G');
            seen.insert((goblin.row, goblin.col));
            (goblin.row, goblin.col)
        }
    }

    fn find_open_spots(
        &self,
        row: i32,
        col: i32,
        map: &HashMap<(i32, i32), char>,
    ) -> Vec<((i32, i32), (i32, i32), i32)> {
        let mut spots = Vec::new();
        for dir in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
            let new_row = row + dir.0;
            let new_col = col + dir.1;
            let current = map.get(&(new_row, new_col)).unwrap();
            if *current == '.' {
                spots.push(((new_row, new_col), (new_row, new_col), 0));
            }
        }
        spots
    }

    fn can_move(&self, row: i32, col: i32, is_elf: bool, map: &HashMap<(i32, i32), char>) -> bool {
        let mut open = false;
        for dir in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
            let new_row = row + dir.0;
            let new_col = col + dir.1;
            let current = map.get(&(new_row, new_col)).unwrap();
            // println!("{current}");
            if is_elf && *current == 'G' {
                return false;
            } else if !is_elf && *current == 'E' {
                return false;
            }
            if *current == '.' {
                open = true
            }
        }
        open
    }

    fn add_characters(&self, current_map: &mut HashMap<(i32, i32), char>) {
        self.elfs.iter().for_each(|elf| {
            current_map.insert((elf.row, elf.col), 'E');
        });
        self.goblins.iter().for_each(|goblin| {
            current_map.insert((goblin.row, goblin.col), 'G');
        });
    }
}

#[derive(Debug, Clone, Copy)]
struct Elf {
    row: i32,
    col: i32,
    hp: i32,
    attack: i32,
}

#[derive(Debug, Clone, Copy)]
struct Goblin {
    row: i32,
    col: i32,
    hp: i32,
    attack: i32,
}

impl Elf {
    fn new(row: i32, col: i32, elf_attack: i32) -> Self {
        Self {
            row,
            col,
            hp: 200,
            attack: elf_attack,
        }
    }
}

impl Goblin {
    fn new(row: i32, col: i32) -> Self {
        Self {
            row,
            col,
            hp: 200,
            attack: 3,
        }
    }
}

impl Q15 {
    pub fn new() -> Self {
        Q15 {}
    }

    fn part1(&mut self) {
        let mut elf_attack = 3;
        loop {
            elf_attack += 1;
            let mut game = Game::new(elf_attack);
            let elfs_alive = game.combat();
            println!("elf_attack - {:?}", elf_attack);
            println!("gobs - {:?}", game.goblins);
            // println!("gobs - {:?}", game.elfs);
            if !elfs_alive {
                continue;
            }

            break;
        }
    }

    fn part2(&mut self) {}
}

impl Runner for Q15 {
    fn run(&mut self) {
        self.part1();
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn Q15() {
        assert_eq!(1, 1);
    }
}
