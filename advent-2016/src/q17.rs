use std::{collections::{HashSet, HashMap}, vec, hash::{Hash, Hasher}};

use crate::Runner;

#[derive(Debug, Clone, Eq)]
struct Vault {
    room: (i32,i32),
    passcode: String
}

impl PartialEq for Vault {
    fn eq(&self, other: &Self) -> bool {
        self.passcode == other.passcode
    }
}

impl Hash for Vault {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.passcode.hash(state);
    }

}

impl Vault {
    fn new(passcode : String) -> Self {
        Vault {
            room : (0,0),
            passcode
        }
    }

    fn get_next_rooms(&self) -> Vec<Vault> {
        let mut rooms = Vec::new();
        let hash = md5::compute(self.passcode.clone());
        let path = format!("{:x}", hash);
        let mut chars = path.chars();
        for direction in [(0,-1, "U"),(0,1,"D"),(-1,0,"L"),(1,0,"R")] {
            let mut new_room = self.clone();
            match chars.next().unwrap() {
                'b' | 'c' | 'd' | 'e' |'f' => {
                    let new_x = self.room.0 + direction.0;
                    let new_y = self.room.1 + direction.1;
                    if new_x >= 0 && new_x < 4 && new_y >= 0 && new_y < 4 {
                        new_room.passcode.push_str(direction.2);
                        new_room.room = (new_x, new_y);
                        rooms.push(new_room);                        
                    } 
                },
                _ => continue
            }
        }
        rooms
    }
}

pub struct Q17 {
    vault : Vault
}

impl Q17 {
    pub fn new() -> Self {
        let passcode = "lpvhkcbi".to_string();
        let vault = Vault::new(passcode);
        Q17 {
            vault,
        }
    }

    fn part1(&self) {
        dijkstra(&self.vault)
    }
}

fn dijkstra(vault: &Vault) {
    let mut stack = vec![vault.clone()];
    let mut dist = HashMap::from([(vault.clone(), 0)]);

    while let Some(current_vault) = stack.pop() {
        if current_vault.room == (3,3){
            continue;
        }

        for new_vault in current_vault.get_next_rooms() {
            let current_distance = if dist.contains_key(&new_vault) {
                *dist.get(&new_vault).unwrap()      
            } else {
                let new_distance =(new_vault.clone(), i32::MAX); 
                dist.insert(new_distance.0, new_distance.1);
                new_distance.1
            };

            let new_distance = *dist.get(&current_vault).unwrap() + 1;

            if new_distance < current_distance {
                dist.insert(new_vault.clone(), new_distance);
                stack.push(new_vault);
            }
        } 

    }
    
    let r = dist.iter().filter(|p| p.0.room == (3,3)).min_by(|x,y| x.1.cmp(y.1)).unwrap();
    let c = dist.iter().filter(|p| p.0.room == (3,3)).count();
    println!("{:?} {}", r, c);
}

impl Runner for Q17 {
    fn run(&mut self) -> () {
        self.part1();
    }
}