use std::collections::HashSet;

use advent_2018::{Runner, get_file};


// unit 
// number of units
// hp
// atk
// atk-type
// initiative
// Option<Weakness, immunity>
// effective power: number of units * atk 

// target selection
// each grp selects one target
//  1. decending order of effective power, if tie, high initiative 
//  2. select enemy grp that will take the most dmg
//     after accouting weakness or immunity
//      - grp with largest effective power, if tie, highest initiative
//  3. if cannot deal any damage, then choose None
//  4. only one atk grp can dmg one def grp

// attacking
// 1. decreasing order of initiative

fn parse_number(num:&str) -> i32 {
    num.parse().unwrap()
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Groups {
    units: Vec<Unit>
}

impl Groups {
    fn new(lines: &str) -> Self {
        let mut units = vec![];
        let mut side = "whatisthis?".to_string();
        lines.lines().enumerate().for_each(|(idx, line)| {
            if line.contains("Immune") {
                side = "im".to_string();
                return
            } else if line.contains("Infection") {
                side = "inf".to_string();
                return
            } else if line.is_empty() {
                return
            } 
            match line.contains("(") {
                true => {
                    let id = idx as i32;
                    let data: Vec<_> = line.split(|p| p == '(' || p == ')').collect();
                    let (first, second, third) = (data[0],data[1],data[2]);
                    let first: Vec<_>= first.split_whitespace().collect();
                    let num_of_units: i32 = parse_number(first[0]);
                    let hp = parse_number(first[4]);
                    let third: Vec<_> = third.split_ascii_whitespace().collect();
                    let atk = parse_number(third[5]);
                    let atk_type = third[6].to_string();
                    let init = parse_number(third[10]);
                    let mut immune = Vec::new();
                    let mut weak = Vec::new();
                    match second.split_once(';') {
                        Some((mut im,mut wk)) => {
                            if im.contains("weak") {
                                let tmp = im;
                                im = wk;
                                wk = tmp;
                            }
                            let (_, im) = im.split_once(" to ").unwrap();
                            immune= im.split(", ").map(String::from).collect(); 
                            let (_, wk) = wk.split_once(" to ").unwrap();
                            weak = wk.split(", ").map(String::from).collect(); 
                            
                        },
                        None => {
                            match second.contains("weak") {
                                true => {
                                    let (_, wk) = second.split_once(" to ").unwrap();
                                    weak = wk.split(", ").map(String::from).collect(); 
                                }
                                false => {
                                    let (_, im) = second.split_once(" to ").unwrap();
                                    immune = im.split(", ").map(String::from).collect(); 
                                }
                            }
                        }
                    }
                    units.push(Unit {atk,hp,id,init,num_of_units,atk_type,immune,weak, side: side.clone()})
                }
                false => {
                    let line: Vec<_>= line.split_whitespace().collect();
                    let id = idx as i32;
                    let num_of_units = parse_number(line[0]);
                    let hp = parse_number(line[4]);
                    let atk = parse_number(line[12]);
                    let atk_type = line[13].to_string();
                    let weak = Vec::new();
                    let immune = Vec::new();
                    let init = parse_number(line[17]);
                    units.push(Unit { id, num_of_units, hp, atk, atk_type, weak, immune, init, side: side.clone() })
                }
            }
            
            // let (im, wk) = second.split_once(';').unwrap();
        });
        Self { units }
    }

    
}

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
struct Unit {
    id: i32,
    num_of_units: i32,
    hp: i32,
    atk: i32,
    atk_type: String,
    weak: Vec<String>,
    immune: Vec<String>,
    init: i32,
    side: String
}

impl Unit {
    fn effective_power(&self) -> i32 {
        self.atk * self.num_of_units
    }

    fn dmg_to(&self, other: &Unit) -> i32{
        if other.weak.contains(&self.atk_type) {
            self.effective_power() * 2
        } else if other.immune.contains(&self.atk_type) {
            0
        } else {
            self.effective_power()
        }
    }
}    

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Simulator {
    grps: Groups,
}

impl Simulator {
    fn new() -> Self {
        let content = get_file("src/input/q24.txt").unwrap();    
        let mut grps = Groups::new(&content);
        Self {grps}
    }

    fn target_selection(&mut self) {

    }

    fn attack_phase(&mut self) {
        
    }

    fn battle(&mut self, boost:i32) -> (String, i32){
        self.grps.units.iter_mut().for_each(|unit| {
            if unit.side == "im" {
                unit.atk += boost
            }
        });
        loop {
            self.grps.units.sort_by(|x,y| y.effective_power().cmp(&x.effective_power()).then(y.init.cmp(&x.init)));
            let mut seen: HashSet<i32>= HashSet::new();
            let mut final_targets = vec![];
            for unit in self.grps.units.iter() {
                // println!("unit - {:?}", unit);
                let mut targets = vec![];
                for target in self.grps.units.iter() {
                    if unit.side != target.side && unit.dmg_to(target) > 0 && !seen.contains(&target.id) {
                        targets.push(target)
                    }
                }
                // println!("unit {unit:?}");
                // println!("targets {targets:?}")
                targets.sort_by(|x,y|unit.dmg_to(y).cmp(&unit.dmg_to(x)).then(y.effective_power().cmp(&x.effective_power())).then(y.init.cmp(&x.init)));
                if !targets.is_empty() {
                    final_targets.push((unit.id, targets[0].id));
                    seen.insert(targets[0].id);
                }
            }
 
            final_targets.sort_by(|x,y| {
                let unit0 = self.grps.units.iter().find(|unit| unit.id == x.0 ).unwrap();
                let unit1 = self.grps.units.iter().find(|unit| unit.id == y.0 ).unwrap();
                unit1.init.cmp(&unit0.init)
            });
            // println!("final_targets - {:?}", final_targets); // im:12  inf:56
            let mut did_kill = false;
            for target in final_targets {
                let (atk_id, def_id) = target;
                let atk = self.grps.units.iter().find(|unit| unit.id == atk_id ).unwrap();
                let def = self.grps.units.iter().find(|unit| unit.id == def_id ).unwrap();
                let dmg = atk.dmg_to(def);
                let kills = def.num_of_units.min(dmg / def.hp);
                if kills > 0 {
                    did_kill = true;
                }
                // println!("{:?}", kills);
                let def1 = self.grps.units.iter_mut().find(|unit| unit.id == def_id ).unwrap();
                def1.num_of_units -= kills;
            }
            if !did_kill {
                return ("inf".to_string(), 0)
            }
            let im_sum:i32= self.grps.units.iter().filter(|unit| unit.side == "im".to_string()).map(|u| u.num_of_units).sum();
            let inf_sum:i32= self.grps.units.iter().filter(|unit| unit.side == "inf".to_string()).map(|u| u.num_of_units).sum();
            self.grps.units = self.grps.units.iter().cloned().filter(|x| x.num_of_units > 0).collect();
            let total = self.grps.units.iter().map(|u| u.num_of_units).sum::<i32>();
            if im_sum == 0 {
                // println!("infection wins!");
                return ("inf".to_string(), total)
            }
            if inf_sum == 0 {
                println!("immune wins!");
                println!("{:?}", self.grps.units);
                return ("im".to_string(), total)
            }
        }
    }
}


pub struct Q24 {

}

impl Q24 {
    pub fn new() -> Self {
        Q24 {}
    }


    fn part1(&mut self) {
        // let mut simulator = Simulator::new();
        // println!("{:?}", simulator);
        // simulator.battle();
        // 24725 22625
        // println!("{:?}", 801  - (801  * 4706  - 22625) / 4706 )
    }

    fn part2(&mut self) {
        let mut boost = 0;
        loop {
            let mut simulator = Simulator::new();
            let (wins, total) = simulator.battle(boost);
            // println!("{:?}", boost);
            if wins == "im" {
                println!("{:?}", boost);
                println!("{:?}", total);
                break
            }
            boost+= 1
        }
    }

}

impl Runner for Q24 {
    fn run(&mut self) {
        // self.part1();
        self.part2();
    }
}


#[cfg(test)]
mod test{
    #[test]
    fn Q24() {
        assert_eq!(1, 1);
    }
}

// while !self.immune.units.is_empty() && !self.infection.units.is_empty() {
//     // println!("{:?}", self.immune.units);
//     // println!("{:?}", self.infection.units);
//     self.immune.units.sort_by(|x,y| y.effective_power().cmp(&x.effective_power()).then(y.init.cmp(&x.init)));
//     self.infection.units.sort_by(|x,y| y.effective_power().cmp(&x.effective_power()).then(y.init.cmp(&x.init)));

//     let mut attacks = vec![];
//     self.immune.units.iter().for_each(|im| {
//         let mut dmgs_to_infection = vec![];
//         self.infection.units.iter().for_each(|inf| {
//             if im.dmg_to(inf) > 0 {
//                 dmgs_to_infection.push((inf.id, im.dmg_to(inf), inf.init, "inf".to_string()))
//             }
//         });
//         dmgs_to_infection.sort_by(|x,y|y.1.cmp(&x.1).then(x.2.cmp(&y.2)));
//         attacks.push((im.id, dmgs_to_infection, im.init));
//     });

//     self.infection.units.iter().for_each(|inf| {
//         let mut dmg_to_imnune = vec![];
//         self.immune.units.iter().for_each(|im| {
//             dmg_to_imnune.push((im.id, inf.dmg_to(im), im.init, "im".to_string()))
//         });
//         dmg_to_imnune.sort_by(|x,y|y.1.cmp(&x.1).then(x.2.cmp(&y.2)));
//         attacks.push((inf.id, dmg_to_imnune, inf.init));
//     });            
//     attacks.sort_by(|x,y|y.2.cmp(&x.2));
//     let mut seen = HashSet::new();
//     // println!("{:?}", attacks);
//     attacks.iter().for_each(|attack| {
//         let (atk_grp, def_grps, atk_init) = attack;
//         let mut did_attack = false;
//         for def_grp in def_grps.iter(){
//             let (id, dmg, init, side) = def_grp;
//             if did_attack {
//                 break
//             }
//             if !seen.insert((id,side)) {
//                 continue
//             }
//             did_attack = true;
//             // println!("attacked {side} with dmg: {dmg}");
//             match side.as_str() {
//                 "im" => {
//                     let atk = self.infection.units.iter_mut().find(|inf| inf.id == *atk_grp ).unwrap();
//                     let def = self.immune.units.iter_mut().find(|im| im.id == *id).unwrap();
//                     let hp_left = def.hp * def.num_of_units - atk.dmg_to(def);
//                     let mut units_left = hp_left / def.hp;
//                     if hp_left % def.hp >0 {
//                         units_left += 1;
//                     }
//                     println!("{:?}", def.num_of_units - units_left);
//                     def.num_of_units = if units_left <= 0{0} else {units_left};
//                 },
//                 "inf" => {
//                     let atk = self.immune.units.iter_mut().find(|im| im.id == *atk_grp).unwrap();
//                     let def = self.infection.units.iter_mut().find(|inf| inf.id == *id ).unwrap();
//                     let hp_left = def.hp * def.num_of_units - atk.dmg_to(def);
//                     let mut units_left = hp_left / def.hp;
//                     if hp_left % def.hp > 0 {
//                         units_left += 1;
//                     }
//                     println!("{:?}", def.num_of_units - units_left);

//                     def.num_of_units = units_left;

//                 },
//                 _ => panic!("wrong side")
//             }
//         }
//     });
//     self.immune.units =self.immune.units.iter().cloned().filter(|x|x.num_of_units > 0).collect();
//     self.infection.units =self.infection.units.iter().cloned().filter(|x|x.num_of_units > 0).collect();
//     println!("");
// }
// println!("{:?}", self.immune);
// println!("{:?}", self.infection);